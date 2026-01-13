// Morse - constants.rs
// Copyright (C) 2025  Jaŭhien Lavonćjeŭ <jauhien.lavoncjeu@gmail.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// GENERATION
pub const LETTERS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];
pub const DIGITS: [char; 10] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
];
pub const MIXED: [char; 41] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    '/', '?', '.', ',', '='
];
pub const ALLOWED_CHARS: [char; 43] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    '/', '?', '.', ',', '=', '+', ' ',
];

// PLAYING
pub const START_TEXT: &str = "VVV = ";
pub const END_TEXT: &str = " +";
pub const START_TEXT_COMPETITIONS_LETTERS: &str = "OOOOO ";
pub const START_TEXT_COMPETITIONS_DIGITS: &str = "00000 ";

// VISUALIZATION
pub const DEFAULT_TEXT: &str = "HELLO MORSE";
pub const WORD_JOINER: char = '\u{2060}';
pub const ZERO_WIDTH_NO_JOINER: char = '\u{200C}';