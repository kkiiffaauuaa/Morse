// Morse - settings_manager.rs
// Copyright (C) 2025-2026  Jaŭhien Lavonćjeŭ <jauhien.lavoncjeu@gmail.com>
//               2021-2022  Felix Häcker (Original Author, Shortwave)
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

use gio::prelude::*;
use gtk::{gio, glib};

use crate::config;
use crate::backend::settings::Key;

#[derive(Clone, Debug)]
pub struct SettingsManager {
    settings: gio::Settings
}

impl SettingsManager {
    pub fn new() -> Self {
        SettingsManager {
            settings: gio::Settings::new(config::APP_ID)
        }
    }

    #[allow(dead_code)]
    pub fn connect_changed<F>(&self, key: Key, callback: F)
    where F: Fn(&gio::Settings, &str) + 'static {
        self.settings.connect_changed(Some(key.to_string().as_str()), callback);
    }

    #[allow(dead_code)]
    pub fn bind_property<P: IsA<glib::Object>>(&self, key: Key, object: &P, property: &str) {
        self.settings
            .bind(key.to_string().as_str(), object, property)
            .flags(gio::SettingsBindFlags::DEFAULT)
            .build();
    }

    #[allow(dead_code)]
    pub fn create_action(&self, key: Key) -> gio::Action {
        self.settings.create_action(key.to_string().as_str())
    }

    #[allow(dead_code)]
    pub fn string(&self, key: Key) -> String {
        self.settings.string(&key.to_string()).to_string()
    }

    #[allow(dead_code)]
    pub fn set_string(&self, key: Key, value: String) {
        self.settings.set_string(&key.to_string(), &value).unwrap();
    }

    #[allow(dead_code)]
    pub fn boolean(&self, key: Key) -> bool {
        self.settings.boolean(&key.to_string())
    }

    #[allow(dead_code)]
    pub fn set_boolean(&self, key: Key, value: bool) {
        self.settings.set_boolean(&key.to_string(), value).unwrap();
    }

    #[allow(dead_code)]
    pub fn integer(&self, key: Key) -> i32 {
        self.settings.int(&key.to_string())
    }

    #[allow(dead_code)]
    pub fn set_integer(&self, key: Key, value: i32) {
        self.settings.set_int(&key.to_string(), value).unwrap();
    }

    #[allow(dead_code)]
    pub fn double(&self, key: Key) -> f64 {
        self.settings.double(&key.to_string())
    }

    #[allow(dead_code)]
    pub fn set_double(&self, key: Key, value: f64) {
        self.settings.set_double(&key.to_string(), value).unwrap();
    }
}

impl Default for SettingsManager {
    fn default() -> Self {
        Self::new()
    }
}
