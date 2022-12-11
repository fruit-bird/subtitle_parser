//! # Subtitle Parser
//!
//! A parser for *(currently only)* YouTube subtitles
//!
//! **TODO:**
//! - Expand on this to only take a YT URL and the code does everything automatically
//! - Convert type from String to &str for less heap allocations
//! - Later use clap to turn this into a CLI
//! - Serialization???

use anyhow::Result;
use std::fmt::Display;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

#[derive(Debug, Default)]
pub struct Subtitle {
    // pub video: Option<YouTubeVideo>,
    pub subtitles: String,
}

// #[derive(Debug, Default)]
// pub struct YouTubeVideo {
//     pub url: String,
//     pub title: String,
// }

impl Subtitle {
    /// Parses an input file of YT subtitles that look like a poem into a single solid block of text
    pub fn new_from_file(file_path: impl AsRef<Path>) -> Result<Self> {
        let input = fs::read_to_string(file_path)?;
        let subtitles = Self::subtitle_formatter(input);
        Ok(Self { subtitles })
    }

    /// Writes the formatted
    pub fn write_to_file(&self) -> Result<File> {
        let mut output_file = File::create("output.txt")?;
        output_file.write_all(self.subtitles.as_bytes())?;
        Ok(output_file)
    }

    /// Where the magic happens.
    ///
    /// Put in a separate function for reusability
    fn subtitle_formatter(input_string: impl AsRef<str>) -> String {
        let mut output_string = String::new();
        input_string
            .as_ref()
            .split('\n')
            .step_by(2)
            .for_each(|s| output_string.push_str(format!("{} ", s).as_str()));

        output_string
    }
}

impl From<&str> for Subtitle {
    fn from(s: &str) -> Self {
        let subtitles = Self::subtitle_formatter(s);
        Self { subtitles }
    }
}

impl Display for Subtitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.subtitles)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FORMATTED_TEXT: &str = "let me see the tongue oh my god who is that coming off the pick and roll ha ha ha oh man oh man hold on can we ";
    const UNFORMATTED_TEXT: &str = r#"let me see the tongue oh my god

who is that coming off the pick and roll

ha ha ha oh man oh man hold on can we"#;

    #[test]
    fn subtitle_formatting_from_file() -> Result<()> {
        let file_path = "test_file.txt";

        let mut file = File::create(file_path)?;
        file.write_all(UNFORMATTED_TEXT.as_bytes())?;

        let subs = Subtitle::new_from_file(file_path)?;
        assert_eq!(subs.subtitles, FORMATTED_TEXT);

        std::fs::remove_file(file_path)?; // deleting the temp file
        Ok(())
    }

    #[test]
    fn subtitle_formatting_from_str() {
        let subs = Subtitle::from(UNFORMATTED_TEXT);
        assert_eq!(subs.subtitles, FORMATTED_TEXT);
    }

    #[test]
    fn check_from_str() -> Result<()> {
        let subs = Subtitle::from(UNFORMATTED_TEXT);
        assert_eq!(subs.subtitles, FORMATTED_TEXT);

        Ok(())
    }
}
