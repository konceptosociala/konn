use gtk::prelude::{ButtonExt, HeaderBarExt, LabelExt, NotebookExt, OrientableExt, WidgetExt};
use gtk::{Orientation, Align};
use relm::Widget;
use relm_derive::{widget, Msg};

use crate::utils::traits::KonnWidgetExt;

use HeaderMsg::*;

#[derive(Msg)]
pub enum HeaderMsg {
    Add,
    Remove,
}

#[widget]
impl Widget for Header {
    fn model() {}

    fn update(&mut self, event: HeaderMsg) {
        match event {
            Add => println!("Add"),
            Remove => println!("Remove"),
        }
    }

    view! {
        #[name="titlebar"]
        gtk::HeaderBar {
            show_close_button: true,

            gtk::Box {
                orientation: Orientation::Horizontal,

                gtk::Notebook {
                    scrollable: true,
                    valign: Align::End,
                    halign: Align::End,
                    show_border: false,
                    request_size: (20, 1),
    
                    switch_page(_, _, _) => println!("Page is switched"),
                    page_removed(_, _, _) => println!("Page is removed"),
                }
            }
        }
    }
}