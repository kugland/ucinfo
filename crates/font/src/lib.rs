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

mod bitmap;
mod draw;
mod unifont;

#[cfg(test)]
mod test_consts;

pub use draw::DrawingMode;
pub use unifont::UNIFONT_VERSION;

use crate::{bitmap::Bitmap, draw::draw, unifont::find_entry};

/// Draw the glyph for the specified codepoint, in the specified mode.
pub fn draw_glyph(codepoint: u32, mode: DrawingMode) -> Option<String> {
    find_entry(codepoint).map(|raw| draw(mode, Bitmap::from_raw_data(raw)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_consts::*;

    // One final test to rule them all.
    #[test]
    fn test_draw_glyph() {
        assert_eq!(
            draw_glyph(0x61, DrawingMode::Simple('#')),
            Some(DRAWING_SIMPLE_LATIN_SMALL_LETTER_A.replace('_', " "))
        );
        assert_eq!(
            draw_glyph(0x5186, DrawingMode::Blocks),
            Some(DRAWING_BLOCKS_CJK_UNIFIED_IDEOGRAPH_5186.replace('_', " "))
        );
    }
}
