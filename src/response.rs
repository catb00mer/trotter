use std::{fs::File, io::Write};

use crate::error::ResponseErr;

/// A gemini response.
#[derive(Debug)]
pub struct Response {
    pub status:  u8,
    pub meta:    String,
    pub content: Vec<u8>,
}

type Result<T> = std::result::Result<T, ResponseErr>;

impl Response {
    /// Return gemtext (if any) inside this response.
    pub fn gemtext(&self) -> Result<String> {
        self.require_status(20)?;
        if let Some(pos) = self.meta.find("text/gemini") {
            if pos == 0 {
                return Ok(self.text()?);
            }
        }

        Err(ResponseErr::UnexpectedFiletype(
            "text/gemini".to_string(),
            self.meta.clone(),
        ))
    }

    /// Return utf8 text (if any) inside this response, regardless of mimetype.
    pub fn text(&self) -> Result<String> {
        self.require_status(20)?;
        Ok(std::str::from_utf8(&self.content)?.to_string())
    }

    /// Save response to file.
    pub fn save(&self, file: &mut File) -> Result<()> {
        self.require_status(20)?;
        file.write_all(&self.content)
            .map_err(|e| ResponseErr::FileWrite(e))?;
        Ok(())
    }

    /// (private) Error if `s` doesn't match the status
    fn require_status(&self, s: u8) -> Result<()> {
        if self.status != s {
            Err(ResponseErr::UnexpectedStatus(s.into(), self.status.into()))
        } else {
            Ok(())
        }
    }
}
