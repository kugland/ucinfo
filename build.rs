use anyhow::anyhow;
use std::collections::HashSet;
use std::{env, fs, path};

/// A single glyph in a font, consisting of a codepoint and a bitmap.
struct Glyph(u32, Vec<u8>);

impl Glyph {
    /// Parse a line from a font file into a `Glyph` struct.
    ///
    /// The line should be in the format `XXXX:YY...YY`, where `XXXX` is the hexadecimal codepoint,
    /// and `YY...YY` is a sequence of hexadecimal digits representing the raw bitmap data.
    fn from_line(line_number: usize, line: &str) -> anyhow::Result<Self> {
        let (code, data) = line
            .split_once(':')
            .ok_or_else(|| anyhow::anyhow!("Invalid line: {line_number}: {line}"))?;
        let code = u32::from_str_radix(code, 16)?;
        let mut out: Vec<u8> = vec![0; data.len() / 2];
        hex::decode_to_slice(data, &mut out)?;
        Ok(Self(code, out))
    }

    /// Calculate the width of the glyph in pixels, assuming a height of 16 pixels.
    fn width(&self) -> u8 {
        (self.1.len()) as u8 / 2
    }

    /// Convert the glyph to a binary representation.
    fn to_vec(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.0.to_le_bytes());
        out.extend_from_slice(&self.1);
    }
}

/// Load all glyphs from the `fonts` directory into a vector, sorted by codepoint.
fn load_glyphs() -> anyhow::Result<Vec<Glyph>> {
    let mut seen_codes: HashSet<u32> = HashSet::new();
    let mut glyphs: Vec<Glyph> = vec![];
    for font_file in fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/fonts"))? {
        let font_file = font_file?;
        let contents = fs::read_to_string(font_file.path())?;
        for (line_number, line) in contents.lines().enumerate() {
            let glyph = Glyph::from_line(line_number, line)?;
            if glyph.width() != 8 && glyph.width() != 16 {
                return Err(anyhow!("Invalid width: U+{:04X}", glyph.0));
            }
            if !seen_codes.insert(glyph.0) {
                return Err(anyhow!("Duplicate codepoint: U+{:04X}", glyph.0));
            }
            glyphs.push(glyph);
        }
    }
    glyphs.sort_by_key(|g| g.0);
    Ok(glyphs)
}

/// Save all glyphs with a given width to a binary file.
fn save_font(glyphs: &[Glyph], width: u8, filename: &str) -> anyhow::Result<()> {
    let mut data = Vec::new();
    let binding = env::var_os("OUT_DIR").unwrap();
    let out_dir = path::Path::new(&binding);

    for glyph in glyphs.iter().filter(|g| g.width() == width) {
        glyph.to_vec(&mut data);
    }
    fs::write(out_dir.join(filename), &data)?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let glyphs = load_glyphs()?;

    save_font(&glyphs, 8, "font_single.bin")?;
    save_font(&glyphs, 16, "font_double.bin")?;

    println!("cargo:rerun-if-changed=fonts/");

    Ok(())
}
