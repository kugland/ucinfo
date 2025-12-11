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
        let (codepoint, data) = line.split_once(':').ok_or_else(error)?;
        let codepoint = u32::from_str_radix(codepoint, 16).map_err(|_| error())?;
        let mut bitmap: Vec<u8> = vec![0; data.len() / 2];
        hex::decode_to_slice(data, &mut bitmap).map_err(|_| error())?;
        if bitmap.len() != 16 && bitmap.len() != 32 {
            return Err(error());
        }
        Ok(Self(codepoint, bitmap))
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

fn output_file_path<P: AsRef<Path>>(filename: P) -> anyhow::Result<path::PathBuf> {
    let binding = env::var_os("OUT_DIR").ok_or_else(|| anyhow!("OUT_DIR not set"))?;
    Ok(path::Path::new(&binding).join(filename))
}

/// Load all glyphs from the `fonts` directory into a vector, sorted by codepoint.
fn load_font(
    metadata_file: &Path,
    unifont_hex_file: &Path,
) -> anyhow::Result<(String, Vec<Glyph>)> {
    let version = {
        // Load metadata to get version
        let contents = fs::read_to_string(metadata_file)?;
        let metadata: serde_json::Value = serde_json::from_str(&contents)?;
        metadata["version"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing version in metadata"))?
            .to_string()
    };

    // Load glyphs from unifont.hex
    let all_glyphs = {
        let contents = fs::read_to_string(unifont_hex_file)?;
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
) -> anyhow::Result<String> {
    let mut data = Vec::new();
    let font_file = output_file_path(filename)?;

    for glyph in glyphs.iter().filter(|g| g.width() == width) {
        glyph.append_to_vec(&mut data);
    }

    fs::write(&font_file, &data)?;

    Ok(font_file.to_string_lossy().to_string())
}

/// Save the Unifont version to a text file.
fn save_unifont_version(version: &str) -> anyhow::Result<String> {
    let version_file = output_file_path("unifont_version.txt")?;
    fs::write(&version_file, version)?;
    Ok(version_file.to_string_lossy().to_string())
}

fn main() -> anyhow::Result<()> {
    let data_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("data")
        .join("unifont");

    if !data_dir.exists() {
        return Err(anyhow::anyhow!(
            "Data directory does not exist: {}",
            data_dir.display()
        ));
    }

    let metadata_file = data_dir.join("metadata.json");
    let unifont_hex_file = data_dir.join("unifont.hex");

    let (version, all_glyphs) = load_font(&metadata_file, &unifont_hex_file)?;

    let version_file = save_unifont_version(&version)?;
    let glyphs_8x16_file = save_unifont_glyphs_bin(&all_glyphs, 8, "unifont_glyphs_8x16.bin")?;
    let glyphs_16x16_file = save_unifont_glyphs_bin(&all_glyphs, 16, "unifont_glyphs_16x16.bin")?;

    println!("cargo:rerun-if-changed={}", metadata_file.display());
    println!("cargo:rerun-if-changed={}", unifont_hex_file.display());
    println!("cargo:rustc-env=UNIFONT_VERSION_FILE={version_file}");
    println!("cargo:rustc-env=UNIFONT_GLYPHS_8X16_FILE={glyphs_8x16_file}");
    println!("cargo:rustc-env=UNIFONT_GLYPHS_16X16_FILE={glyphs_16x16_file}");

    Ok(())
}
