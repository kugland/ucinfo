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

/// Representation of two-dimensional bitmap data.
pub(crate) struct Bitmap {
    pub width: usize,
    pub height: usize,
    bits: &'static [u8],
}

impl Bitmap {
    /// Create a new [Bitmap] from raw byte data.
    pub(crate) fn from_raw_data(bits: &'static [u8]) -> Self {
        let (width, height) = (bits.len() / 2, 16);
        Self {
            width,
            height,
            bits,
        }
    }

    /// Get the bit at the specified (x, y) coordinate.
    pub(crate) fn get_pixel(&self, x: usize, y: usize) -> u8 {
        let idx = y * (self.width / 8) + (x / 8);
        let bit = 7 - (x % 8);
        (self.bits[idx] >> bit) & 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_consts::*;

    #[test]
    fn test_bitmap_dimensions() {
        let Bitmap { width, height, .. } = Bitmap::from_raw_data(LATIN_SMALL_LETTER_A);
        assert_eq!(width, 8);
        assert_eq!(height, 16);

        let Bitmap { width, height, .. } = Bitmap::from_raw_data(CJK_UNIFIED_IDEOGRAPH_5186);
        assert_eq!(width, 16);
        assert_eq!(height, 16);
    }
}
