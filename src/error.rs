use crate::{Status, UserAgent};

#[derive(thiserror::Error, Debug)]
pub enum ActorError {
    #[error("Url parse error: {0}")]
    Url(#[from] url::ParseError),

    #[error("Failed reading/writing to stream: {0}")]
    Stream(#[from] std::io::Error),

    #[error("Header isn't utf8: {0}")]
    Utf8Header(std::str::Utf8Error),

    #[error("OpenSSL error: {0}")]
    SslErr(#[from] openssl::error::ErrorStack),

    #[error("Key and/or cert files are either missing or malformed")]
    KeyCertFileError(openssl::error::ErrorStack),

    #[error("Failed to establish tcp connection: {0}")]
    TcpError(std::io::Error),

    #[error("The gemini header received was malformed")]
    MalformedHeader,

    #[error("Couldn't parse status code: {0}")]
    MalformedStatus(std::num::ParseIntError),

    #[error("The domain in the url is malformed")]
    DomainErr,

    #[error("Visiting {0} isn't allowed from your user-agent ({1}).")]
    RobotDenied(String, UserAgent),

    #[error("Tcp connection timeout: {0}")]
    Timeout(tokio::time::error::Elapsed),

    #[error("Server has no certificate")]
    NoCertificate,

    #[error("Server certificate is malformed, because its subject name isn't utf8")]
    SubjectNameNotUtf8(openssl::error::ErrorStack),

    #[error("Server certificate is malformed, because it indicates no domains")]
    NoDomains,

    #[error("Certificate is valid for {0}, not {1}")]
    DomainUncerified(String, String),
}

#[derive(thiserror::Error, Debug)]
pub enum ResponseErr {
    #[error("Content isn't utf8: {0}")]
    Utf8Content(std::str::Utf8Error),

    #[error("Expected status {0}, received {1}")]
    UnexpectedStatus(Status, Status, String),

    #[error("Expected filetype {0}, receieved {1}")]
    UnexpectedFiletype(String, String),

    #[error("Failed to write file: {0}")]
    FileWrite(std::io::Error),

    #[error("Failed to create file: {0}")]
    FileCreate(std::io::Error),

    #[error("Failed to serialize server's certificate to pem.")]
    SerializingToPem(openssl::error::ErrorStack),

    #[error("Server's certificate pem is invalid utf-8: {0}")]
    PemInvalidUtf8(std::str::Utf8Error),
}
