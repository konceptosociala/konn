use gtk::{prelude::{GtkWindowExt, ImageExt, StyleContextExt, WidgetExt}, IconSize};

pub trait KonnWidgetExt {
    fn set_request_size(&self, width_height: (i32, i32));

    fn set_classes(&self, classes: &[&str]);
}

impl<W: WidgetExt> KonnWidgetExt for W {
    fn set_request_size(&self, width_height: (i32, i32)) {
        self.set_size_request(width_height.0, width_height.1);
    }
    
    fn set_classes(&self, classes: &[&str]) {
        let ctx = self.style_context();

        for class in classes {
            ctx.add_class(class);
        }
    }
}

pub trait KonnWindowExt {
    fn set_size(&self, width_height: (i32, i32));
}

impl<W: GtkWindowExt> KonnWindowExt for W {
    fn set_size(&self, width_height: (i32, i32)) {
        self.set_default_size(width_height.0, width_height.1);
    }
}

pub trait KonnImageExt {
    fn set_icon(&self, name_size: (&str, IconSize));
}

impl<I: ImageExt> KonnImageExt for I {
    fn set_icon(&self, name_size: (&str, IconSize)) {
        self.set_from_icon_name(Some(name_size.0), name_size.1);
    }
}