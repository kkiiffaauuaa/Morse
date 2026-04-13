// Morse - alphabet_dialog.rs
// Copyright (C) 2026  Jaŭhien Lavonćjeŭ <jauhien.lavoncjeu@gmail.com>
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

use crate::application::MorseApplication;
use crate::backend::settings::{Key, settings_manager::SettingsManager};
use crate::constants::ALPHABETS;
use crate::i18n::i18n;
use morse_player::{Alphabet, MorsePlayer, TextType, SpeedSystem, WaveType};
use adw::{subclass::prelude::*, prelude::*, ToastOverlay, Toast};
use gtk::{glib, Grid, DropDown, Box, Label, Orientation, Align, Button};
use std::{cell::{OnceCell, RefCell, Cell}, rc::Rc};
use glib::clone;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/teacond/Morse/ui/alphabet_dialog.ui")]
    pub struct MorseAlphabetDialog {
        #[template_child]
        toast_overlay: TemplateChild<ToastOverlay>,
        #[template_child]
        alphabets_combo: TemplateChild<DropDown>,
        #[template_child]
        letters_grid: TemplateChild<Grid>,
        #[template_child]
        digits_grid: TemplateChild<Grid>,
        #[template_child]
        punctuation_grid: TemplateChild<Grid>,
        #[template_child]
        ns_punctuation_grid: TemplateChild<Grid>,
        grid_items: OnceCell<Rc<RefCell<Vec<Button>>>>,
        player: OnceCell<Rc<MorsePlayer>>,
        is_playing_global: OnceCell<Rc<Cell<bool>>>,
        is_playing_local: OnceCell<Rc<Cell<bool>>>,
        settings_manager: OnceCell<Rc<SettingsManager>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MorseAlphabetDialog {
        const NAME: &'static str = "MorseAlphabetDialog";
        type Type = super::MorseAlphabetDialog;
        type ParentType = adw::Dialog;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MorseAlphabetDialog {
        fn constructed(&self) {
            self.parent_constructed();

            self.grid_items.set(Rc::new(RefCell::new(Vec::new()))).unwrap();
            self.player.set(Rc::new(MorseApplication::default().player())).unwrap();
            self.is_playing_global.set(MorseApplication::default().get_is_playing()).unwrap();
            self.is_playing_local.set(Rc::new(Cell::new(false))).unwrap();
            self.settings_manager.set(Rc::new(MorseApplication::default().settings_manager())).unwrap();

            self.alphabets_combo.connect_selected_notify(glib::clone!(
                #[weak(rename_to = this)] self,
                move |alphabets_combo| {
                    let (alphabet, alphabet_type) = match alphabets_combo.selected() {
                        1 => (ALPHABETS.cyrillic.visible.clone(), Alphabet::Cyrillic),
                        2 => (ALPHABETS.greek.visible.clone(), Alphabet::Greek),
                        3 => (ALPHABETS.hebrew.visible.clone(), Alphabet::Hebrew),
                        4 => (ALPHABETS.arabic.visible.clone(), Alphabet::Arabic),
                        5 => (ALPHABETS.persian.visible.clone(), Alphabet::Persian),
                        6 => (ALPHABETS.korean.visible.clone(), Alphabet::Korean),
                        _ => (ALPHABETS.latin.visible.clone(), Alphabet::Latin)
                    };
                    this.player.get().unwrap().set_alphabet(alphabet_type);

                    this.construct_alphabet(
                        alphabet,
                        ALPHABETS.digits.visible.clone(),
                        ALPHABETS.symbols.visible.clone(),
                        ALPHABETS.symbols.supported.clone()
                    );
                }
            ));
            
            self.construct_alphabet(
                ALPHABETS.latin.visible.clone(),
                ALPHABETS.digits.visible.clone(),
                ALPHABETS.symbols.visible.clone(),
                ALPHABETS.symbols.supported.clone()
            );
            self.player.get().unwrap().set_alphabet(Alphabet::Latin);
            self.alphabets_combo.set_selected(self.settings_manager.get().unwrap().integer(Key::Alphabet) as u32);
    
            self.obj().connect_closed(clone!(
                #[weak(rename_to = this)] self,
                move |_| {
                    if this.is_playing_local.get().unwrap().get() {
                        this.player.get().unwrap().stop();
                    }
                }
            ));
        }
    }
    impl WidgetImpl for MorseAlphabetDialog {}
    impl AdwDialogImpl for MorseAlphabetDialog {}

    impl MorseAlphabetDialog {
        fn construct_alphabet(&self, letters: Vec<char>, digits: Vec<char>, punctuation: Vec<char>, ns_punctuation: Vec<char>) {
            let mut grid_items = self.grid_items.get().unwrap().borrow_mut();

            for el in grid_items.iter() {
                el.unparent();
            }

            grid_items.clear();

            for (grid, chars) in [
                (self.letters_grid.clone(), letters),
                (self.digits_grid.clone(), digits),
                (self.punctuation_grid.clone(), punctuation),
                (self.ns_punctuation_grid.clone(), ns_punctuation)
                ] {
                for (i, el) in chars.iter().enumerate() {
                    let char_str = el.to_string();

                    let alphabet_button_box = Box::builder()
                    .orientation(Orientation::Horizontal)
                    .build();

                    let alphabet_button = Button::builder()
                    .hexpand(true)
                    .css_classes(["card", "activatable"])
                    .child(&alphabet_button_box)
                    .build();

                    let char_widget = Label::builder()
                    .label(&char_str)
                    .margin_start(20)
                    .margin_top(10)
                    .margin_bottom(10)
                    .build();

                    let morse_widget = Label::builder()
                    .label(self.player.get().unwrap().get_morse(el))
                    .margin_end(20)
                    .margin_top(10)
                    .margin_bottom(10)
                    .hexpand(true)
                    .halign(Align::Center)
                    .build();

                    alphabet_button_box.append(&char_widget);
                    alphabet_button_box.append(&morse_widget);
                    
                    alphabet_button.connect_clicked(clone!(
                        #[weak(rename_to = this)] self,
                        move |_| {
                            if this.is_playing_global.get().unwrap().get() {
                                this.toast_overlay.add_toast(
                                    Toast::builder()
                                    .title(&i18n("Unable to play this character while the audio is playing"))
                                    .build()
                                );
                            }
                            else if this.is_playing_local.get().unwrap().get() { }
                            else {
                                this.is_playing_local.get().unwrap().set(true);

                                let (duration, _) = this.player.get().unwrap().timings(
                                    &char_str,
                                    TextType::Mixed,
                                    this.settings_manager.get().unwrap().integer(Key::DefaultSpeed) as u32,
                                    3
                                );
                                let frequency = this.settings_manager.get().unwrap().integer(Key::Frequency) as f32;
                                let wave_type = match this.settings_manager.get().unwrap().integer(Key::WaveType) {
                                    0 => WaveType::Square,
                                    1 => WaveType::Triangle,
                                    2 => WaveType::Sawtooth,
                                    _ => WaveType::Sine,
                                };
                                let speed_system: SpeedSystem = match this.settings_manager.get().unwrap().integer(Key::SpeedSystem) {
                                    0 => SpeedSystem::CODEX,
                                    _ => SpeedSystem::PARIS
                                };

                                this.player.get().unwrap().set_speed_system(speed_system);
                                this.player.get().unwrap().set_volume(this.settings_manager.get().unwrap().double(Key::PlaybackVolume) as f32);
                                this.player.get().unwrap().play(
                                    &char_str,
                                    TextType::Letters,
                                    this.settings_manager.get().unwrap().integer(Key::DefaultSpeed) as u32,
                                    3,
                                    frequency,
                                    wave_type,
                                    48000
                                );

                                glib::timeout_add_local_once(duration, clone!(
                                    #[weak] this,
                                    move || {
                                        this.is_playing_local.get().unwrap().set(false);
                                    }
                                ));
                            }
                        }
                    ));

                    grid.attach(
                        &alphabet_button,
                        (i % 2) as i32,
                        (i / 2) as i32,
                        1,
                        1
                    );

                    grid_items.push(alphabet_button);
                }
            }
        }
    }
}

glib::wrapper! {
    pub struct MorseAlphabetDialog(ObjectSubclass<imp::MorseAlphabetDialog>)
        @extends adw::Dialog, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl MorseAlphabetDialog {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
