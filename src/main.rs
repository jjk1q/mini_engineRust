use std::collections::HashSet;
use std::num::NonZeroU32;
use std::rc::Rc;

use softbuffer::{Context, Surface};
use winit::application::ApplicationHandler;
use winit::event::ElementState;
use winit::event::WindowEvent::{self};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{ Key, KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};
use winit::dpi::LogicalSize;
use std::time::{Duration, Instant};

fn main() {
    let event_loop = EventLoop::new().expect("не палучилась бурмалда");
    let mut app = App {
        window: None,
        surface: None,
        clear_color: 0x00202030,
        player: Player {
            pos: Vec2 {
                x: 200.0,
                y: 200.0
            },
            w: 100,
            h: 100,
            color: 0x00FF8800,
            speed: 60.0,
        },
        fps: FpsCounter { 
            frames: 0,
            started: Instant::now()
        },
        clock: Clock{
            last: Instant::now()
        },
        input: Input { held: HashSet::new()  }
    };

   
    event_loop.run_app(&mut app).expect("еее бурмалда не запустить");
}

#[derive(Debug, Clone, Copy)]
struct Rect {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
}

struct Player {
    pos: Vec2,
    w: u32,
    h: u32,
    color: u32,
    speed: f32,
}

#[derive()]
struct App{
    window: Option<Rc<Window>>,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
    clear_color: u32,
    player: Player,
    fps: FpsCounter,
    clock: Clock,
    input: Input
}

struct FpsCounter{
    frames: u32,
    started: Instant
}

struct Clock{
    last: Instant
}

struct Input{
    held: HashSet<KeyCode>
}

struct Vec2{
    x: f32,
    y: f32
}

impl Input{
    fn press(&mut self, code: KeyCode){
        self.held.insert(code);
    }

    fn release(&mut self, code: KeyCode){
        self.held.remove(&code);
    }

    fn is_held(&self, code: KeyCode) -> bool{
        self.held.contains(&code)
    }
}

impl Clock{
    fn delta(&mut self) -> f32{
        let now = Instant::now();
        let diff = now - self.last;
        self.last = now;
        diff.as_secs_f32()
    }
}

impl FpsCounter{
    fn tick(&mut self) -> Option<u32>{
        self.frames += 1;
        if self.started.elapsed() >= Duration::from_secs(1){
            let fps = self.frames;
            self.frames = 0;
            self.started = Instant::now();
            return Some(fps);
        }
        None
    }
}

impl Player {
    fn move_by(&mut self, dx: f32, dy: f32, bounds_w: f32, bounds_h: f32) {
        let max_x = (bounds_w - self.w as f32).max(0.0);
        let max_y = (bounds_h - self.h as f32).max(0.0);

        self.pos.x = (self.pos.x + dx).clamp(0.0, max_x);
        self.pos.y = (self.pos.y + dy).clamp(0.0, max_y);
    }

    fn rect(&self) -> Rect{
        Rect {x: self.pos.x as i32,y: self.pos.y as i32, w: self.w, h: self.h }
    }
}

fn draw_rect(buffer: &mut [u32], buffer_w: usize, buffer_h: usize, rect: &Rect, color: u32) {

    let start_x = rect.x.max(0);
    let end_x = (rect.x + rect.w as i32).min(buffer_w as i32);

    let start_y = rect.y.max(0);
    let end_y = (rect.y + rect.h as i32).min(buffer_h as i32);

    for y in start_y..end_y {
        for x in start_x..end_x {
            let index = y as usize * buffer_w + x as usize;
            buffer[index] = color;
        }
    }
}

impl App{
    fn redraw(&self){
        let window = self.window.as_ref();
        if let Some(window) = window{
            window.request_redraw();    
        }
    }

    fn update(&mut self, dt: f32){
        let mut dx = 0.0;
        let mut dy = 0.0;

        if self.input.is_held(KeyCode::ArrowUp) { dy += -1.0; }
        if self.input.is_held(KeyCode::ArrowDown) { dy += 1.0 }
        if self.input.is_held(KeyCode::ArrowLeft) { dx += -1.0 }
        if self.input.is_held(KeyCode::ArrowRight) { dx += 1.0 }

        let Some(window) = self.window.as_ref() else { return;};
        let size = window.inner_size();
        let bounds_w = size.width as f32;
        let bounds_h = size.height as f32;

        self.player.move_by(dx * self.player.speed * dt, dy * self.player.speed * dt, bounds_w, bounds_h);

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
                if let PhysicalKey::Code(code) = key_event.physical_key{
                    match key_event.state{
                        ElementState::Pressed => {
                            self.input.press(code);
                            match code{
                                KeyCode::Escape => event_loop.exit(),
                                KeyCode::KeyC => {
                                    self.clear_color = rand::random::<u32>() & 0x00FFFFFF;
                                }
                                _ => {}
                            }
                        }
                        ElementState::Released => {
                            self.input.release(code);
                        }
                    }
                }
                println!("{:?}", self.input.held);
                self.redraw();
            },
            WindowEvent::RedrawRequested =>{
                let dt = self.clock.delta();
                self.update(dt);
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
                        
                        draw_rect(&mut buffer, size.width as usize, size.height as usize, &self.player.rect(), self.player.color);
                        
                        buffer.present().expect("line 131");

                        let fps = self.fps.tick();
                        if let Some(fps) = fps{
                            println!("{}", fps);
                            println!("{}", dt);
                        }
                        
                        window.request_redraw();
                    }
                }
            },
            _ => {}
        }
    }
}