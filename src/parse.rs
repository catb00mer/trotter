#[derive(Debug)]
pub enum Symbol {
    Text(String),
    /// (url, text)
    Link(String, String),
    List(String),
    Quote(String),
    Header1(String),
    Header2(String),
    Header3(String),
    /// (alt-text, content)
    Codeblock(String, String),
}

#[derive(Debug)]
pub struct Gemtext(pub Vec<Symbol>);

impl Gemtext {
    pub fn inner(self) -> Vec<Symbol> {
        self.0
    }

    /// Parse a gemtext string into a vector of [`Symbol`].
    pub fn parse(s: &str) -> Self {
        let mut v: Vec<Symbol> = Vec::new();

        let mut lines = s.lines();
        loop {
            let Some(x) = lines.next() else {
                break;
            };

            if let Some(x) = x.strip_prefix("=>") {
                if let Some((link, name)) = x.trim().split_once(' ') {
                    v.push(Symbol::Link(link.to_string(), name.trim().to_string()))
                } else {
                    v.push(Symbol::Link(x.trim().to_string(), String::new()))
                }
                continue;
            }

            if let Some(x) = x.strip_prefix("*") {
                v.push(Symbol::List(x.trim().to_string()));
                continue;
            }
            if let Some(x) = x.strip_prefix(">") {
                v.push(Symbol::Quote(x.trim().to_string()));
                continue;
            }
            if let Some(x) = x.strip_prefix("###") {
                v.push(Symbol::Header3(x.trim().to_string()));
                continue;
            }
            if let Some(x) = x.strip_prefix("##") {
                v.push(Symbol::Header2(x.trim().to_string()));
                continue;
            }
            if let Some(x) = x.strip_prefix("#") {
                v.push(Symbol::Header1(x.trim().to_string()));
                continue;
            }

            if let Some(x) = x.strip_prefix("```") {
                // Get alt text
                let alt_text = x.trim().to_string();

                // Get block
                let mut block: Vec<&str> = Vec::new();
                loop {
                    let Some(x) = lines.next() else {
                        break;
                    };
                    if x.starts_with("```") {
                        break;
                    }
                    block.push(x);
                }
                v.push(Symbol::Codeblock(
                    alt_text,
                    block
                        .into_iter()
                        .map(|x| format!("{x}\n"))
                        .collect::<String>(),
                ));
                continue;
            }

            v.push(Symbol::Text(x.to_string()));
        }
        Gemtext(v)
    }
}
