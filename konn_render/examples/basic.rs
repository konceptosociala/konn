use flatbox_render::renderer::WindowExtent;
use glutin::{
    dpi::LogicalSize, 
    event::{Event, WindowEvent}, 
    event_loop::{ControlFlow, EventLoop}, 
    platform::unix::WindowBuilderExtUnix, 
    window::WindowBuilder, 
    Api, ContextBuilder, GlRequest,
};
use konn_render::renderer::Renderer;

fn main() {
    let event_loop = EventLoop::new();
    let window = init_window("Konn render to OpenGL example", 800, 600);
    let ctx = init_ctx(window, &event_loop);
    let mut renderer = Renderer::new(|ptr| ctx.get_proc_address(ptr) as *const _, 800, 600);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
    
        match event {
            Event::LoopDestroyed => (),
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => {
                    renderer_resize(&mut renderer, physical_size);
                    ctx.resize(physical_size);
                },
                _ => (),
            },
            Event::RedrawRequested(_) => {
                let extent = renderer.get_extent();
                renderer.render(konn_render::sample_texture(extent.width as u32, extent.height as u32)).unwrap();
                ctx.swap_buffers().unwrap();
            },
            _ => (),
        }
    });
    

}

fn init_window(title: &str, width: u32, height: u32) -> WindowBuilder {
    WindowBuilder::new()
        .with_base_size(LogicalSize::new(width, height))
        .with_title(title)
}

fn init_ctx(window: WindowBuilder, event_loop: &EventLoop<()>) -> glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window> {
    let ctx = unsafe { ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .build_windowed(window, event_loop)
        .expect("Cannot create window context")
        .make_current()
        .expect("Cannot create window context")
    };
    ctx
}

fn renderer_resize(
    renderer: &mut Renderer,
    physical_size: glutin::dpi::PhysicalSize<u32>,
) {
    let w = physical_size.width as f32;
    let h = physical_size.height as f32;
    if w > h {
        renderer.resize(WindowExtent {
            x: 0.0, 
            y: (h - w) / 2.0, 
            width: w, 
            height: w,
        });
    } else {
        renderer.resize(WindowExtent {
            x: (w - h) / 2.0, 
            y: 0.0, 
            width: h, 
            height: h,
        });
    }
}