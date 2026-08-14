// Morse - application.rs
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

use crate::MorseApplicationWindow;
use crate::backend::settings::settings_manager::SettingsManager;
use crate::i18n::i18n;
use crate::widgets::alphabet_dialog::MorseAlphabetDialog;
use crate::widgets::error_window::MorseErrorWindow;
use crate::widgets::preferences_dialog::MorsePreferencesDialog;
use crate::{APP_ID, PATH_ID, VERSION};

use adw::{prelude::*, subclass::prelude::*};
use gtk::{gio, glib};
use morse_player::MorsePlayer;

use std::{cell::Cell, rc::Rc};

mod imp {
    use super::*;

    #[derive(Debug)]
    pub struct MorseApplication {
        pub player: Option<MorsePlayer>,
        pub is_playing: Rc<Cell<bool>>,
        pub settings_manager: SettingsManager,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MorseApplication {
        const NAME: &'static str = "MorseApplication";
        type Type = super::MorseApplication;
        type ParentType = adw::Application;

        fn new() -> Self {
            let player: Option<MorsePlayer> = MorsePlayer::new().ok();

            Self {
                player: player,
                is_playing: Rc::new(Cell::new(false)),
                settings_manager: SettingsManager::new(APP_ID),
            }
        }
    }

    impl ObjectImpl for MorseApplication {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_gactions();
            obj.set_accels_for_action("app.quit", &["<primary>q"]);
            obj.set_accels_for_action("app.preferences", &["<primary>comma"]);
            obj.set_accels_for_action("app.alphabet", &["<primary>i"]);
            obj.set_accels_for_action("win.toggle-playback", &["<primary>p"]);
            obj.set_accels_for_action("volume.toggle-mute", &["<primary>m"]);
            obj.set_accels_for_action("volume.increase", &["<primary>plus"]);
            obj.set_accels_for_action("volume.decrease", &["<primary>minus"]);
        }
    }

    impl ApplicationImpl for MorseApplication {
        fn activate(&self) {
            let application = self.obj();

            if self.player.is_some() {
                let window = application.active_window().unwrap_or_else(|| {
                    let window = MorseApplicationWindow::new(&*application);
                    window.upcast()
                });
                window.present();
            } else {
                let window = MorseErrorWindow::new(
                    &i18n("Audio Error"),
                    &i18n(
                        "Failed to create audio stream. Please check your audio output settings.",
                    ),
                    &*application,
                );
                window.present();
            }
        }
    }

    impl GtkApplicationImpl for MorseApplication {}
    impl AdwApplicationImpl for MorseApplication {}
}

glib::wrapper! {
    pub struct MorseApplication(ObjectSubclass<imp::MorseApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl MorseApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .property("resource-base-path", PATH_ID)
            .build()
    }

    fn setup_gactions(&self) {
        let quit_action = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| app.quit())
            .build();
        let about_action = gio::ActionEntry::builder("about")
            .activate(move |app: &Self, _, _| app.show_about())
            .build();
        let preferences_action = gio::ActionEntry::builder("preferences")
            .activate(move |app: &Self, _, _| app.show_preferences())
            .build();
        let alphabet_action = gio::ActionEntry::builder("alphabet")
            .activate(move |app: &Self, _, _| app.show_alphabet())
            .build();
        self.add_action_entries([
            quit_action,
            about_action,
            preferences_action,
            alphabet_action,
        ]);
    }

    fn show_about(&self) {
        let window = self.active_window().unwrap();
        let about = adw::AboutDialog::builder()
            .application_icon(APP_ID)
            .application_name(&i18n("Morse"))
            .developer_name("Jaŭhien Lavonćjeŭ")
            .version(VERSION)
            .developers(vec![
                "Jaŭhien Lavonćjeŭ <jauhien.lavoncjeu@gmail.com>",
                "Iñaki https://github.com/igonzalezb",
            ])
            .copyright("© 2025-2026 Jaŭhien Lavonćjeŭ")
            .issue_url("https://github.com/teacond/Morse/issues")
            .license_type(gtk::License::Gpl30)
            // # Translator: Leave there your name
            .translator_credits(&i18n("translator-credits"))
            .build();

        about.present(Some(&window));
    }

    fn show_preferences(&self) {
        let window = self.active_window().unwrap();
        MorsePreferencesDialog::new().present(Some(&window));
    }

    fn show_alphabet(&self) {
        let window = self.active_window().unwrap();
        MorseAlphabetDialog::new().present(Some(&window));
    }

    pub fn player(&self) -> MorsePlayer {
        self.imp().player.clone().unwrap()
    }

    pub fn settings_manager(&self) -> SettingsManager {
        self.imp().settings_manager.clone()
    }

    pub fn get_is_playing(&self) -> Rc<Cell<bool>> {
        self.imp().is_playing.clone()
    }
}

impl Default for MorseApplication {
    fn default() -> Self {
        gio::Application::default()
            .expect("Could not get default GApplication")
            .downcast()
            .unwrap()
    }
}
