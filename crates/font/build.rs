use anyhow::anyhow;
use std::path::Path;
use std::{env, fs, path};

/// A single glyph in a font, consisting of a codepoint and a bitmap.
struct Glyph(u32, Vec<u8>);

impl Glyph {
    /// Parse a line from a font file into a `Glyph` struct.
    ///
    /// The line should be in the format `XXXX:YY...YY`, where `XXXX` is the hexadecimal codepoint,
    /// and `YY...YY` is a sequence of hexadecimal digits representing the raw bitmap data.
    fn from_line(line_number: usize, line: &str) -> anyhow::Result<Self> {
        let error = || anyhow::anyhow!("Invalid line: {line_number}: {line}");
        let (code, data) = line.split_once(':').ok_or_else(error)?;
        let code = u32::from_str_radix(code, 16)?;
        let mut out: Vec<u8> = vec![0; data.len() / 2];
        hex::decode_to_slice(data, &mut out).map_err(|_| error())?;
        if out.len() != 16 && out.len() != 32 {
            return Err(error());
        }
        Ok(Self(code, out))
    }

    /// Calculate the width of the glyph in pixels, assuming a height of 16 pixels.
    fn width(&self) -> u8 {
        (self.1.len()) as u8 / 2
    }

    /// Convert the glyph to a binary representation.
    fn append_to_vec(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.0.to_ne_bytes());
        out.extend_from_slice(&self.1);
    }
}

/// Load all glyphs from the `fonts` directory into a vector, sorted by codepoint.
fn load_font() -> anyhow::Result<(String, Vec<Glyph>)> {
    let font_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("data")
        .join("unifont");

    if !font_dir.exists() {
        return Err(anyhow!("Font directory does not exist: {:?}", font_dir));
    }

    let version = {
        // Load metadata to get version
        let metadata = font_dir.join("metadata.json");
        let contents = fs::read_to_string(metadata.as_path())?;
        let metadata: serde_json::Value = serde_json::from_str(&contents)?;
        metadata["version"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing version in metadata"))?
            .to_string()
    };

    // Load glyphs from unifont.hex
    let all_glyphs = {
        let font_file = font_dir.join("unifont.hex");
        let contents = fs::read_to_string(font_file.as_path())?;
        let mut all_glyphs = contents
            .lines()
            .enumerate()
            .map(|(line_number, line)| Glyph::from_line(line_number, line))
            .collect::<anyhow::Result<Vec<Glyph>>>()?;
        all_glyphs.sort_by_key(|g| g.0);
        all_glyphs
    };

    Ok((version, all_glyphs))
}

/// Save all glyphs with a given width to a binary file.
fn save_unifont_glyphs_bin<P: AsRef<Path>>(
    glyphs: &[Glyph],
    width: u8,
    filename: P,
) -> anyhow::Result<()> {
    let mut data = Vec::new();
    let binding = env::var_os("OUT_DIR").unwrap();
    let out_dir = path::Path::new(&binding);

    for glyph in glyphs.iter().filter(|g| g.width() == width) {
        glyph.append_to_vec(&mut data);
    }

    fs::write(out_dir.join(filename), &data)?;
    Ok(())
}

/// Save the Unifont version to a text file.
fn save_unifont_version(version: &str) -> anyhow::Result<()> {
    let binding = env::var_os("OUT_DIR").unwrap();
    let out_dir = path::Path::new(&binding);
    fs::write(out_dir.join("unifont_version.txt"), version)?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let (version, all_glyphs) = load_font()?;

    save_unifont_version(&version)?;
    save_unifont_glyphs_bin(&all_glyphs, 8, "unifont_glyphs_8x16.bin")?;
    save_unifont_glyphs_bin(&all_glyphs, 16, "unifont_glyphs_16x16.bin")?;

    println!("cargo:rerun-if-changed=../../data/unifont/metadata.json");
    println!("cargo:rerun-if-changed=../../data/unifont/unifont.hex");

    Ok(())
}
