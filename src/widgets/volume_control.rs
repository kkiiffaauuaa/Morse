// Morse - volume_control.rs
// Copyright (C) 2025  Jaŭhien Lavonćjeŭ <jauhien.lavoncjeu@gmail.com>
//               2024  Felix Häcker (Shortwave)
//               2022  Emmanuele Bassi (Original Author, Amberol)
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

use std::cell::Cell;
use std::marker::PhantomData;
use std::sync::LazyLock;

use glib::{Properties, subclass::Signal};
use gtk::{Button, Scale, Widget, gdk, gio, glib, prelude::*, subclass::prelude::*};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, Properties)]
    #[template(resource = "/io/github/teacond/Morse/ui/volume_control.ui")]
    #[properties(wrapper_type = super::MorseVolumeControl)]
    pub struct MorseVolumeControl {
        #[template_child]
        volume_low_button: TemplateChild<Button>,
        #[template_child]
        volume_scale: TemplateChild<Scale>,

        #[property(get = Self::volume, set = Self::set_volume, minimum = 0.0, maximum = 1.0, default = 1.0)]
        volume: PhantomData<f64>,
        #[property(get, set=Self::set_toggle_mute)]
        toggle_mute: Cell<bool>,

        prev_volume: Cell<f64>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MorseVolumeControl {
        const NAME: &'static str = "MorseVolumeControl";
        type Type = super::MorseVolumeControl;
        type ParentType = Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();

            klass.install_property_action("volume.toggle-mute", "toggle-mute");

            klass.install_action("volume.increase", None, |obj, _, _| {
                obj.set_volume((obj.volume() + 0.05).clamp(0.0, 1.0));
            });

            klass.install_action("volume.decrease", None, |obj, _, _| {
                obj.set_volume((obj.volume() - 0.05).clamp(0.0, 1.0));
            });
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for MorseVolumeControl {
        fn constructed(&self) {
            self.parent_constructed();

            self.volume_scale.adjustment().connect_notify_local(
                Some("value"),
                glib::clone!(
                    #[weak(rename_to = this)]
                    self,
                    move |adj, _| {
                        let value = adj.value();
                        if value == adj.lower() {
                            this.volume_low_button
                                .set_icon_name("audio-volume-muted-symbolic");
                        } else if value <= 0.333 {
                            this.volume_low_button
                                .set_icon_name("audio-volume-low-symbolic");
                        } else if value <= 0.666 {
                            this.volume_low_button
                                .set_icon_name("audio-volume-medium-symbolic");
                        } else {
                            this.volume_low_button
                                .set_icon_name("audio-volume-high-symbolic");
                        }
                        this.obj().notify_volume();
                        this.obj().emit_by_name::<()>("volume-changed", &[&value]);
                    }
                ),
            );

            let event_controller = gtk::EventControllerScroll::builder()
                .name("volume-scroll")
                .flags(gtk::EventControllerScrollFlags::VERTICAL)
                .build();

            event_controller.connect_scroll(glib::clone!(
                #[weak(rename_to = this)]
                self,
                #[upgrade_or_panic]
                move |_, _, dy| {
                    let adj = this.volume_scale.adjustment();
                    let delta = dy * adj.step_increment();
                    let d = (adj.value() - delta).clamp(adj.lower(), adj.upper());
                    adj.set_value(d);
                    glib::Propagation::Stop
                }
            ));
            self.volume_scale.add_controller(event_controller);

            let shortcut_controller = gtk::ShortcutController::new();
            shortcut_controller.set_scope(gtk::ShortcutScope::Global);

            shortcut_controller.add_shortcut(gtk::Shortcut::new(
                Some(gtk::KeyvalTrigger::new(
                    gdk::Key::m,
                    gdk::ModifierType::CONTROL_MASK,
                )),
                Some(gtk::NamedAction::new("volume.toggle-mute")),
            ));
            shortcut_controller.add_shortcut(gtk::Shortcut::new(
                Some(gtk::KeyvalTrigger::new(
                    gdk::Key::plus,
                    gdk::ModifierType::CONTROL_MASK,
                )),
                Some(gtk::NamedAction::new("volume.increase")),
            ));
            shortcut_controller.add_shortcut(gtk::Shortcut::new(
                Some(gtk::KeyvalTrigger::new(
                    gdk::Key::minus,
                    gdk::ModifierType::CONTROL_MASK,
                )),
                Some(gtk::NamedAction::new("volume.decrease")),
            ));
            self.obj().add_controller(shortcut_controller);
        }

        fn dispose(&self) {
            while let Some(child) = self.obj().first_child() {
                child.unparent();
            }
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: LazyLock<Vec<Signal>> = LazyLock::new(|| {
                vec![
                    Signal::builder("volume-changed")
                        .param_types([f64::static_type()])
                        .build(),
                ]
            });

            SIGNALS.as_ref()
        }
    }

    impl WidgetImpl for MorseVolumeControl {}

    impl MorseVolumeControl {
        fn set_toggle_mute(&self, muted: bool) {
            if muted != self.toggle_mute.replace(muted) {
                if muted {
                    let prev_value = self.volume_scale.value();
                    self.prev_volume.replace(prev_value);
                    self.volume_scale.set_value(0.0);
                } else {
                    let prev_value = self.prev_volume.get();
                    self.volume_scale.set_value(prev_value);
                }
                self.obj().notify_toggle_mute();
            }
        }

        pub fn volume(&self) -> f64 {
            self.volume_scale.value()
        }

        pub fn set_volume(&self, value: f64) {
            self.volume_scale.set_value(value);
        }
    }
}

glib::wrapper! {
    pub struct MorseVolumeControl(ObjectSubclass<imp::MorseVolumeControl>)
        @extends gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::ConstraintTarget, gtk::Accessible, gtk::Buildable;
}

impl MorseVolumeControl {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
