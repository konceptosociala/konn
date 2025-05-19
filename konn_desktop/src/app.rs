use gtk::prelude::{GtkWindowExt, OrientableExt, WidgetExt};
use gtk::{Inhibit, Orientation};
use relm::Widget;
use relm_derive::{Msg, widget};

use crate::widgets::header::Header;
use crate::widgets::tab::Tab;
use crate::utils::traits::KonnWindowExt;

use AppMsg::*;

#[cfg(target_os = "macos")]
pub const UI_THEME: &[u8] = include_bytes!("../themes/macos/gtk.gresource");

#[cfg(target_os = "windows")]
pub const UI_THEME: &[u8] = include_bytes!("../themes/windows/gtk.gresource");

pub struct AppModel {
    
}

#[derive(Msg)]
pub enum AppMsg {
    Quit,
}

#[widget]
impl Widget for App {
    fn model() -> AppModel {
        #[cfg(any(target_os = "macos", target_os = "windows"))]
        App::apply_theme();
        
        AppModel { }
    }

    fn update(&mut self, event: AppMsg) {
        match event {
            Quit => App::quit(),
        }
    }

    view! {
        #[name="window"]
        gtk::Window {
            titlebar: view! {
                Header {

                }
            },
            size: (1024, 576),

            gtk::Box {
                orientation: Orientation::Vertical,

                Tab {

                }
            },

            delete_event(_, _) => (Quit, Inhibit(false)),
        }
    }
}

impl App {
    pub fn quit() {
        gtk::main_quit();
    }

    #[cfg(any(target_os = "macos", target_os = "windows"))]
    pub fn apply_theme() {
        use gtk::prelude::CssProviderExt as _;

        gio::resources_register(&gio::Resource::from_data(&glib::Bytes::from_static(UI_THEME)).unwrap());

        let provider = gtk::CssProvider::new();
        provider.load_from_resource("/org/gnome/theme/gtk.css");

        gtk::StyleContext::add_provider_for_screen(&gdk::Screen::default().unwrap(), &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    }
}