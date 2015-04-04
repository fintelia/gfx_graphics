extern crate piston;
extern crate graphics;
extern crate sdl2_window;
extern crate gfx_device_gl;
extern crate gfx_graphics;

use std::path::Path;
use std::rc::Rc;
use std::cell::RefCell;
use piston::window::{ Window, WindowSettings, Size, OpenGLWindow };
use gfx_graphics::gfx::traits::*;
use gfx_graphics::{ Gfx2d, Texture, gfx, TextureSettings };
use sdl2_window::{ Sdl2Window, OpenGL };

fn main() {
    let mut window = Sdl2Window::new(
        OpenGL::_3_2,
        WindowSettings::new(
            "gfx_graphics: image_test".to_string(),
            Size { width: 300, height: 300 }
        )
        .exit_on_esc(true)
    );

    let mut device = gfx_device_gl::GlDevice::new(|s| window.get_proc_address(s));
    let size = window.size();
    let frame = gfx::Frame::new(size.width as u16, size.height as u16);
    let mut renderer = device.create_renderer();

    let rust_logo = Texture::from_path(&mut device,
                                       &Path::new("./assets/rust.png"),
                                       &TextureSettings::new()).unwrap();
    let mut g2d = Gfx2d::new(&mut device);
    let window = Rc::new(RefCell::new(window));
    for e in piston::events(window) {
        use piston::event::*;

        if let Some(_) = e.render_args() {
            use graphics::*;

            g2d.draw(&mut renderer, &frame, |c, g| {
                let transform = c.transform.trans(100.0, 100.0);

                clear([1.0; 4], g);
                Rectangle::new([1.0, 0.0, 0.0, 1.0])
                    .draw([0.0, 0.0, 100.0, 100.0],
                          &c.draw_state,
                          c.transform,
                          g);
                Rectangle::new([0.0, 1.0, 0.0, 0.3])
                    .draw([50.0, 50.0, 100.0, 100.0],
                          &c.draw_state,
                          c.transform,
                          g);
                image(&rust_logo, transform, g);
            });

            device.submit(renderer.as_buffer());
            renderer.reset();
        }

        if let Some(_) = e.after_render_args() {
            device.after_frame();
        }
    }
}
