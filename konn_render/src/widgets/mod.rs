pub mod button;

use std::fmt::{Debug, Display};

use crate::renderer::Canvas;

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pos {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct WidgetId(pub &'static str);

impl Display for WidgetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

pub trait Widget {
    fn position(&self) -> Pos;

    fn size(&self) -> Size;
    
    fn draw(&self, canvas: &mut Canvas);

    fn event(&mut self, event: Option<Event>);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Click,
    Hover,
}