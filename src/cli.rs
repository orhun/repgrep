use std::path::PathBuf;
use std::process;

use clap::AppSettings::ColoredHelp;
use clap::Clap;
use clap::{crate_authors, crate_version, IntoApp};

// TODO: options to support in the future
// -P/--pcre2
// -F/--fixed-strings
// -U/--multiline
// --multiline-dotall
// -f/--file

/// See `rg --help` for more detailed information on each of the flags passed.
#[derive(Clap, Debug)]
#[clap(
  version = crate_version!(),
  author = crate_authors!(),
  setting = ColoredHelp,
)]
pub struct Args {
    /// The pattern to search. Required unless patterns are passed via -e/--regexp.
    #[clap(name = "PATTERN")]
    pub pattern: Option<String>,
    /// The paths in which to search.
    #[clap(name = "PATH", parse(from_os_str))]
    pub paths: Vec<PathBuf>,
    /// Used to provide multiple patterns.
    #[clap(short = "e", long = "regexp")]
    pub patterns: Vec<String>,

    /// How many lines of context should be shown after each match.
    #[clap(short = "A", long = "after-context")]
    pub after_context: Option<usize>,
    /// How many lines of context should be shown before each match.
    #[clap(short = "B", long = "before-context")]
    pub before_context: Option<usize>,
    /// How many lines of context should be shown before and after each match.
    #[clap(short = "C", long = "context")]
    pub context: Option<usize>,
    /// Treat CRLF ('\r\n') as a line terminator.
    #[clap(long = "crlf")]
    pub crlf: bool,
    /// Provide the encoding to use when searching files.
    #[clap(short = "E", long = "encoding")]
    pub encoding: Option<String>,
    /// Follow symlinks.
    #[clap(short = "L", long = "follow")]
    pub follow_symlinks: bool,
    /// Ignore case when searching.
    #[clap(short = "i", long = "ignore-case")]
    pub ignore_case: bool,
    /// Invert the matches on each line.
    #[clap(short = "v", long = "invert-match")]
    pub invert_match: bool,
    /// Print both matching and non-matching lines.
    #[clap(long = "passthru")]
    pub passthru: bool,
    /// Use smart case matching.
    #[clap(short = "S", long = "smart-case")]
    pub smart_case: bool,
    /// Use case sensitive matching.
    #[clap(short = "s", long = "case-sensitive")]
    pub case_sensitive: bool,
    /// Sort the results (ascending).
    #[clap(long = "sort")]
    pub sort: Option<String>,
    /// Sort the results (descending).
    #[clap(long = "sortr")]
    pub sortr: Option<String>,
    /// How many threads to use.
    #[clap(short = "j", long = "threads")]
    pub threads: Option<usize>,
    /// Trim leading/trailing whitespace.
    #[clap(long = "trim")]
    pub trim: bool,
    /// Search only a specific type of file.
    #[clap(short = "t", long = "type")]
    pub r#type: Vec<String>,
    /// Inverse of --type.
    #[clap(short = "T", long = "type-not")]
    pub type_not: Vec<String>,
    /// Set the "unrestricted" searching options for ripgrep.
    /// Note that this is currently limited to only two occurrences `-uu` since
    /// binary searching is not supported in repgrep.
    #[clap(short = "u", long = "unrestricted", parse(from_occurrences))]
    pub unrestricted: usize,
    /// When matching, use a word boundary search.
    #[clap(short = "w", long = "word-regexp")]
    pub word_regexp: bool,

    /// A list of globs to match files.
    #[clap(short = "g", long = "glob")]
    pub glob: Vec<String>,
    /// A list of case insensitive globs to match files.
    #[clap(long = "iglob")]
    pub iglob: Vec<String>,
    /// Search hidden files.
    #[clap(long = "hidden")]
    pub hidden: bool,
    /// Use the given ignore file when searching.
    #[clap(long = "ignore-file")]
    pub ignore_file: Option<PathBuf>,
    /// When given an --ignore-file, read its rules case insensitively.
    #[clap(long = "ignore-file-case-insensitive")]
    pub ignore_file_case_insensitive: bool,
    /// Don't traverse filesystems for each path specified.
    #[clap(long = "one-file-system")]
    pub one_file_system: bool,
}

impl Args {
    /// Provides the command line arguments to pass down to ripgrep.
    pub fn rg_args(&self) -> impl Iterator<Item = std::ffi::OsString> {
        // Skip the first argument, which _should_ be the binary name.
        std::env::args_os().skip(1)
    }

    /// Returns the patterns used by `rg` in the search.
    pub fn rg_patterns(&self) -> Vec<&str> {
        if let Some(pattern) = &self.pattern {
            vec![pattern]
        } else {
            self.patterns.iter().map(|p| p.as_ref()).collect()
        }
    }
}

pub fn parse_arguments() -> Args {
    let mut args = Args::parse();

    // Check we have a pattern.
    if args.pattern.is_none() && args.patterns.is_empty() {
        eprintln!("\nNo pattern was provided!\n");
        Args::into_app().print_help().unwrap();
        process::exit(1);
    }

    // If a positional pattern was passed _and_ patterns via flags were passed, then
    // assume that the positional pattern is a path.
    if args.pattern.is_some() && !args.patterns.is_empty() {
        args.paths.push(PathBuf::from(args.pattern.take().unwrap()));
    }

    // We don't support binary searches.
    if args.unrestricted > 2 {
        eprintln!("Binary file searching is not supported. Changing -uuu to -uu");
        args.unrestricted = 2;
    }

    args
}

#[cfg(test)]
mod tests {
    #[test]
    fn checks_if_no_pattern_was_passed() {
        unimplemented!();
    }

    #[test]
    fn reads_pattern_as_path_if_pattern_flag_given() {
        unimplemented!();
    }

    #[test]
    fn does_not_allow_unrestricted_above_two() {
        unimplemented!();
    }

    #[test]
    fn returns_rg_patterns() {
        unimplemented!();
    }
}
