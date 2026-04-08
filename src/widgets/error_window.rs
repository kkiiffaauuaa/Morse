// Morse - error_window.rs
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

use gtk::{gio, glib, Label, Button, Application};
use adw::{subclass::prelude::*, prelude::*, ApplicationWindow};
use glib::clone;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/teacond/Morse/ui/error_window.ui")]
    pub struct MorseErrorWindow {
        #[template_child]
        pub error_title: TemplateChild<Label>,
        #[template_child]
        pub error_description: TemplateChild<Label>,
        #[template_child]
        close_btn: TemplateChild<Button>
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MorseErrorWindow {
        const NAME: &'static str = "MorseErrorWindow";
        type Type = super::MorseErrorWindow;
        type ParentType = ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MorseErrorWindow {
        fn constructed(&self) {
            self.parent_constructed();

            self.close_btn.connect_clicked(clone!(
                #[weak(rename_to = this)] self,
                move |_| {
                    this.obj().close();
                }
            ));
        }
    }

    impl WidgetImpl for MorseErrorWindow {}
    impl WindowImpl for MorseErrorWindow {}
    impl ApplicationWindowImpl for MorseErrorWindow {}
    impl AdwApplicationWindowImpl for MorseErrorWindow {}
}

glib::wrapper! {
    pub struct MorseErrorWindow(ObjectSubclass<imp::MorseErrorWindow>)
        @extends gtk::Window, gtk::Widget, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup, gtk::ShortcutManager, gtk::Accessible,
                    gtk::Buildable, gtk::ConstraintTarget, gtk::Root, gtk::Native;
}

impl MorseErrorWindow {
    pub fn new(title: &str, description: &str, application: &impl IsA<Application>) -> Self {
        let window: MorseErrorWindow = glib::Object::builder().build();
        window.set_title(Some(title));
        window.set_application(Some(application));
        window.imp().error_title.set_text(title);
        window.imp().error_description.set_text(description);
        window
    }
}
