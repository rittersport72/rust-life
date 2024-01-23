mod life;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use piston::event_loop::{EventSettings, Events};
use piston::input::mouse::MouseCursorEvent;
use piston::input::{Button, MouseButton, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::{PressEvent, ReleaseEvent};

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new(
        "life",
        [
            life::GRID_X_COUNT * life::CELL_SIZE,
            life::GRID_Y_COUNT * life::CELL_SIZE,
        ],
    )
    .graphics_api(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

    // Create a new game and run it.
    let mut app = life::Application::new();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }
        if let Some(args) = e.update_args() {
            app.update(&args);
        }
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            app.mouse_pressed();
        }
        if let Some(Button::Mouse(MouseButton::Left)) = e.release_args() {
            app.mouse_released();
        }
        if let Some(position) = e.mouse_cursor_args() {
            app.mouse_moved(position);
        }
    }
}
