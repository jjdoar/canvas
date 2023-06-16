use canvas::{render::WgpuCanvasRenderer, Canvas};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() {
    env_logger::init();

    let width = 400;
    let height = 400;
    let pixel_size = 3;

    let window_size = LogicalSize::new((width * pixel_size) as f64, (height * pixel_size) as f64);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(window_size)
        // .with_min_inner_size(min_window_size)
        .build(&event_loop)
        .unwrap();

    let mut canvas = Canvas::new(width, height);
    let mut renderer = WgpuCanvasRenderer::new(&window, &canvas).unwrap();

    for x in 0..width {
        canvas.set_pixel(x, x, &[255, 255, 255, 255]);
    }

    let mut minimized = false;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            window_id: _,
            event,
        } => match event {
            WindowEvent::CloseRequested => control_flow.set_exit(),
            WindowEvent::Resized(size) => {
                if size.width == 0 || size.height == 0 {
                    minimized = true;
                } else {
                    minimized = false;
                    renderer.resize_surface(size.width, size.height)
                }
            }
            WindowEvent::ScaleFactorChanged {
                scale_factor: _,
                new_inner_size: size,
            } => {
                if size.width == 0 || size.height == 0 {
                    minimized = true;
                } else {
                    minimized = false;
                    renderer.resize_surface(size.width, size.height)
                }
            }
            _ => {}
        },
        Event::MainEventsCleared => {
            if !minimized {
                window.request_redraw();
            }
        }
        Event::RedrawRequested(_) => {
            if !minimized {
                renderer.render(&canvas).unwrap();
            }
        }
        _ => {}
    });
}
