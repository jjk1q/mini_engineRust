use std::num::NonZeroU32;
use std::rc::Rc;

use softbuffer::{Context, Surface};
use winit::application::ApplicationHandler;
use winit::event::ElementState;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{Key, KeyCode, NamedKey, PhysicalKey};
use winit::window::{Window, WindowId};
use winit::dpi::LogicalSize;

fn main() {
    let event_loop = EventLoop::new().expect("не палучилась бурмалда");

    let rect = Rect { x: 100, y: 100, w: 100, h: 100 };
    let player = Player{rect, color: 0x00FF8800, speed: 10};
    let mut app = App {window: None, surface: None, clear_color: 0x00202030, player};
    event_loop.run_app(&mut app).expect("еее бурмалда не запустить");


}

fn draw_rect(buffer: &mut [u32],buffer_width: usize, px: usize, py: usize, w: usize, h: usize, color: u32){

    let buffer_height = buffer.len() / buffer_width;
    for y in py..(py + h).min(buffer_height){
        for x in px..(px + w).min(buffer_width){
            let index = y * buffer_width + x;
            buffer[index] = color;
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Rect{
    x: i32,
    y: i32,
    w: u32,
    h: u32
}

struct Player{
    rect: Rect,
    color: u32,
    speed: i32
}

struct App{
    window: Option<Rc<Window>>,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
    clear_color: u32,
    player: Player
}

impl Player{
    fn move_by(&mut self, dx: i32, dy: i32, bounds_w: i32, bounds_h: i32){
        let max_x = (bounds_w - self.rect.w as i32).max(0);
        self.rect.x = (self.rect.x + dx).clamp(0, max_x);

        let max_y = (bounds_h - self.rect.h as i32).max(0);
        self.rect.y = (self.rect.y + dy).clamp(0, max_y);
    }
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

                        draw_rect(&mut buffer,size.width as usize, 500, 680, 100, 100, 0x00E63946);
                        buffer.present().expect("..."); 
                    }
                }
            },
            _ => {}
        }
    }
}