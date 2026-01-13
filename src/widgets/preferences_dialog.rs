// Morse - preferences_dialog.rs
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

use crate::backend::settings::{Key, settings_manager};

use adw::{ComboRow, SpinRow, subclass::prelude::*};
use gtk::glib;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/teacond/Morse/ui/preferences_dialog.ui")]
    pub struct MorsePreferencesDialog {
        #[template_child]
        additions_combo: TemplateChild<ComboRow>,
        #[template_child]
        start_delay_spin: TemplateChild<SpinRow>,
        #[template_child]
        wave_type_combo: TemplateChild<ComboRow>,
        #[template_child]
        freq_spin: TemplateChild<SpinRow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MorsePreferencesDialog {
        const NAME: &'static str = "MorsePreferencesDialog";
        type Type = super::MorsePreferencesDialog;
        type ParentType = adw::PreferencesDialog;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MorsePreferencesDialog {
        fn constructed(&self) {
            self.parent_constructed();

            settings_manager::bind_property::<ComboRow>(
                Key::Additions,
                self.additions_combo.as_ref(),
                "selected"
            );

            settings_manager::bind_property::<SpinRow>(
                Key::StartDelay,
                self.start_delay_spin.as_ref(),
                "value"
            );

            settings_manager::bind_property::<ComboRow>(
                Key::WaveType,
                self.wave_type_combo.as_ref(),
                "selected"
            );

            settings_manager::bind_property::<SpinRow>(
                Key::Frequency,
                self.freq_spin.as_ref(),
                "value"
            );
        }
    }
    impl WidgetImpl for MorsePreferencesDialog {}
    impl PreferencesDialogImpl for MorsePreferencesDialog {}
    impl AdwDialogImpl for MorsePreferencesDialog {}
}

glib::wrapper! {
    pub struct MorsePreferencesDialog(ObjectSubclass<imp::MorsePreferencesDialog>)
        @extends adw::PreferencesDialog, adw::Dialog, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::ShortcutManager;
}

impl MorsePreferencesDialog {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
