use gtk::prelude::{OrientableExt, WidgetExt};
use gtk::{IconSize, Orientation};
use relm::Widget;
use relm_derive::{widget, Msg};

use crate::utils::traits::{KonnImageExt, KonnWidgetExt};

#[derive(Msg)]
pub enum TabMsg {

}

#[widget]
impl Widget for Tab {
    fn model() {}

    fn update(&mut self, event: TabMsg) {
        match event {
            
        }
    }

    view! {
        gtk::Box {
            orientation: Orientation::Horizontal,
            hexpand: false,
            vexpand: false,

            gtk::Button {
                classes: &["titlebutton", "circular"],
                margin_start: 3,

                gtk::Image {
                    icon: ("window-close", IconSize::Menu),
                }
            }
        }
    }
}