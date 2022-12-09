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

#[derive(Default)]
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
    /// Normal constructor
    pub fn new(subtitles: impl Into<String>) -> Self {
        Self {
            subtitles: subtitles.into(),
        }
    }

    /// Parses an input file of YT subtitles that look like a poem into a single solid block of text
    pub fn new_from_file(file_path: &Path) -> Result<Self> {
        let mut subtitles = String::new();

        fs::read_to_string(file_path)?
            .split('\n')
            .step_by(2)
            .for_each(|s| subtitles.push_str(format!("{} ", s).as_str()));

        Ok(Self { subtitles })
    }

    pub fn write_to_file(&self) -> Result<()> {
        let mut output_file = File::create("output.txt")?;
        output_file.write_all(self.subtitles.as_bytes())?;

        Ok(())
    }
}

impl Display for Subtitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.subtitles)
    }
}
