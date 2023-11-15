use std::{path::PathBuf, time::Duration};

use openssl::ssl::{Ssl, SslConnector, SslFiletype, SslMethod, SslVerifyMode};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tokio_openssl::SslStream;
use url::Url;
use wildmatch::WildMatch;

use crate::{error::ActorError, Response, UserAgent};

/// 🎠 An ergonomic way to call [`Actor::get`] with the default actor.
///
/// ```
/// Actor::trot("localhost").await
/// ```
pub async fn trot(url: impl Into<String>) -> Result<Response> {
    let url = url.into();
    Actor::default().get(url).await
}

/// 🎠 An ergonomic way to call [`Actor::input`] with the default actor.
///
/// ```
/// Actor::trot_in("localhost/input", "notice me!").await
/// ```
pub async fn trot_in(url: impl Into<String>, input: impl Into<String>) -> Result<Response> {
    Actor::default().input(url.into(), input.into()).await
}

/// Make a gemini request.
pub struct Actor {
    pub cert:       Option<PathBuf>,
    pub key:        Option<PathBuf>,
    pub user_agent: Option<UserAgent>,
    /// Timeout for establishing tcp connections (default is 5 secs)
    pub timeout:    Duration,
}

type Result<T> = std::result::Result<T, ActorError>;

impl Default for Actor {
    fn default() -> Self {
        Self {
            user_agent: None,
            cert:       None,
            key:        None,
            timeout:    Duration::from_secs(5),
        }
    }
}

impl Actor {
    /// Set your client certificate file path
    pub fn cert_file(mut self, cert: impl Into<PathBuf>) -> Self {
        self.cert = Some(cert.into());
        self
    }

    /// Set your client key file path
    pub fn key_file(mut self, key: impl Into<PathBuf>) -> Self {
        self.key = Some(key.into());
        self
    }

    /// *Please* include a user-agent if you're making any
    /// kind of service that indescriminately uses other
    /// peoples' content on gemini.
    ///
    /// This allows people to block types of services they
    /// don't want to access their content.
    ///
    /// More info: [robots.txt for Gemini](https://geminiprotocol.net/docs/companion/robots.gmi)
    pub fn user_agent(mut self, useragent: UserAgent) -> Self {
        self.user_agent = Some(useragent);
        self
    }

    /// Send gemini request to url.
    ///
    /// Url can elide the `gemini://` prefix. It's up to you.
    pub async fn get(&self, url: impl Into<String>) -> Result<Response> {
        let url = self.build_url(url.into(), None)?;

        self.obey_robots(&url).await?;
        Ok(self.send_request(&url).await?)
    }

    /// Send gemini request to url with input.
    ///
    /// Input is automatically percent-encoded.
    pub async fn input(
        &self,
        url: impl Into<String>,
        input: impl Into<String>,
    ) -> Result<Response> {
        let input = input.into();
        let input = urlencoding::encode(&input);
        let url = self.build_url(url.into(), Some(&input))?;

        self.obey_robots(&url).await?;
        Ok(self.send_request(&url).await?)
    }

    /// (private) Internal function for sending a request.
    async fn send_request(&self, url: &Url) -> Result<Response> {
        // Build connector
        let mut connector = SslConnector::builder(SslMethod::tls_client())?;
        connector.set_verify_callback(SslVerifyMode::FAIL_IF_NO_PEER_CERT, |_, _| true);

        // Add client certificate
        if let Some(key) = &self.key {
            connector
                .set_private_key_file(key, SslFiletype::PEM)
                .map_err(|e| ActorError::KeyCertFileError(e))?;
        }
        if let Some(cert) = &self.cert {
            connector
                .set_certificate_file(cert, SslFiletype::PEM)
                .map_err(|e| ActorError::KeyCertFileError(e))?;
        }

        // Connect with tcp
        let domain = url.domain().ok_or(ActorError::DomainErr)?;
        let port = url.port().unwrap_or(1965);

        let tcp = tokio::time::timeout(
            self.timeout,
            TcpStream::connect(&format!("{domain}:{port}")),
        )
        .await
        .map_err(|t| ActorError::Timeout(t))?
        .map_err(|e| ActorError::TcpError(e))?;

        // Wrap connection in ssl stream
        let mut ssl = Ssl::new(connector.build().context())?;
        ssl.set_connect_state();
        ssl.set_hostname(domain)?; // <- SNI (Server name indication) and don't you forget it 💢

        let mut stream = SslStream::new(ssl, tcp)?;

        // Write request
        stream
            .write_all(&format!("{url}\r\n",).into_bytes())
            .await?;

        // Get response header
        let mut header: Vec<u8> = Vec::new();
        let mut p = b' ';
        for _ in 0..=1026 {
            let c = stream.read_u8().await?;

            // Break if \r\n
            if p == b'\r' && c == b'\n' {
                let _ = header.pop();
                break;
            }

            header.push(c);
            p = c;
        }

        let header = std::str::from_utf8(&header).map_err(|e| ActorError::Utf8Header(e))?;

        // Strip status and meta from the header
        let (status, meta) = header.split_once(' ').ok_or(ActorError::MalformedHeader)?;
        let status = status
            .parse::<u8>()
            .map_err(|e| ActorError::MalformedStatus(e))?;
        let meta = meta.to_string();

        // Get remaining response content
        let mut content: Vec<u8> = Vec::new();
        stream.read_to_end(&mut content).await?;

        // Get certificate pem
        let certificate = stream
            .ssl()
            .peer_certificate()
            .ok_or(ActorError::NoCertificate)?;

        // Get list of valid domains
        let valid_domains: Vec<String> = certificate
            .subject_alt_names()
            .ok_or(ActorError::NoSubjectNames)?
            .into_iter()
            .filter_map(|x| {
                if let Some(name) = x.dnsname() {
                    Some(name.to_string())
                } else {
                    None
                }
            })
            .collect();

        // Error if none of them match
        if valid_domains
            .iter()
            .filter(|x| WildMatch::new(x).matches(&domain))
            .count()
            == 0
        {
            return Err(ActorError::DomainUncerified(
                format!("{valid_domains:?}"),
                domain.to_string(),
            ))?;
        }

        Ok(Response {
            content,
            status,
            meta,
            certificate,
        })
    }

    /// (private) Internal function for obeying robots.txt
    async fn obey_robots(&self, url: &Url) -> Result<()> {
        let Some(user_agent) = &self.user_agent else {
            return Ok(());
        };

        if let Ok(response) = self
            .send_request(&Url::parse(&format!(
                "gemini://{}/robots.txt",
                url.domain().ok_or(ActorError::DomainErr)?
            ))?)
            .await
        {
            if let Ok(txt) = response.text() {
                let mut robots_map = crate::utils::parse_robots(&txt);

                // Track the disallows that affect us
                let mut disallow_list: Vec<&str> = Vec::new();

                // Add our useragent's entries to disallow list
                if let Some(for_me) = robots_map.get_mut(user_agent.to_string().as_str()) {
                    disallow_list.append(for_me);
                }

                // Add * entries to disallow list
                if let Some(for_everyone) = robots_map.get_mut("*") {
                    disallow_list.append(for_everyone);
                }

                for path in disallow_list {
                    if path == "/" || url.path().starts_with(&path) {
                        return Err(ActorError::RobotDenied(
                            path.to_string(),
                            user_agent.clone(),
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    fn build_url(&self, mut url: String, input: Option<&str>) -> Result<Url> {
        //  Add `gemini://` if it's not in the url
        if let Some(pos) = url.find("gemini://") {
            if pos != 0 {
                url = format!("gemini://{url}");
            }
        } else {
            url = format!("gemini://{url}");
        }

        let mut url = Url::parse(&url)?;

        // Add slash to path
        if url.path() == "" {
            url.set_path("/");
        }

        url.set_query(input);

        Ok(url)
    }
}
