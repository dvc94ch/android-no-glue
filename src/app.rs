use glutin::config::ConfigsFinder;
use glutin::context::ContextBuilder;
use glutin::surface::Surface;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

pub fn app_main() {
    simple_logger::init().ok();
    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("A fantastic window!");

    let confs = unsafe { ConfigsFinder::new().find(&*el).unwrap() };
    let conf = confs[0].clone();
    println!("Configeration chosen: {:?}", conf);

    let ctx = unsafe { ContextBuilder::new().build(&conf).unwrap() };
    let win = unsafe { Surface::build_window(&conf, &*el, wb).unwrap() };

    // On android the surface can only be created after the resume event
    // was received.
    let mut surf = None;
    el.run(move |event, _, control_flow| {
        println!("{:?}", event);
        *control_flow = ControlFlow::Wait;

        match event {
            Event::Resumed => {
                let surface = unsafe { Surface::new_from_existing_window(&conf, &win).unwrap() };
                unsafe {
                    ctx.make_current(&surface).unwrap();
                }
                gl::load_with(|s| ctx.get_proc_address(s).unwrap());
                surf = Some(surface)
            }
            Event::Suspended => surf = None,
            Event::LoopDestroyed => return,
            Event::MainEventsCleared => win.request_redraw(),
            Event::RedrawRequested(_) => {
                if let Some(surf) = &surf {
                    unsafe {
                        gl::ClearColor(1.0, 0.5, 0.7, 1.0);
                        gl::Clear(gl::COLOR_BUFFER_BIT);
                    }
                    surf.swap_buffers().unwrap();
                }
            }
            Event::WindowEvent { ref event, .. } => {
                let size = match event {
                    WindowEvent::ScaleFactorChanged {
                        new_inner_size: size,
                        ..
                    } => size,
                    WindowEvent::Resized(size) => size,
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                    _ => return,
                };
                if let Some(surf) = &surf {
                    ctx.update_after_resize();
                    surf.update_after_resize(*size);
                    unsafe {
                        gl::Viewport(0, 0, size.width as _, size.height as _);
                    }
                }
            }
            _ => (),
        }
    });
}
