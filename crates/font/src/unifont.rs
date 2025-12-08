////////       This file is part of the source code for ucinfo, a CLI tool to show         ////////
////////       information about Unicode characters.                                       ////////
////////                                                                                   ////////
////////       Copyright © 2024  André Kugland                                             ////////
////////                                                                                   ////////
////////       This program is free software: you can redistribute it and/or modify        ////////
////////       it under the terms of the GNU General Public License as published by        ////////
////////       the Free Software Foundation, either version 3 of the License, or           ////////
////////       (at your option) any later version.                                         ////////
////////                                                                                   ////////
////////       This program is distributed in the hope that it will be useful,             ////////
////////       but WITHOUT ANY WARRANTY; without even the implied warranty of              ////////
////////       MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the                ////////
////////       GNU General Public License for more details.                                ////////
////////                                                                                   ////////
////////       You should have received a copy of the GNU General Public License           ////////
////////       along with this program. If not, see https://www.gnu.org/licenses/.         ////////

/// Search for a glyph entry by its codepoint.
///
/// Returns a reference to a slice of bytes containing the raw bitmap, or `None` if the codepoint
/// doesn’t exist in the font.
pub(crate) fn find_entry(codepoint: u32) -> Option<&'static [u8]> {
    find_entry_internal(codepoint, UNIFONT_GLYPHS_8X16.0)
        .or_else(|| find_entry_internal(codepoint, UNIFONT_GLYPHS_16X16.0))
}

/// Recursive binary search for glyph entry in a font.
fn find_entry_internal<const N: usize>(
    codepoint: u32,
    font: &'static [[u8; N]],
) -> Option<&'static [u8]> {
    let idx = font
        .binary_search_by_key(&codepoint, |record| {
            u32::from_ne_bytes(record[0..4].try_into().unwrap())
        })
        .ok()?;
    Some(&font[idx][4..])
}

/// Version of the included Unifont font.
pub const UNIFONT_VERSION: &str = include_str!(env!("UNIFONT_VERSION_FILE"));

// These fonts are formatted as records of either 4 + 16 or 4 + 32 bytes, the first 4 being the
// codepoint and the rest being the bitmap data. The records are sorted by codepoint, so we can
// use binary search to find the bitmap for a given codepoint.
const UNIFONT_GLYPHS_8X16: (&[[u8; 20]], &[u8]) =
    include_bytes!(env!("UNIFONT_GLYPHS_8X16_FILE")).as_chunks::<20>();
const UNIFONT_GLYPHS_16X16: (&[[u8; 36]], &[u8]) =
    include_bytes!(env!("UNIFONT_GLYPHS_16X16_FILE")).as_chunks::<36>();

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_consts::*;

    #[test]
    fn test_find_entry() {
        assert_eq!(find_entry(0x20).unwrap(), &[0; 16]);
        assert_eq!(find_entry(0x0061).unwrap(), LATIN_SMALL_LETTER_A);
        assert_eq!(find_entry(0x3000).unwrap(), &[0; 32]);
        assert_eq!(find_entry(0x5186).unwrap(), CJK_UNIFIED_IDEOGRAPH_5186);
        assert!(find_entry(0x110000).is_none());
    }

    #[test]
    fn test_font_record_size() {
        assert_eq!(UNIFONT_GLYPHS_8X16.1.len(), 0);
        assert_eq!(UNIFONT_GLYPHS_16X16.1.len(), 0);
    }
}
