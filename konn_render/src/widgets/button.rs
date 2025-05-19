use crate::{
    renderer::Canvas,
    Rect, Color,
};

use super::{Event, Pos, Size, Widget};

pub struct Button {
    position: Pos,
    size: Size,
    hover: bool,
    click: bool,
}

impl Button {
    pub fn new(position: Pos, size: Size) -> Button {
        Button { 
            position, 
            size,
            hover: false,
            click: false,
        }
    }
}

impl Widget for Button {
    fn position(&self) -> Pos { self.position }

    fn size(&self) -> Size { self.size }

    fn draw(&self, canvas: &mut Canvas) {
        canvas.clear(Color::from_rgba8(255, 255, 255, 255));
        
        let color = if self.click {
            Color::from_rgba8(255, 0, 0, 255)
        } else if self.hover {
            Color::from_rgba8(0, 255, 0, 255)
        } else {
            Color::from_rgba8(0, 0, 255, 255)
        };

        canvas.rect(Rect::from_xywh(0.0, 0.0, self.size.width as f32, 25.0).unwrap(), color);
        canvas.submit();
    }

    fn event(&mut self, event: Option<Event>) {
        match event {
            Some(Event::Click) => self.click = true,
            Some(Event::Hover) => self.hover = true,
            None => {
                self.click = false;
                self.hover = false;
            }
        }
    }
}