/// Representation of a glyph, either single-width (8x16) or double-width (16x16).
pub struct Glyph(&'static [u8]);

impl Glyph {
    /// Create a new `Glyph` from the raw bitmap data.
    pub fn new(bitmap: &'static [u8]) -> Self {
        Self(bitmap)
    }

    /// Search for a glyph with a given codepoint.
    ///
    /// Returns a reference to a slice of bytes containing the bitmap data for the given codepoint,
    /// or `None` if the codepoint doesn't exist in the font. For single-width glyphs, the slice
    /// will be 16 bytes long; for double-width glyphs, 32 bytes long.
    pub fn from_codepoint(codepoint: u32) -> Option<Self> {
        search_glyph_recursive(codepoint, FONT_SINGLE, 16)
            .or_else(|| search_glyph_recursive(codepoint, FONT_DOUBLE, 32))
            .map(Self::new)
    }

    /// Get the width of the glyph in pixels.
    pub fn width(&self) -> usize {
        match self.0.len() {
            16 => 1,
            32 => 2,
            width => panic!("Invalid bitmap length: {width}"),
        }
    }

    /// Draw the glyph using the given characters for zero and one bits.
    pub fn draw_ascii(&self, zero: &str, one: &str) -> String {
        let mut result = String::new();
        for row in self.0.chunks(self.width()) {
            for byte in row.iter() {
                for bit in (0..8).rev() {
                    result.push_str(if byte & (1 << bit) != 0 { one } else { zero });
                }
            }
            result.push('\n');
        }
        result.pop(); // Remove the trailing newline
        result
    }

    /// Draw the glyph using Unicode block elements. (▀, ▄, █)
    pub fn draw_blocks(&self) -> String {
        let lines = self.draw_ascii(" ", "#");
        let ascii = lines.lines().collect::<Vec<_>>();
        let mut result = String::new();
        for chunk in ascii.chunks(2) {
            for col in 0..(chunk[0].len()) {
                let top = chunk[0].chars().nth(col).unwrap();
                let bottom = chunk[1].chars().nth(col).unwrap();
                result.push(match (top, bottom) {
                    (' ', ' ') => ' ',
                    (' ', '#') => '▄',
                    ('#', ' ') => '▀',
                    ('#', '#') => '█',
                    _ => unreachable!(),
                });
            }
            result.push('\n');
        }
        result.pop(); // Remove the trailing newline
        result
    }
}

/// Recursive binary search for glyph in a font.
fn search_glyph_recursive(codepoint: u32, font: &[u8], glyph_size: usize) -> Option<&[u8]> {
    let record_size = 4 + glyph_size;
    debug_assert_eq!(font.len() % record_size, 0);

    let total_records = font.len() / record_size;
    let index = total_records / 2 * record_size;
    let code = u32::from_le_bytes(font[index..index + 4].try_into().unwrap());
    if total_records == 1 && code != codepoint {
        None
    } else if code < codepoint {
        search_glyph_recursive(codepoint, &font[index..], glyph_size)
    } else if code > codepoint {
        search_glyph_recursive(codepoint, &font[..index], glyph_size)
    } else {
        Some(&font[index + 4..index + 4 + glyph_size])
    }
}

// These fonts are formatted as records of either 4 + 16 or 4 + 32 bytes, the first 4 being the
// codepoint (little-endian) and the rest being the bitmap data. The records are sorted by
// codepoint, so we can use binary search to find the bitmap for a given codepoint.
const FONT_SINGLE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/font_single.bin"));
const FONT_DOUBLE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/font_double.bin"));

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;
    use indoc::indoc;

    const LOWERCASE_A: &[u8] = &hex!("0000000000003C42023E4242463A0000");

    const YEN_KANJI: &[u8] = &hex!(
        "00007FFC410441044104410441047FFC"
        "40044004400440044004400440144008"
    );

    #[test]
    fn test_from_codepoint() {
        assert_eq!(
            Glyph::from_codepoint(0x20).unwrap().0,
            &[0; 16],
            "Glyph for U+0020 SPACE is blank 8x16"
        );
        assert_eq!(
            Glyph::from_codepoint(0x3000).unwrap().0,
            &[0; 32],
            "Glyph for U+3000 IDEOGRAPHIC SPACE is blank 8x32"
        );
        assert!(
            Glyph::from_codepoint(0x110000).is_none(),
            "Out-of-range codepoint has no glyph"
        );
        assert_eq!(
            Glyph::from_codepoint(0x0061).unwrap().0,
            LOWERCASE_A,
            "Test glyph for letter a. If the design changes, this test will fail.",
        );
        assert_eq!(
            Glyph::from_codepoint(0x5186).unwrap().0,
            YEN_KANJI,
            "Test glyph for 円. If the design changes, this test will fail.",
        );
    }

    #[test]
    fn test_glyph_width() {
        assert_eq!(Glyph(LOWERCASE_A).width(), 1, "Single-width glyph");
        assert_eq!(Glyph(YEN_KANJI).width(), 2, "Double-width glyph");
    }

    #[test]
    fn test_glyph_draw_ascii() {
        assert_eq!(
            Glyph(LOWERCASE_A).draw_ascii("_", "#"),
            indoc! {"
                ________
                ________
                ________
                ________
                ________
                ________
                __####__
                _#____#_
                ______#_
                __#####_
                _#____#_
                _#____#_
                _#___##_
                __###_#_
                ________
                ________
            "},
            "Draw ASCII art for U+0061 LATIN SMALL LETTER A",
        );
        assert_eq!(
            Glyph(YEN_KANJI).draw_ascii("_", "#"),
            indoc! {"
                ________________
                _#############__
                _#_____#_____#__
                _#_____#_____#__
                _#_____#_____#__
                _#_____#_____#__
                _#_____#_____#__
                _#############__
                _#___________#__
                _#___________#__
                _#___________#__
                _#___________#__
                _#___________#__
                _#___________#__
                _#_________#_#__
                _#__________#___
            "},
            "Draw ASCII art for U+5186 YEN SIGN (円)",
        );
    }

    #[test]
    fn test_glyph_draw_blocks() {
        assert_eq!(
            Glyph(LOWERCASE_A).draw_blocks().replace(' ', "_"),
            indoc! {"
                ________
                ________
                ________
                _▄▀▀▀▀▄_
                __▄▄▄▄█_
                _█____█_
                _▀▄▄▄▀█_
                ________
            "},
            "Draw block art for U+0061 LATIN SMALL LETTER A"
        );

        assert_eq!(
            Glyph(YEN_KANJI).draw_blocks().replace(' ', "_"),
            indoc! {"
                _▄▄▄▄▄▄▄▄▄▄▄▄▄__
                _█_____█_____█__
                _█_____█_____█__
                _█▄▄▄▄▄█▄▄▄▄▄█__
                _█___________█__
                _█___________█__
                _█___________█__
                _█_________▀▄▀__
            "},
            "Draw block art for U+5186 YEN SIGN (円)"
        );
    }

    #[test]
    fn test_font_record_size() {
        assert_eq!(
            FONT_SINGLE.len() % 20,
            0,
            "Size of 8x16 font is a multiple of 20"
        );
        assert_eq!(
            FONT_DOUBLE.len() % 36,
            0,
            "Size of 16x16 font is a multiple of 36"
        );
    }
}
