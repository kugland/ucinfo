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

use hex_literal::hex;
use indoc::indoc;

pub(crate) const LATIN_SMALL_LETTER_A: &[u8] = &hex!("0000000000003C42023E4242463A0000");

pub(crate) const CJK_UNIFIED_IDEOGRAPH_5186: &[u8] = &hex!(
    "00007FFC410441044104410441047FFC"
    "40044004400440044004400440144008"
);

pub(crate) const DRAWING_SIMPLE_LATIN_SMALL_LETTER_A: &str = indoc! {"
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
    ________"};

pub(crate) const DRAWING_SIMPLE_CJK_UNIFIED_IDEOGRAPH_5186: &str = indoc! {"
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
    _#__________#___"};

pub(crate) const DRAWING_BLOCKS_LATIN_SMALL_LETTER_A: &str = indoc! {"
    ________
    ________
    ________
    _▄▀▀▀▀▄_
    __▄▄▄▄█_
    _█____█_
    _▀▄▄▄▀█_
    ________"};

pub(crate) const DRAWING_BLOCKS_CJK_UNIFIED_IDEOGRAPH_5186: &str = indoc! {"
    _▄▄▄▄▄▄▄▄▄▄▄▄▄__
    _█_____█_____█__
    _█_____█_____█__
    _█▄▄▄▄▄█▄▄▄▄▄█__
    _█___________█__
    _█___________█__
    _█___________█__
    _█_________▀▄▀__"};
