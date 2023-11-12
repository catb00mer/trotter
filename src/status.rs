/// Enum for representing gemini status codes.
#[derive(Debug)]
pub enum Status {
    /// 10 As per definition of single-digit code 1 in 3.2.
    Input,

    /// 11 As per status code 10, but for use with sensitive input such as passwords. Clients should present the prompt as per status code 10, but the user's input should not be echoed to the screen to prevent it being read by "shoulder surfers".
    SensitiveInput,

    /// 20 As per definition of single-digit code 2 in 3.2.
    Success,

    /// 30 As per definition of single-digit code 3 in 3.2.
    RedirectTemporary,

    /// 31 The requested resource should be consistently requested from the new URL provided in future. Tools like search engine indexers or content aggregators should update their configurations to avoid requesting the old URL, and end-user clients may automatically update bookmarks, etc. Note that clients which only pay attention to the initial digit of status codes will treat this as a temporary redirect. They will still end up at the right place, they just won't be able to make use of the knowledge that this redirect is permanent, so they'll pay a small performance penalty by having to follow the redirect each time.
    RedirectPermanent,

    /// 40 As per definition of single-digit code 4 in 3.2.
    TemporaryFailure,

    /// 41 The server is unavailable due to overload or maintenance. (cf HTTP 503)
    ServerUnavailable,

    /// 42 A CGI process, or similar system for generating dynamic content, died unexpectedly or timed out.
    CgiError,

    /// 43 A proxy request failed because the server was unable to successfully complete a transaction with the remote host. (cf HTTP 502, 504)
    ProxyError,

    /// 44 Rate limiting is in effect. <META> is an integer number of seconds which the client must wait
    /// before another request is made to this server. (cf HTTP 429)
    SlowDown,

    /// 50 As per definition of single-digit code 5 in 3.2.
    PermanentFailure,

    /// 51 The requested resource could not be found but may be available in the future. (cf HTTP 404) (struggling to remember this important status code? Easy: you can't find things hidden at Area 51!)
    NotFound,

    /// 52 The resource requested is no longer available and will not be available again. Search engines and similar tools should remove this resource from their indices. Content aggregators should stop requesting the resource and convey to their human users that the subscribed resource is gone. (cf HTTP 410)
    Gone,

    /// 53 The request was for a resource at a domain not served by the server and the server does not accept proxy requests.
    ProxyRequestRefused,

    /// 59 The server was unable to parse the client's request, presumably due to a malformed request. (cf HTTP 400)
    BadRequest,

    /// 60 As per definition of single-digit code 6 in 3.2.
    ClientCertificateRequired,

    /// 61 The supplied client certificate is not authorised for accessing the particular requested resource. The problem is not with the certificate itself, which may be authorised for other resources.
    CertificateNotAuthorised,

    /// 62 The supplied client certificate was not accepted because it is not valid. This indicates a problem with the certificate in and of itself, with no consideration of the particular requested resource. The most likely cause is that the certificate's validity start date is in the future or its expiry date has passed, but this code may also indicate an invalid signature, or a violation of X509 standard requirements. The <META> should provide more information about the exact error.   
    CertificateNotValid,

    /// _ Represents any other unsupported status code
    BadStatus,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {self:?}", self.value())
    }
}

impl From<u8> for Status {
    fn from(n: u8) -> Self {
        match n {
            10 => Status::Input,
            11 => Status::SensitiveInput,
            20 => Status::Success,
            30 => Status::RedirectTemporary,
            31 => Status::RedirectPermanent,
            40 => Status::TemporaryFailure,
            41 => Status::ServerUnavailable,
            42 => Status::CgiError,
            43 => Status::ProxyError,
            44 => Status::SlowDown,
            50 => Status::PermanentFailure,
            51 => Status::NotFound,
            52 => Status::Gone,
            53 => Status::ProxyRequestRefused,
            59 => Status::BadRequest,
            60 => Status::ClientCertificateRequired,
            61 => Status::CertificateNotAuthorised,
            62 => Status::CertificateNotValid,
            _ => Status::BadStatus,
        }
    }
}

impl Status {
    /// Return the status number this enum entry represents.
    ///
    /// ## Panics
    ///
    /// If you use `Status::BadStatus`
    pub fn value(&self) -> u8 {
        match self {
            Status::Input => 10,
            Status::SensitiveInput => 11,
            Status::Success => 20,
            Status::RedirectTemporary => 30,
            Status::RedirectPermanent => 31,
            Status::TemporaryFailure => 40,
            Status::ServerUnavailable => 41,
            Status::CgiError => 42,
            Status::ProxyError => 43,
            Status::SlowDown => 44,
            Status::PermanentFailure => 50,
            Status::NotFound => 51,
            Status::Gone => 52,
            Status::ProxyRequestRefused => 53,
            Status::BadRequest => 59,
            Status::ClientCertificateRequired => 60,
            Status::CertificateNotAuthorised => 61,
            Status::CertificateNotValid => 62,
            Status::BadStatus => panic!("Hello. You shouldn't be using `Status::BadStatus`. It's meant to be an error entry for numbers that aren't a valid status code."),
        }
    }
}
