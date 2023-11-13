/// Enum representing user-agents defined by the gemini version of the robots.txt spec.
#[derive(Debug, Clone)]
pub enum UserAgent {
    /// Gemini bots which fetch content in order to build
    /// public long-term archives of Geminispace, which will
    /// serve old Gemini content even after the original has
    /// changed or disappeared (analogous to archive.org's
    /// "Wayback Machine"), should respect robots.txt
    /// directives aimed at a User-agent of "archiver".
    Archiver,

    /// Gemini bots which fetch content in order to build
    /// searchable indexes of Geminispace should respect
    /// robots.txt directives aimed at a User-agent of
    /// "indexer".
    Indexer,

    /// Gemini bots which fetch content in order to study
    /// large-scale statistical properties of Geminispace
    /// (e.g. number of domains/pages, distribution of MIME
    /// media types, response sizes, TLS versions, frequency
    /// of broken links, etc.), without rehosting, linking
    /// to, or allowing search of any fetched content,
    /// should respect robots.txt directives aimed at a
    /// User-agent of "researcher".
    Researcher,

    /// Gemini bots which fetch content in order to
    /// translate said content into HTML and publicly serve
    /// the result over HTTP(S) (in order to make
    /// Geminispace accessible from within a standard web
    /// browser) should respect robots.txt directives aimed
    /// at a User-agent of "webproxy".
    Webproxy,
}

impl std::fmt::Display for UserAgent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                UserAgent::Archiver => "archiver",
                UserAgent::Indexer => "indexer",
                UserAgent::Researcher => "researcher",
                UserAgent::Webproxy => "webproxy",
            }
        )
    }
}
