// Morse - main.rs
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

#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

#[macro_use]
extern crate strum_macros;

#[cfg(target_os = "windows")]
use std::env;
#[cfg(target_os = "windows")]
use std::process::Command;

mod widgets;
mod backend;
mod application;
mod config;
mod constants;
mod i18n;

use self::application::MorseApplication;
use self::widgets::window::MorseApplicationWindow;

use config::{APP_ID, PKGNAME, PREFIX, LOCALEDIR, DATADIR};
use gettextrs::{bind_textdomain_codeset, bindtextdomain, textdomain};
use gtk::{gio, glib, prelude::*};

fn main() -> glib::ExitCode {
    #[cfg(target_os = "windows")] {
        if env::var("GDK_DISABLE").unwrap_or_default() != "dcomp" || env::var("GSK_RENDERER").unwrap_or_default() != "vulkan" {
            let current_exe = env::current_exe().expect("Failed to get current exe path");
            let status = Command::new(current_exe)
                .env("GDK_DISABLE", "dcomp")
                .env("GSK_RENDERER", "vulkan")
                .args(env::args().skip(1))
                .status()
                .expect("Failed to restart with GDK_DISABLE");
            std::process::exit(status.code().unwrap_or(0));
        }
    }
    
    // Set up gettext translations
    let localepath = get_prefix().join(LOCALEDIR);

    bindtextdomain(PKGNAME, localepath.to_str().unwrap()).expect("Unable to bind the text domain");
    bind_textdomain_codeset(PKGNAME, "UTF-8").expect("Unable to set the text domain encoding");
    textdomain(PKGNAME).expect("Unable to switch to the text domain");

    // Load resources
    let path = get_prefix().join(DATADIR).join(PKGNAME).join(format!("{}.gresource", APP_ID));
    let resources = gio::Resource::load(path.to_str().unwrap()).expect("Could not load resources");
    gio::resources_register(&resources);

    // Create a new GtkApplication. The application manages our main loop,
    // application windows, integration with the window manager/compositor, and
    // desktop features such as file opening and single-instance applications.
    let app = MorseApplication::new(APP_ID, &gio::ApplicationFlags::empty());

    // Run the application. This function will block until the application
    // exits. Upon return, we have our exit code to return to the shell. This
    // is the code you see when you do `echo $?` after running a command in a
    // terminal.
    app.run()
}

fn get_prefix() -> std::path::PathBuf {
    if cfg!(target_os = "windows") {
        let mut path = std::env::current_exe().unwrap();
        path.pop();
        path.pop();
        path
    }
    else {
        std::path::PathBuf::from(PREFIX)
    }
}
