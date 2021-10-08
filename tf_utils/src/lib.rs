//! Utility functions for [Tubefeeder-extractor](https://github.com/Tubefeeder/tubefeeder-extractor), e.g. parsing of RSS and
//! parsing of human-readable times.

pub mod rss;

/// Parse textual upload date (e.g. `4 months ago`) to a approximate date.
pub fn timeago_parser<S: AsRef<str>>(
    date: S,
) -> Result<chrono::NaiveDateTime, tf_core::ParseError> {
    let duration_ago = parse_duration::parse(date.as_ref())
        .map_err(|_| tf_core::ParseError("Parsing date".to_string()))?;
    return Ok(
        chrono::Local::now().naive_local() - chrono::Duration::from_std(duration_ago).unwrap()
    );
}
