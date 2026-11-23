use std::rc::Rc;

use softbuffer::{Context, Surface};
use winit::application::ApplicationHandler;
use winit::event::ElementState;
use winit::event::WindowEvent::{self, KeyboardInput};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::window::{Window, WindowId};
use winit::dpi::LogicalSize;

fn main() {
    let event_loop = EventLoop::new().expect("не палучилась бурмалда");
    let mut app = App {window: None, surface: None, clear_color: 0x00202030};
    event_loop.run_app(&mut app).expect("еее бурмалда не запустить");


}

#[derive()]
struct App{
    window: Option<Rc<Window>>,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
    clear_color: u32
}


impl ApplicationHandler for App{
    fn resumed(&mut self, event_loop: &ActiveEventLoop){
        if self.window.is_none(){
            let attrs = Window::default_attributes()
            .with_title("Mini Engine")
            .with_inner_size(LogicalSize::new(800.0, 600.0));

            let window= Rc::new(event_loop.create_window(attrs).expect("не удалось создать бурмалду извините"));

            let context = Context::new(window.clone()).expect("...");
            let surface = Surface::new(&context, window.clone()).expect("...");

            self.window = Some(window);
            self.surface = Some(surface);

        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent){
        match event{
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput{event: key_event, ..} => {
                if key_event.state == ElementState::Pressed{
                    if key_event.logical_key == Key::Named(NamedKey::Escape){
                        event_loop.exit();
                    }
                    else {
                        println!("{:?}", key_event.logical_key);
                    }
                }
            },
            _ => {}
        }
    }
}