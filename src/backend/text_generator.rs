// Morse - text_generator.rs
// Copyright (C) 2025-2026  Jaŭhien Lavonćjeŭ <jauhien.lavoncjeu@gmail.com>
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

use rand::prelude::*;
use rand::rngs::{StdRng, SysRng};
use anyhow::{anyhow, Result};

pub fn generate_text(seed: Option<u64>, chars_count: u64, word_length: u64, chars_in_use: Vec<char>) -> Result<String> {
    let mut rng: StdRng;

    match seed {
        Some(value) => rng = StdRng::seed_from_u64(value),
        None => rng = StdRng::try_from_rng(&mut SysRng).unwrap(),
    }

    if word_length == 0 {
        return Err(anyhow!("The value of `word_length` must be greater than 0."));
    }

    if word_length > chars_count {
        return Err(anyhow!("The value of `chars_count` must be greater than or equal to `word_length`."));
    }

    if chars_in_use.is_empty() {
        return Err(anyhow!("The Vec<char> `chars_in_use` cannot be empty."));
    }

    let mut unused_chars = chars_in_use.clone();
    let mut result = Vec::<char>::new();
    let mut previous_chars = vec!('\0', '\0');

    for i in 1..=chars_count {
        let mut new_char: char;

        loop {
            if chars_count - i < chars_in_use.len() as u64 && !unused_chars.is_empty() {
                new_char = *unused_chars.choose(&mut rng).unwrap();
            }
            else {
                new_char = *chars_in_use.choose(&mut rng).unwrap();
            }

            // 3 chars cannot be in a row
            if !(new_char == previous_chars[0] && new_char == previous_chars[1] && chars_in_use.len() > 1) {
                break;
            }
        }

        previous_chars[0] = previous_chars[1];
        previous_chars[1] = new_char;

        if let Some(index) = unused_chars.iter().position(|&x| x == new_char) {
            unused_chars.remove(index);
        }

        result.push(new_char);
    
        if i % word_length == 0 && i != chars_count {
            result.push(' ');
            previous_chars[0] = previous_chars[1];
            previous_chars[1] = ' ';
        }
    }

    Ok(result.iter().collect())
}
