// Morse - window.rs
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

use std::cell::{RefCell, OnceCell};
use std::time::{Duration, Instant};

use rand::prelude::*;

use crate::constants::{
    ALPHABETS,
    START_TEXT,
    START_TEXT_COMPETITIONS_LETTERS,
    START_TEXT_COMPETITIONS_DIGITS,
    END_TEXT,
    DEFAULT_TEXT,
    WORD_JOINER,
    ZERO_WIDTH_NO_JOINER
};
use crate::i18n::i18n;
use crate::application::MorseApplication;
use crate::widgets::volume_control::MorseVolumeControl;
use crate::backend::text_generator::generate_text;
use crate::backend::settings::{Key, settings_manager::SettingsManager};
use morse_player::{TextType, WaveType, Alphabet};

use adw::{
    ActionRow, 
    ComboRow, 
    OverlaySplitView, 
    PreferencesGroup, 
    SpinRow, 
    SwitchRow, 
    Toast, 
    ToastOverlay,
    StyleManager,
    subclass::prelude::*,
    prelude::*,
};
use gtk::{
    gio,
    glib,
    Label,
    Button,
    Popover,
    TextBuffer,
    TextView,
    TextTag,
    Grid,
    CheckButton,
    gdk::Display,
};

use gio::{SimpleAction};
use glib::{clone, ControlFlow, MainContext, SourceId};

mod imp {
    use morse_player::MorsePlayer;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/teacond/Morse/ui/window.ui")]
    pub struct MorseApplicationWindow {
        #[template_child]
        toast_overlay: TemplateChild<ToastOverlay>,
        #[template_child]
        split_view: TemplateChild<OverlaySplitView>,
        #[template_child]
        play_button: TemplateChild<Button>,
        #[template_child]
        stop_button: TemplateChild<Button>,
        #[template_child]
        timer_label: TemplateChild<Label>,
        #[template_child]
        preferences_group_general: TemplateChild<PreferencesGroup>,
        #[template_child]
        preferences_group_text: TemplateChild<PreferencesGroup>,
        #[template_child]
        preferences_group_speed: TemplateChild<PreferencesGroup>,
        #[template_child]
        text_type_combo: TemplateChild<ComboRow>,
        #[template_child]
        random_switch: TemplateChild<SwitchRow>,
        #[template_child]
        groups_switch: TemplateChild<SwitchRow>,
        #[template_child]
        groups_spin: TemplateChild<SpinRow>,
        #[template_child]
        characters_row: TemplateChild<ActionRow>,
        #[template_child]
        speed_spin: TemplateChild<SpinRow>,
        #[template_child]
        delay_spin: TemplateChild<SpinRow>,
        #[template_child]
        sidebar_back_button: TemplateChild<Button>,
        #[template_child]
        show_sidebar_button: TemplateChild<Button>,
        #[template_child]
        characters_popover: TemplateChild<Popover>,
        #[template_child]
        popover_grid: TemplateChild<Grid>,
        #[template_child]
        remove_all_button: TemplateChild<Button>,
        #[template_child]
        add_all_button: TemplateChild<Button>,
        #[template_child]
        text_view: TemplateChild<TextView>,
        #[template_child]
        text_buffer: TemplateChild<TextBuffer>,
        #[template_child]
        copy_text_button: TemplateChild<Button>,
        #[template_child]
        generate_text_button: TemplateChild<Button>,
        #[template_child]
        volume_control: TemplateChild<MorseVolumeControl>,
        pref_action: OnceCell<SimpleAction>,
        timeouts_vec: OnceCell<RefCell<Vec<SourceId>>>,
        check_buttons_vec: OnceCell<RefCell<Vec<CheckButton>>>,
        settings_manager: OnceCell<RefCell<SettingsManager>>,
        player: OnceCell<RefCell<MorsePlayer>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MorseApplicationWindow {
        const NAME: &'static str = "MorseApplicationWindow";
        type Type = super::MorseApplicationWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.install_action("win.toggle-playback", None, |win, _, _| {
                let this = &win.imp();
                if this.play_button.is_visible() {
                    this.play_button.emit_clicked();
                }
                else {
                    this.stop_button.emit_clicked();
                }
            });
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            MorseVolumeControl::ensure_type();
            obj.init_template();
        }
    }

    impl ObjectImpl for MorseApplicationWindow {
        fn constructed(&self) {
            self.parent_constructed();

            // Setting variables
            let word_length: u64 = 5;
            let style_manager = StyleManager::default(); // just a style manager
            let text_tag = TextTag::builder() // highlighting playable char
            .foreground_rgba(&style_manager.accent_color_rgba())
            .name("accent_tag")
            .build();
            self.pref_action.set( // setting a variable of preferences action (will be used to block this action)
                MorseApplication::default()
                .lookup_action("preferences")
                .unwrap()
                .downcast_ref::<gio::SimpleAction>()
                .unwrap()
                .clone()
            ).unwrap();

            self.timeouts_vec.set(RefCell::new(Vec::new())).unwrap();
            self.check_buttons_vec.set(RefCell::new(Vec::new())).unwrap();
            self.settings_manager.set(RefCell::new(MorseApplication::default().settings_manager())).unwrap();
            self.player.set(RefCell::new(MorseApplication::default().player())).unwrap();

            // Other interface settings
            self.text_buffer.set_text(DEFAULT_TEXT);
            self.text_buffer.tag_table().add(&text_tag);
            self.set_check_buttons_grid();
            self.characters_popover.set_parent(&self.characters_row.get());

            // Binding property
            self.settings_manager.get().unwrap().borrow().bind_property::<MorseVolumeControl>(
                Key::PlaybackVolume,
                self.volume_control.as_ref(),
                "volume"
            );

            self.settings_manager.get().unwrap().borrow().connect_changed(Key::Alphabet, clone!(
                #[weak(rename_to = this)] self,
                move |_, _| {
                    this.set_check_buttons_grid();
                }
            ));

            self.play_button.connect_clicked(clone!(
                #[weak(rename_to = this)] self,
                move |play_button| {
                    if this.random_switch.is_active() {
                        this.generate_text_clicked(word_length);
                        if this.check_buttons_vec.get().unwrap().borrow().is_empty() {
                            return;
                        }
                    }

                    let frequency = this.settings_manager.get().unwrap().borrow().integer(Key::Frequency) as f32;
                    let start_delay = this.settings_manager.get().unwrap().borrow().integer(Key::StartDelay);
                    let mut start_text_duration = Duration::from_secs(0);
                    let mut end_text_duration = Duration::from_secs(0);
                    let speed = this.speed_spin.value() as u32;
                    let delay = this.delay_spin.value() as u32;
                    let start_iter = this.text_buffer.start_iter();
                    let end_iter = this.text_buffer.end_iter();
                    let text_buffer_string = this.text_buffer.text(&start_iter, &end_iter, true).to_uppercase();
                    let allowed_chars = this.get_allowed_chars();
                    let base_text: String = text_buffer_string.chars().filter(|c| allowed_chars.contains(c)).collect();
                    let wave_type = match this.settings_manager.get().unwrap().borrow().integer(Key::WaveType) {
                        0 => WaveType::Square,
                        1 => WaveType::Triangle,
                        2 => WaveType::Sawtooth,
                        _ => WaveType::Sine,
                    };
                    let text_type: TextType = match this.text_type_combo.selected() {
                        0 => TextType::Letters,
                        1 => TextType::Digits,
                        _ => TextType::Mixed,
                    };
                    let text: String = match this.settings_manager.get().unwrap().borrow().integer(Key::Additions) {
                        0 => base_text.clone(),
                        1 => {
                            start_text_duration = this.player.get().unwrap().borrow().timings(START_TEXT, text_type, speed, delay).0;
                            end_text_duration = this.player.get().unwrap().borrow().timings(END_TEXT, text_type, speed, delay).0;
                            START_TEXT.to_string() + &base_text + &END_TEXT
                        }
                        _ => {
                            let mut start_string: String;
                            if text_type == TextType::Letters {
                                start_string = START_TEXT_COMPETITIONS_LETTERS.to_string();
                            }
                            else {
                                start_string = START_TEXT_COMPETITIONS_DIGITS.to_string();
                            }
                            start_string += &(speed.to_string() + " " + START_TEXT);
                            start_text_duration = this.player.get().unwrap().borrow().timings(
                                &start_string,
                                text_type,
                                speed,
                                delay,
                            ).0;
                            end_text_duration = this.player.get().unwrap().borrow().timings(
                                END_TEXT,
                                text_type,
                                speed,
                                delay,
                            ).0;
                            start_string + &base_text + &END_TEXT
                        },
                    };

                    let (base_duration, timings) = this.player.get().unwrap().borrow().timings(&base_text, text_type, speed, delay);

                    if base_text.is_empty() {
                        this.toast_overlay.add_toast(
                            Toast::builder()
                            .title(&i18n("The text doesn't contain any allowed characters"))
                            .build()
                        );
                        return
                    }

                    // Interface changes
                    this.player.get().unwrap().borrow().set_volume(this.volume_control.volume() as f32);
                    this.set_timer_label(base_duration);
                    play_button.set_visible(false);
                    this.stop_button.set_visible(true);
                    this.generate_text_button.set_sensitive(false);
                    this.preferences_group_general.set_sensitive(false);
                    this.preferences_group_text.set_sensitive(false);
                    this.preferences_group_speed.set_sensitive(false);
                    this.text_view.set_editable(false);
                    this.pref_action.get().unwrap().set_enabled(false);

                    let start_delay_timeout = glib::timeout_add_local_once(Duration::from_secs(start_delay as u64), clone!(
                        #[weak] this,
                        move || {
                            this.player.get().unwrap().borrow().play(&text, text_type, speed, delay, frequency, wave_type, 48000);
                            let text_start_instant = Instant::now();
                                
                            let timer_timeout = glib::timeout_add_local(Duration::from_millis(250), clone!(
                                #[strong] this,
                                move || {
                                    if text_start_instant.elapsed().as_secs_f32() - start_text_duration.as_secs_f32() >= 0.0 {
                                        let elapsed = text_start_instant.elapsed() - start_text_duration;
                                        if elapsed >= base_duration {
                                            return ControlFlow::Break;
                                        }
                                        this.set_timer_label(base_duration - elapsed);
                                    }
                                    return ControlFlow::Continue;
                                }
                            ));

                            let text_timeout = glib::timeout_add_local_once(base_duration + start_text_duration + end_text_duration, clone!(
                                #[weak] this,
                                move || {
                                    this.set_default_state();
                                }
                            ));

                            let mut ids_for_iter: Vec<i32> = Vec::new();
                            for (i, text_buffer_char) in text_buffer_string.chars().into_iter().enumerate() {
                                if allowed_chars.contains(&text_buffer_char) {
                                    ids_for_iter.push(i as i32);
                                }
                            }

                            let mut timeouts_vec_borrowed = this.timeouts_vec.get().unwrap().borrow_mut();
                            let mut char_id: i32 = 0;
                            for (i, timing) in timings.iter().enumerate() {
                                if i < ids_for_iter.len() {
                                    char_id = ids_for_iter[i];
                                }
                                let char_timeout = glib::timeout_add_local_once(
                                    start_text_duration + *timing + text_start_instant.elapsed(),
                                    clone!(
                                        #[weak] this,
                                        move || {
                                            this.text_buffer.remove_all_tags(&start_iter, &end_iter);
                                            this.text_buffer.apply_tag_by_name(
                                                "accent_tag",
                                                &this.text_buffer.iter_at_offset(char_id),
                                                &this.text_buffer.iter_at_offset(char_id + 1),
                                            );
                                        }
                                    )
                                );
                                timeouts_vec_borrowed.push(char_timeout);
                            }
                                
                            timeouts_vec_borrowed.push(text_timeout);
                            timeouts_vec_borrowed.push(timer_timeout);
                        }
                    ));

                    this.timeouts_vec.get().unwrap().borrow_mut().push(start_delay_timeout);
                }
            ));

            self.stop_button.connect_clicked(clone!(
                #[weak(rename_to = this)] self,
                move |_| {
                    this.player.get().unwrap().borrow().stop();
                    this.set_default_state();
                }
            ));

            self.text_type_combo.connect_selected_notify(clone!(
                #[weak(rename_to = this)] self,
                move |_| {
                    this.set_check_buttons_grid();
                }
            ));

            self.groups_switch.connect_active_notify(clone!(
                #[weak(rename_to = this)] self,
                move |groups_switch| {
                    this.groups_spin.set_sensitive(!groups_switch.is_active());
                    this.groups_spin.set_value(this.speed_spin.value() / 5.0);
                }
            ));

            self.sidebar_back_button.connect_clicked(clone!(
                #[weak(rename_to = this)] self,
                move |_| {
                    this.split_view.set_show_sidebar(false);
                }
            ));

            self.show_sidebar_button.connect_clicked(clone!(
                #[weak(rename_to = this)] self,
                move |_| {
                    this.split_view.set_show_sidebar(true);
                }
            ));

            self.characters_row.connect_activated(clone!(
                #[weak(rename_to = this)] self,
                move |_| {
                    this.characters_popover.popup();
                }
            ));

            self.remove_all_button.connect_clicked(clone!(
                #[weak(rename_to = this)] self,
                move |_| {
                    for check_button in this.check_buttons_vec.get().unwrap().borrow().to_vec() {
                        check_button.set_active(false);
                    }
                }
            ));

            self.add_all_button.connect_clicked(clone!(
                #[weak(rename_to = this)] self,
                move |_| {
                    for check_button in this.check_buttons_vec.get().unwrap().borrow().to_vec() {
                        check_button.set_active(true);
                    }
                }
            ));

            self.speed_spin.connect_value_notify(clone!(
                #[weak(rename_to = this)] self,
                move |speed_spin| {
                    if this.groups_switch.is_active() {
                        this.groups_spin.set_value(speed_spin.value() / 5.0);
                    }
                }
            ));

            self.volume_control.connect_volume_notify(clone!(
                #[weak(rename_to = this)] self,
                move |volume_control| {
                    this.player.get().unwrap().borrow().set_volume(volume_control.volume() as f32);
                }
            ));

            self.copy_text_button.connect_clicked(clone!(
                #[weak(rename_to = this)] self,
                move |_| {
                    let text_view_buffer = this.text_view.buffer();
                    let bounds = text_view_buffer.bounds();
                    let text = text_view_buffer.text(&bounds.0, &bounds.1, false);
                    let mut result = String::new();

                    let mut space_count = 0;

                    for c in text.chars() {
                        if c == ' ' {
                            space_count += 1;
                            if space_count % word_length == 0 {
                                result.push('\n');
                            } 
                            else {
                                result.push(' ');
                            }
                        }
                        else {
                            result.push(c);
                        }
                    }

                    Display::default().unwrap().clipboard().set_text(&result);
                    this.toast_overlay.add_toast(Toast::builder().title(&i18n("Copied to clipboard")).build());
                }
            ));

            self.generate_text_button.connect_clicked(clone!(
                #[weak(rename_to = this)] self,
                move |_| {
                    this.generate_text_clicked(word_length);
                }
            ));

            self.text_buffer.connect_text_notify(clone!(
                #[weak(rename_to = this)] self,
                move |text_buffer| {
                    if text_buffer.text(&text_buffer.start_iter(), &text_buffer.end_iter(), false).len() == 0 {
                        this.play_button.set_sensitive(false);
                    }
                    else {
                        this.play_button.set_sensitive(true);
                    }
                }
            ));

            // To change color of tag dependings on the accent color
            style_manager.connect_accent_color_rgba_notify(clone!(
                #[weak] text_tag,
                move |style_manager| {
                    text_tag.set_foreground_rgba(Some(&style_manager.accent_color_rgba()));
                }
            ));
        }
    }
    impl WidgetImpl for MorseApplicationWindow {}
    impl WindowImpl for MorseApplicationWindow {}
    impl ApplicationWindowImpl for MorseApplicationWindow {}
    impl AdwApplicationWindowImpl for MorseApplicationWindow {}

    impl MorseApplicationWindow {
        fn generate_text_clicked(&self, word_length: u64) {
            let chars_in_use = self.get_enabled_chars();
            if chars_in_use.is_empty() {
                self.toast_overlay.add_toast(
                    Toast::builder()
                    .title(&i18n("To generate text enable at least 1 character"))
                    .build()
                );
            }
            else {
                self.set_text_buffer(
                    &generate_text(
                        Some(rand::rng().random_range(0..u64::MAX)),
                        self.groups_spin.value() as u64 * word_length,
                        word_length,
                        chars_in_use.to_vec()
                    ).unwrap()
                );
            }
        }

        fn get_allowed_chars(&self) -> Vec<char> {
            let (mut allowed_chars, alphabet) = self.get_selected_alphabet();
            allowed_chars.extend(ALPHABETS.get("digits").unwrap().clone());
            allowed_chars.extend(ALPHABETS.get("symbols").unwrap().clone());
            allowed_chars.extend(ALPHABETS.get("other").unwrap().clone());
            if alphabet != Alphabet::Latin {
                allowed_chars.extend(ALPHABETS.get(&Alphabet::Latin.to_string()).unwrap().clone())
            }
            allowed_chars
        }

        fn get_selected_alphabet(&self) -> (Vec<char>, Alphabet) {
            match self.settings_manager.get().unwrap().borrow().integer(Key::Alphabet) {
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
            }
        }

        fn set_check_buttons_grid(&self) {
            let (base_alphabet, alphabet_type) = self.get_selected_alphabet();

            let chars: Vec<char> = match self.text_type_combo.selected() {
                0 => base_alphabet,
                1 => ALPHABETS.get("digits").unwrap().clone(),
                _ => {
                    let mut mixed: Vec<char> = base_alphabet;
                    mixed.extend(ALPHABETS.get("digits").unwrap().clone().clone());
                    mixed.extend(ALPHABETS.get("symbols").unwrap().clone().clone());
                    mixed
                },
            };

            let mut check_buttons_vec = self.check_buttons_vec.get().unwrap().borrow_mut();
            for check_button in check_buttons_vec.iter() {
                check_button.unparent();
            }
            check_buttons_vec.clear();
            self.player.get().unwrap().borrow().set_alphabet(alphabet_type);

            for (i, char) in chars.iter().enumerate() {
                let check_button = CheckButton::builder()
                .label(char.to_string())
                .active(true)
                .build();
                check_button.set_active(true);

                if chars.len() <= 10 {
                    self.popover_grid.attach(
                        &check_button,
                        (i / 2) as i32,
                        (i % 2) as i32,
                        1,
                        1
                    )
                }
                else {
                    self.popover_grid.attach(
                        &check_button,
                        (i / 6) as i32,
                        (i % 6) as i32,
                        1,
                        1
                    )
                }

                check_buttons_vec.push(check_button);
            }
        }

        fn get_enabled_chars(&self) -> Vec<char> {
            let mut enabled_chars: Vec<char> = Vec::new();
            for check_button in self.check_buttons_vec.get().unwrap().borrow().to_vec() {
                if check_button.is_active() {
                    enabled_chars.push(check_button.label().unwrap().chars().next().unwrap());
                }
            }
            enabled_chars
        }

        fn set_default_state(&self) {
            self.play_button.set_visible(true);
            self.stop_button.set_visible(false);
            self.generate_text_button.set_sensitive(true);
            self.preferences_group_general.set_sensitive(true);
            self.preferences_group_text.set_sensitive(true);
            self.preferences_group_speed.set_sensitive(true);
            self.text_view.set_editable(true);
            self.pref_action.get().unwrap().set_enabled(true);
            self.timer_label.set_text("00:00");
            self.text_buffer.remove_all_tags(&self.text_buffer.start_iter(), &self.text_buffer.end_iter());

            let context = MainContext::default();
            for id in self.timeouts_vec.get().unwrap().borrow_mut().drain(..) {
                if let Some(_) = context.find_source_by_id(&id) {
                    id.remove();
                }
            }
        }

        fn set_timer_label(&self, time: Duration) {
            let secs = time.as_secs();
            fn get_formated_time(secs: u64) -> String {
                let mut time_text = String::new();
                if secs < 10 {
                    time_text += "0";
                    time_text += &secs.to_string();
                }
                else {
                    time_text += &secs.to_string();
                }
                time_text
            }
            
            let mut time_string = String::new();
            let hours = secs / 3600;
            let minutes = (secs - (hours * 3600)) / 60;
            let seconds = secs - hours * 3600 - minutes * 60;
            if hours > 0 {
                time_string += &get_formated_time(hours);
                time_string += ":";
            }
            time_string += &get_formated_time(minutes);
            if hours == 0 {
                time_string += ":";
                time_string += &get_formated_time(seconds);
            }
            self.timer_label.set_text(&time_string);
        }

        fn set_text_buffer(&self, input_string: &str) {
            let punctuation: [char; 5] = ['.', ',', '/', '=', '?'];
            let mut output = String::new();
            let chars_vec: Vec<char> = input_string.chars().collect();
            let chars_vec_len = chars_vec.len();
            for i in 0..chars_vec_len {
                let c = chars_vec[i];
        
                if punctuation.contains(&c) {
                    if i > 0 && chars_vec[i - 1] != ' ' {
                        output.push(WORD_JOINER);
                    }
                    else {
                        output.push(ZERO_WIDTH_NO_JOINER);
                    }
                    
                    output.push(c);

                    if i + 1 < chars_vec_len && chars_vec[i + 1] != ' ' {
                        output.push(WORD_JOINER);
                    }
                    else {
                        output.push(ZERO_WIDTH_NO_JOINER);
                    }
                }
                else {
                    output.push(c);
                }
            }
            self.text_buffer.set_text(&output);
        }
    }
}

glib::wrapper! {
    pub struct MorseApplicationWindow(ObjectSubclass<imp::MorseApplicationWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl MorseApplicationWindow {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }
}
