use anyhow::Result;
use subtitle_parser::Subtitle;

fn main() -> Result<()> {
    let subtitles = Subtitle::new_from_file("input.txt".as_ref())?;
    subtitles.write_to_file()?;

    Ok(())
}
