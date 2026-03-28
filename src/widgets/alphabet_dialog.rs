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
use crate::backend::settings::Key;
use crate::constants::ALPHABETS;
use morse_player::{Alphabet, MorsePlayer};
use adw::{subclass::prelude::*};
use gtk::{prelude::*, glib, Grid, DropDown, Box, Label, Orientation, Align};
use std::{cell::{OnceCell, RefCell}};

mod imp {

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/teacond/Morse/ui/alphabet_dialog.ui")]
    pub struct MorseAlphabetDialog {
        #[template_child]
        alphabets_combo: TemplateChild<DropDown>,
        #[template_child]
        alphabet_grid: TemplateChild<Grid>,
        grid_items: OnceCell<RefCell<Vec<Box>>>,
        player: OnceCell<RefCell<MorsePlayer>>,
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

            let settings_manager = MorseApplication::default().settings_manager();

            self.grid_items.set(RefCell::new(Vec::new())).unwrap();
            self.player.set(RefCell::new(MorseApplication::default().player())).unwrap();

            self.alphabets_combo.connect_selected_notify(glib::clone!(
                #[weak(rename_to = this)] self,
                move |alphabets_combo| {
                    let (alphabet, alphabet_type) = match alphabets_combo.selected() {
                        1 => {
                            (ALPHABETS.get(&Alphabet::Cyrillic.to_string()).unwrap().clone(), Alphabet::Cyrillic)
                        },
                        2 => {
                            (ALPHABETS.get(&Alphabet::Greek.to_string()).unwrap().clone(), Alphabet::Greek)
                        },
                        3 => {
                            (ALPHABETS.get(&Alphabet::Hebrew.to_string()).unwrap().clone(), Alphabet::Hebrew)
                        },
                        4 => {
                            (ALPHABETS.get(&Alphabet::Arabic.to_string()).unwrap().clone(), Alphabet::Arabic)
                        },
                        5 => {
                            (ALPHABETS.get(&Alphabet::Persian.to_string()).unwrap().clone(), Alphabet::Persian)
                        },
                        6 => {
                            (ALPHABETS.get(&Alphabet::Korean.to_string()).unwrap().clone(), Alphabet::Korean)
                        },
                        _ => {
                            (ALPHABETS.get(&Alphabet::Latin.to_string()).unwrap().clone(), Alphabet::Latin)
                        }
                    };

                    this.player.get().unwrap().borrow().set_alphabet(alphabet_type);
                    this.construct_alphabet(alphabet);
                }
            ));
            
            self.construct_alphabet(ALPHABETS.get(&Alphabet::Latin.to_string()).unwrap().clone());
            self.player.get().unwrap().borrow().set_alphabet(Alphabet::Latin);
            self.alphabets_combo.set_selected(settings_manager.integer(Key::Alphabet) as u32);
        }
    }
    impl WidgetImpl for MorseAlphabetDialog {}
    impl AdwDialogImpl for MorseAlphabetDialog {}

    impl MorseAlphabetDialog {
        fn construct_alphabet(&self, chars: Vec<char>) {
            let mut grid_items = self.grid_items.get().unwrap().borrow_mut();

            for el in grid_items.iter() {
                el.unparent();
            }

            grid_items.clear();

            for (i, el) in chars.iter().enumerate() {
                let alphabet_item = Box::builder()
                .orientation(Orientation::Horizontal)
                .hexpand(true)
                .css_classes(["card"])
                .build();

                let char_widget = Label::builder()
                .label(&el.to_string())
                .margin_start(20)
                .margin_top(10)
                .margin_bottom(10)
                .build();
                let morse_widget = Label::builder()
                .label(self.player.get().unwrap().borrow().get_morse(el))
                .margin_end(20)
                .margin_top(10)
                .margin_bottom(10)
                .hexpand(true)
                .halign(Align::Center)
                .build();
                alphabet_item.append(&char_widget);
                alphabet_item.append(&morse_widget);
                
                self.alphabet_grid.attach(
                    &alphabet_item,
                    (i % 2) as i32,
                    (i / 2) as i32,
                    1,
                    1
                );

                grid_items.push(alphabet_item);
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
