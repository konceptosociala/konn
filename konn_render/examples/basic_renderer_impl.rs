use flatbox_core::catch::CatchError;
use glutin::{
    dpi::{LogicalSize, PhysicalSize},
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::unix::WindowBuilderExtUnix,
    window::{Window, WindowBuilder},
    Api, ContextBuilder, ContextWrapper, GlRequest, PossiblyCurrent,
};
use konn_render::{widgets::{button::Button, Pos, Size, Widget, WidgetId}, Context as KonnRenderContext};
use konn_render::WindowExtent;

pub struct KonnRenderImpl {
    gl_ctx: ContextWrapper<PossiblyCurrent, Window>,
    ctx: KonnRenderContext,
    cursor_pos: Pos,
    clicked: bool,
}

impl KonnRenderImpl {
    pub fn new(event_loop: &EventLoop<()>) -> KonnRenderImpl {
        let window = WindowBuilder::new()
            .with_base_size(LogicalSize::new(800, 600))
            .with_title("Konn render to OpenGL example");

        let gl_ctx = unsafe {
            ContextBuilder::new()
                .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
                .build_windowed(window, event_loop)
                .expect("Cannot create window context")
                .make_current()
                .expect("Cannot create window context")
        };

        let ctx = KonnRenderContext::new(|ptr| gl_ctx.get_proc_address(ptr) as *const _);

        KonnRenderImpl { 
            gl_ctx, 
            ctx, 
            cursor_pos: Pos::default(),
            clicked: false,
        }
    }

    pub fn add_widget(&mut self, id: WidgetId, widget: impl Widget + 'static) {
        self.ctx.add_widget(id, widget);
    } 

    pub fn resize(&mut self, physical_size: PhysicalSize<u32>) {
        let w = physical_size.width as f32;
        let h = physical_size.height as f32;
        if w > h {
            self.ctx.resize(WindowExtent {
                x: 0.0,
                y: (h - w) / 2.0,
                width: w,
                height: w,
            });
        } else {
            self.ctx.resize(WindowExtent {
                x: (w - h) / 2.0,
                y: 0.0,
                width: h,
                height: h,
            });
        }

        self.gl_ctx.resize(physical_size);
    }

    pub fn execute(&mut self) {
        self.ctx.execute(self.cursor_pos, self.clicked)
            .catch();
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let mut konn_render = KonnRenderImpl::new(&event_loop);

    konn_render.add_widget(WidgetId("button1"), Button::new(
        Pos {
            x: 25,
            y: 25,
        }, 
        Size {
            width: 50,
            height: 50,
        }
    ));

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => (),
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => {
                    konn_render.resize(physical_size);
                },
                WindowEvent::CursorMoved { position, .. } => {
                    konn_render.cursor_pos = Pos {
                        x: position.x as u32,
                        y: position.y as u32,
                    };
                },
                WindowEvent::MouseInput { state, button, .. } => {
                    if button == MouseButton::Left {
                        konn_render.clicked = state == ElementState::Pressed;
                    }
                }
                _ => {},
            },
            Event::MainEventsCleared => {
                konn_render.gl_ctx.window().request_redraw();
            },
            Event::RedrawRequested(_) => {
                konn_render.execute();
                konn_render.gl_ctx.swap_buffers().unwrap();
            }
            _ => {},
        }
    });
}