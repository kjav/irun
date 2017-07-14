extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate sdl2_window;

use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use opengl_graphics::*;
use opengl_graphics::glyph_cache::GlyphCache;
use sdl2_window::Sdl2Window;

fn main() {
    let opengl = OpenGL::V2_1;
    let mut window: Sdl2Window = WindowSettings::new("opengl_graphics: hello_world", [200, 200])
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let mut glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf", TextureSettings::new()).unwrap();
    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new());

    let mut x = 20.0;
    let mut y = 160.0;

    let mut A_pressed = false;
    let mut D_pressed = false;
    let mut W_pressed = false;
    let mut S_pressed = false;

    while let Some(e) = events.next(&mut window) {
        use graphics::*;

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                let transform = c.transform.trans(x, y);

                clear([0.0, 0.0, 0.0, 1.0], g);
                text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32)
                    .draw("Hello world!", &mut glyphs, &c.draw_state, transform, g);
            });
        }

        if let Some(Button::Keyboard(Key::A)) = e.press_args() {
            A_pressed = true;
        } else if let Some(Button::Keyboard(Key::A)) = e.release_args() {
          A_pressed = false;
        }

        if A_pressed {
            x -= 1.0;
            println!("Moved thing left");
        }

        if let Some(Button::Keyboard(Key::D)) = e.press_args() {
            D_pressed = true;
        } else if let Some(Button::Keyboard(Key::D)) = e.release_args() {
            D_pressed = false;
        }

        if D_pressed {
            x += 1.0;
            println!("Moved thing right");
        }

        if let Some(Button::Keyboard(Key::W)) = e.press_args() {
            W_pressed = true;
        } else if let Some(Button::Keyboard(Key::W)) = e.release_args() {
            W_pressed = false;
        }

        if W_pressed {
            y -= 1.0;
            println!("Moved thing up");
        }

        if let Some(Button::Keyboard(Key::S)) = e.press_args() {
            S_pressed = true;
        } else if let Some(Button::Keyboard(Key::S)) = e.release_args() {
            S_pressed = false;
        }

        if S_pressed {
            y += 1.0;
            println!("Moved thing down");
        }
    }
}
