mod adapter;

// ## Mainloop imports ##
use std::{process::exit, task::Poll};
////use rand::Rng;
////use rusqlite::{params, Connection};
////use core::panic;
////use std::io;
////use chrono::Utc;

// ## Winit imports ##
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    keyboard::{KeyCode, PhysicalKey},
    application::ApplicationHandler,
    window::Window,
};

// ## WGPU imports ##


#[derive(Default)]
struct App {
	window: Option<Window>,
}

// Window redraw behaviour
impl ApplicationHandler for App {
	fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes: winit::window::WindowAttributes = Window::default_attributes()
        .with_title("Flashcard Application")
        .with_visible(false)
        .with_window_level(winit::window::WindowLevel::AlwaysOnTop); // Appears above all other windows

		self.window = Some(event_loop.create_window(window_attributes).unwrap());
		
        self.window.as_ref().unwrap().set_window_level(winit::window::WindowLevel::Normal); // Can be moved behind other windows

        self.window.as_ref().unwrap().set_visible(true);
	}

	fn window_event(
			&mut self,
			_event_loop: &winit::event_loop::ActiveEventLoop,
			_window_id: winit::window::WindowId,
			event: winit::event::WindowEvent,
		) {
			match event {
				WindowEvent::CloseRequested => {
					println!("EXITING: CODE 0");
					////event_loop.exit();
                    exit(0);
				},
                WindowEvent::Resized(..) => {
                    self.window.as_ref().unwrap().request_redraw();
                },
				WindowEvent::RedrawRequested => {
					/* Redraw the application.
					It's preferable for applications that do not render continuously to render in
					this event rather than in AboutToWait, since rendering in here allows
					the program to gracefully handle redraws requested by the OS.
	
					Draw.
	
					Queue a RedrawRequested event.
					
					You only need to call this if you've determined that you need to redraw in
					applications which do not always need to. Applications that redraw continuously
					can render here instead. */
					self.window.as_ref().unwrap().request_redraw();
					// Only redraws when needed (OS event)
				},
				_ => (),
			}
		}
}

fn main() {
    ////env_logger::init();

    adapter::main();

    let event_loop: EventLoop<()> = EventLoop::new().unwrap();
	event_loop.set_control_flow(ControlFlow::Wait); // Only redraws when needed (OS event)

	let mut app = App::default();
    let _ = event_loop.run_app(&mut app);
}