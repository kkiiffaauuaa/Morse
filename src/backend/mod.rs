// Morse - mod.rs
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

use std::time::Duration;
use crate::constants::{CODEX_DURATION, PARIS_DURATION, DIGITS_DURATION};

pub mod text_generator;
pub mod settings;

#[derive(PartialEq, Default, Clone, Copy)]
pub enum TextType {
    #[default]
    Letters,
    Digits,
    Mixed,
}

#[derive(PartialEq, Display, Default, Clone, Copy, Debug)]
pub enum SpeedSystem {
    #[default]
    CODEX,
    PARIS
}

pub fn calculate_dot_duration(speed: f64, speed_system: SpeedSystem, text_type: Option<TextType>) -> Duration {
    let speed_to_use: f64;

    if let Some(text_type) = text_type {
        speed_to_use = match text_type {
            TextType::Letters => if speed_system == SpeedSystem::CODEX { CODEX_DURATION } else { PARIS_DURATION },
            TextType::Digits => DIGITS_DURATION,
            TextType::Mixed => ((if speed_system == SpeedSystem::CODEX { CODEX_DURATION } else { PARIS_DURATION }) + DIGITS_DURATION) / 2.0
        };
    }
    else {
        speed_to_use = if speed_system == SpeedSystem::CODEX { CODEX_DURATION } else { PARIS_DURATION };
    }

    Duration::from_secs_f64(speed_to_use * 100.0 / speed)
}
