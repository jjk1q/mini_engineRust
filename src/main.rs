use std::num::NonZeroU32;
use std::rc::Rc;

use softbuffer::{Context, Surface};
use winit::application::ApplicationHandler;
use winit::event::ElementState;
use winit::event::WindowEvent::{self, KeyboardInput, RedrawRequested};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{Key, KeyCode, NamedKey, PhysicalKey};
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
                    else if key_event.physical_key == PhysicalKey::Code(KeyCode::KeyC){
                        self.clear_color = rand::random::<u32>() & 0x00FFFFFF;
                        let window = self.window.as_ref();
                        if let Some(window) = window{
                            window.request_redraw();
                        }
                    }
                    else {
                        println!("{:?}", key_event.logical_key);
                    }
                }
            },
            WindowEvent::RedrawRequested =>{
                let window = self.window.as_ref();
                let surface = self.surface.as_mut();
                if let (Some(window), Some(surface)) = (window, surface){
                    let size = window.inner_size();
                    let (width, height) = (NonZeroU32::new(size.width), NonZeroU32::new(size.height));

                    if let (Some(width), Some(height)) = (width, height){
                        surface.resize(width, height).expect("line 68");
                        let mut buffer = surface.buffer_mut().expect("line 69");
                        for pixel in buffer.iter_mut(){
                            *pixel = self.clear_color;  
                        }
                        buffer.present().expect("..."); 
                    }
                }
            },
            _ => {}
        }
    }
}