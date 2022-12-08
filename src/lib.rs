//! # Subtitle Parser
//!
//! A parser for *(currently only)* YouTube subtitles
//!
//! **TODO:**
//! - Expand on this to only take a YT URL and the code does everything automatically
//! - Later use clap crate to turn this into CLI
//! - Later allow this to be serialized

use anyhow::Result;
use std::fs;
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
        let file_contents = fs::read_to_string(file_path)?;

        let mut lines_vec = file_contents
            .split('\n')
            .step_by(2)
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        lines_vec.iter_mut().for_each(|s| s.push(' '));

        let mut subtitles = String::new();
        lines_vec.iter().for_each(|s| subtitles.push_str(s));

        Ok(Self { subtitles })
    }
}

impl std::fmt::Display for Subtitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.subtitles)
    }
}
