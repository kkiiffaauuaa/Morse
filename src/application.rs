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

use crate::config::{APP_ID, PATH_ID, VERSION};
use crate::i18n::i18n;
use crate::widgets::preferences_dialog::MorsePreferencesDialog;
use crate::MorseApplicationWindow;
use crate::backend::settings::settings_manager::SettingsManager;

use morse_player::MorsePlayer;
use adw::{prelude::*, subclass::prelude::*};
use gtk::{gio, glib};

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct MorseApplication {
        pub player: MorsePlayer,
        pub settings_manager: SettingsManager
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MorseApplication {
        const NAME: &'static str = "MorseApplication";
        type Type = super::MorseApplication;
        type ParentType = adw::Application;

        fn new() -> Self { 
            Self {
                player: MorsePlayer::new(),
                settings_manager: SettingsManager::new()
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
            obj.set_accels_for_action("win.toggle-playback", &["<ctrl>p"]);
            obj.set_accels_for_action("volume.toggle-mute", &["<ctrl>m"]);
            obj.set_accels_for_action("volume.increase", &["<ctrl>plus"]);
            obj.set_accels_for_action("volume.decrease", &["<ctrl>minus"]);
        }
    }

    impl ApplicationImpl for MorseApplication {
        fn activate(&self) {
            let application = self.obj();
            let window = application.active_window().unwrap_or_else(|| {
                let window = MorseApplicationWindow::new(&*application);
                window.upcast()
            });

            window.present();
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
        self.add_action_entries([quit_action, about_action, preferences_action]);
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

    pub fn player(&self) -> MorsePlayer {
        self.imp().player.clone()
    }

    pub fn settings_manager(&self) -> SettingsManager {
        self.imp().settings_manager.clone()
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
