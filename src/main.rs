mod cmdline;
mod glyph;
use clap::Parser;

use crate::{cmdline::Cmdline, glyph::Glyph};

fn main() -> std::io::Result<()> {
    let cmdline = Cmdline::parse();

    println!("{:?}", cmdline);

    for cp in cmdline.codepoints() {
        if let Some(glyph) = Glyph::from_codepoint(cp) {
            match cmdline.glyphs {
                cmdline::Glyphs::Ascii => println!("{}", glyph.draw_ascii("  ", "##")),
                cmdline::Glyphs::Blocks => println!("{}", glyph.draw_blocks()),
                cmdline::Glyphs::None => {}
            }
        } else {
            eprintln!("Invalid codepoint: U+{:04X}", cp);
        }
    }
    Ok(())
}
