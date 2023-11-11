use openssl::ssl::{Ssl, SslConnector, SslFiletype, SslMethod, SslVerifyMode};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tokio_openssl::SslStream;
use url::Url;

use crate::{error::ActorError, Response, UserAgent};

/// Make a gemini request.
#[derive(Default)]
pub struct Actor {
    pub cert:       Option<String>,
    pub key:        Option<String>,
    pub user_agent: Option<UserAgent>,
}

type Result<T> = std::result::Result<T, ActorError>;

/// 🎠 An ergonomic way to call [`Actor::get`] with the default actor.
///
/// ```
/// Actor::trot("localhost").await
/// ```
pub async fn trot(url: impl Into<String>) -> Result<Response> {
    let url = url.into();
    Actor::default().get(url).await
}

impl Actor {
    /// Set your client certificate file path
    pub fn cert_file(mut self, cert: &str) -> Self {
        self.cert = Some(cert.to_string());
        self
    }

    /// Set your client key file path
    pub fn key_file(mut self, key: &str) -> Self {
        self.key = Some(key.to_string());
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
        let mut url = url.into();

        //  Add `gemini://` if it's not in the url
        if let Some(pos) = url.find("gemini://") {
            if pos != 0 {
                url = format!("gemini://{url}");
            }
        } else {
            url = format!("gemini://{url}");
        }

        let url = Url::parse(&url)?;
        self.obey_robots(&url).await?;
        Ok(self.send_request(url).await?)
    }

    /// (private) Internal function for sending a request.
    async fn send_request(&self, mut url: Url) -> Result<Response> {
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

        // Create connection
        let domain = url.domain().ok_or(ActorError::DomainErr)?;
        let port = url.port().unwrap_or(1965);
        let tcp = TcpStream::connect(&format!("{domain}:{port}"))
            .await
            .map_err(|e| ActorError::TcpError(e))?;

        let mut ssl = Ssl::new(connector.build().context())?;
        ssl.set_connect_state();
        ssl.set_hostname(domain)?; // <- SNI (Server name indication) and don't you forget it 💢
        let mut stream = SslStream::new(ssl, tcp)?;

        // Add slash to path
        if url.path() == "" {
            url.set_path("/");
        }

        // Write request
        stream
            .write_all(&format!("{url}\r\n",).into_bytes())
            .await?;

        // Get response header
        let mut header: Vec<u8> = Vec::new();
        let mut p = b' ';
        loop {
            let c = stream.read_u8().await?;

            // Break if \r\n
            if p == b'\r' && c == b'\n' {
                let _ = header.pop();
                break;
            }

            header.push(c);
            p = c;
        }

        let header = std::str::from_utf8(&header)?;

        // Strip status and meta from the header
        let (status, meta) = header.split_once(' ').ok_or(ActorError::MalformedHeader)?;
        let status = status
            .parse::<u8>()
            .map_err(|e| ActorError::MalformedStatus(e))?;
        let meta = meta.to_string();

        // Get remaining response content
        let mut content: Vec<u8> = Vec::new();
        stream.read_to_end(&mut content).await?;

        Ok(Response {
            content,
            status,
            meta,
        })
    }

    /// (private) Internal function for obeying robots.txt
    async fn obey_robots(&self, url: &Url) -> Result<()> {
        let Some(user_agent) = &self.user_agent else {
            return Ok(());
        };

        if let Ok(response) = self
            .send_request(Url::parse(&format!(
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
}
