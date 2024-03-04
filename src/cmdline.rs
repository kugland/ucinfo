use clap::{ArgAction, Args, Parser, ValueEnum};
use once_cell::sync::Lazy;
use regex::Regex;

/// Command-line tool to display information about Unicode characters.
#[derive(Debug, Parser)]
#[clap(version, disable_help_flag = true, disable_version_flag = true)]
pub struct Cmdline {
    #[clap(flatten)]
    command: Command,

    /// How to display the glyph.
    #[clap(short, long, default_value = "blocks", value_name = "STYLE", help_heading = Some("Display options"))]
    pub glyphs: Glyphs,

    /// Show this help message.
    #[clap(short, long, action = ArgAction::Help, help_heading = Some("Information"))]
    help: (),

    /// Show the version of the program.
    #[clap(short, long, action = ArgAction::Help, help_heading = Some("Information"))]
    version: (),
}

#[derive(Debug, Args)]
#[group(required = true, multiple = false)]
struct Command {
    /// The characters to display.
    #[clap(help_heading = Some("Query"), display_order = 0)]
    chars: Vec<String>,

    /// Search the name of the character using a regex.
    #[clap(short, long, value_name = "REGEX", help_heading = Some("Query"), display_order = 1)]
    search: Option<String>,
}

/// How to display the glyph.
#[derive(Clone, Debug, Parser, ValueEnum)]
pub enum Glyphs {
    /// Draw the glyph using ASCII characters.
    Ascii,
    /// Draw the glyph using Unicode block elements. (▀, ▄, █)
    Blocks,
    /// Do not draw the glyph.
    None,
}

impl Cmdline {
    /// Get the codepoints from the characters given on the command line.
    pub fn codepoints(&self) -> Vec<u32> {
        static CP_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[Uu]\+[0-9a-fA-F]{4,6}$").unwrap());

        self.command.chars
            .iter()
            .flat_map(|s| {
                if CP_RE.is_match(s) {
                    vec![u32::from_str_radix(&s[2..], 16).unwrap()]
                } else {
                    s.chars().map(char::into).collect()
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmdline() {
        let cmdline = Cmdline::parse_from(vec!["bin", "U+0061", "test"]);
        assert_eq!(cmdline.codepoints(), vec![0x61, 0x74, 0x65, 0x73, 0x74]);
    }
}
