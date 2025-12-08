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

use crate::Bitmap;

pub enum DrawingMode {
    /// Use space for bg and the given character for fg.
    ///
    /// This is the only mode that won’t draw proportionally on terminals.
    Simple(char),
    /// Use space for bg and the given character for fg (wide).
    Wide(char),
    /// Use Unicode block elements (' ', '▀', '▄', '█').
    Blocks,
    /// Use Unicode full block for fg and space for bg (wide).
    WideBlocks,
}

/// Draw a [Bitmap] using the specified drawing mode.
pub(crate) fn draw(mode: DrawingMode, bitmap: Bitmap) -> String {
    use DrawingMode::*;
    match mode {
        Simple(ch) => draw_simple(bitmap, " ", &ch.to_string()),
        Wide(ch) => draw_simple(bitmap, "  ", &ch.to_string().repeat(2)),
        Blocks => draw_blocks(bitmap),
        WideBlocks => draw_simple(bitmap, "  ", "██"),
    }
}

/// Draw [Bitmap] using the given strings for zero and one bits.
fn draw_simple(bitmap: Bitmap, zero: &str, one: &str) -> String {
    let mut result = String::new();
    for row in 0..bitmap.height {
        for col in 0..bitmap.width {
            let bit = bitmap.get_pixel(col, row);
            result.push_str(if bit { one } else { zero });
        }
        result.push('\n');
    }
    result.pop(); // Remove the trailing newline
    result
}

/// Draw a [Bitmap] using Unicode block elements. (' ', '▀', '▄', '█')
fn draw_blocks(bitmap: Bitmap) -> String {
    let mut result = String::new();
    for y in (0..bitmap.height).step_by(2) {
        for x in 0..bitmap.width {
            result.push(match (bitmap.get_pixel(x, y), bitmap.get_pixel(x, y + 1)) {
                (false, false) => ' ',
                (false, true) => '▄',
                (true, false) => '▀',
                (true, true) => '█',
            });
        }
        result.push('\n');
    }
    result.pop(); // Remove the trailing newline
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_consts::*;

    #[test]
    fn test_draw_simple() {
        assert_eq!(
            draw(
                DrawingMode::Simple('#'),
                Bitmap::from_raw_data(LATIN_SMALL_LETTER_A)
            ),
            DRAWING_SIMPLE_LATIN_SMALL_LETTER_A.replace('_', " ")
        );
        assert_eq!(
            draw(
                DrawingMode::Simple('#'),
                Bitmap::from_raw_data(CJK_UNIFIED_IDEOGRAPH_5186)
            ),
            DRAWING_SIMPLE_CJK_UNIFIED_IDEOGRAPH_5186.replace('_', " "),
        );
    }

    #[test]
    fn test_draw_wide() {
        assert_eq!(
            draw(
                DrawingMode::Wide('#'),
                Bitmap::from_raw_data(LATIN_SMALL_LETTER_A)
            ),
            DRAWING_SIMPLE_LATIN_SMALL_LETTER_A
                .replace('#', "##")
                .replace('_', "  ")
        );
        assert_eq!(
            draw(
                DrawingMode::Wide('#'),
                Bitmap::from_raw_data(CJK_UNIFIED_IDEOGRAPH_5186)
            ),
            DRAWING_SIMPLE_CJK_UNIFIED_IDEOGRAPH_5186
                .replace('#', "##")
                .replace('_', "  "),
        );
    }

    #[test]
    fn test_draw_blocks() {
        assert_eq!(
            draw(
                DrawingMode::Blocks,
                Bitmap::from_raw_data(LATIN_SMALL_LETTER_A)
            ),
            DRAWING_BLOCKS_LATIN_SMALL_LETTER_A.replace('_', " "),
        );

        assert_eq!(
            draw(
                DrawingMode::Blocks,
                Bitmap::from_raw_data(CJK_UNIFIED_IDEOGRAPH_5186)
            ),
            DRAWING_BLOCKS_CJK_UNIFIED_IDEOGRAPH_5186.replace('_', " "),
        );
    }

    #[test]
    fn test_draw_wide_blocks() {
        assert_eq!(
            draw(
                DrawingMode::WideBlocks,
                Bitmap::from_raw_data(LATIN_SMALL_LETTER_A)
            ),
            DRAWING_SIMPLE_LATIN_SMALL_LETTER_A
                .replace('#', "██")
                .replace('_', "  "),
        );
        assert_eq!(
            draw(
                DrawingMode::WideBlocks,
                Bitmap::from_raw_data(CJK_UNIFIED_IDEOGRAPH_5186)
            ),
            DRAWING_SIMPLE_CJK_UNIFIED_IDEOGRAPH_5186
                .replace('#', "██")
                .replace('_', "  "),
        );
    }
}
