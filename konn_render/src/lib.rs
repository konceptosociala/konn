pub mod renderer;
pub mod widgets;

use std::collections::{hash_map::Entry, HashMap};

use flatbox_core::logger::error;
use flatbox_render::{error::RenderError, hal::GlInitFunction};
use renderer::Renderer;
use widgets::{Event, Pos, Widget, WidgetId};

pub use tiny_skia::{Rect, Color};
pub use flatbox_render::renderer::WindowExtent;

pub struct Context {
    renderer: Renderer,
    widgets: HashMap<WidgetId, Box<dyn Widget>>,
}

impl Context {
    pub fn new(init_function: impl GlInitFunction) -> Context {
        Context {
            renderer: Renderer::new(init_function),
            widgets: HashMap::new(),
        }
    }

    pub fn add_widget(&mut self, id: WidgetId, widget: impl Widget + 'static) {
        if let Entry::Vacant(entry) = self.widgets.entry(id) {
            self.renderer.add_canvas(id, widget.size());
            entry.insert(Box::new(widget));
        } else {
            error!("Widget with id `{id}` already exists");
        }
    }

    pub fn window_extent(&self) -> WindowExtent {
        self.renderer.get_extent()
    }

    pub fn resize(&mut self, new_extent: WindowExtent) {
        self.renderer.resize(new_extent);
    }

    pub fn execute(&mut self, cursor_pos: Pos, clicked: bool) -> Result<(), RenderError> {
        self.renderer.clear();
        
        for (id, widget) in &mut self.widgets {
            let pos = widget.position();
            let size = widget.size();

            if cursor_pos.x <= size.width 
                && cursor_pos.x >= pos.x
                && cursor_pos.y <= size.height 
                && cursor_pos.y >= pos.y
            {
                if clicked {
                    widget.event(Some(Event::Click));
                } else {
                    widget.event(Some(Event::Hover));
                }
            } else {
                widget.event(None);
            }

            let canvas = self.renderer.get_canvas(id).unwrap_or_else(|| {
                panic!("Canvas for widget `{id}` not created");
            });
            widget.draw(canvas);

            self.renderer.render(id)?;
        }

        Ok(())
    }
}

// pub fn sample_texture(width: u32, height: u32) -> Pixmap {
//     let paint = Paint {
//         anti_alias: true,
//         shader: Shader::SolidColor(Color::from_rgba8(255, 0, 0, 255)),
//         ..Default::default()
//     };

//     let mut pixmap = Pixmap::new(width, height).unwrap();
//     pixmap.fill(Color::from_rgba8(255, 255, 255, 255));
//     pixmap.fill_rect(
//         Rect::from_xywh(300.0, 300.0, 500.0, 500.0).unwrap(),
//         &paint,
//         Transform::identity(),
//         None,
//     );

//     pixmap
// }
