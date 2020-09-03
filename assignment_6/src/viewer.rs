use minifb::{Key, Window, WindowOptions, MouseMode, MouseButton};
use image::RgbaImage;

pub trait ViewerStrategy {
    fn mouse_move(&self, x: f32, y: f32);
    fn mouse_pressed(&mut self, x: f32, y: f32, button: MouseButton);
    fn mouse_wheel(dx: u32, dy: u32, is_direction_normal: bool);
    fn key_pressed(&self, key: minifb::Key);
    fn redraw(&mut self, img: &mut RgbaImage);
    fn new() -> Self;
}

pub struct Viewer<S: ViewerStrategy> {
    pub redraw_next: bool,
    window: Window,
    buffer: RgbaImage,
    pub s: S,
}

impl<S: ViewerStrategy> Viewer<S> {
    pub fn new(window_name: &str, w: usize, h: usize) -> Self {
        let window = Window::new(window_name, w, h, WindowOptions::default())
        .unwrap_or_else(|e| panic!("{}", e));
        let buffer = RgbaImage::new(w as u32, h as u32);
        Self {
            redraw_next: true,
            window,
            buffer,
            s: S::new(),
        }
    }

    pub fn resize(w: u32, h: u32) {}

    pub fn update(&mut self) {
        if !self.redraw_next {
            return;
        }
        self.redraw_next = false;
        self.s.redraw(&mut self.buffer);
    }

    pub fn launch(&mut self, redraw_interval: std::time::Duration) {
        self.window.limit_update_rate(Some(redraw_interval));
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {

            self.window.get_mouse_pos(MouseMode::Discard).map(|(x, y)| {

                self.s.mouse_move(x, y);
                if self.window.get_mouse_down(MouseButton::Left) {
                    self.s.mouse_pressed(x, y, MouseButton::Left);
                    self.redraw_next = true;
                } else if self.window.get_mouse_down(MouseButton::Right) {
                    self.s.mouse_pressed(x, y, MouseButton::Right);
                    self.redraw_next = true;
                }
            });

            self.window.get_keys_pressed(minifb::KeyRepeat::No).map(|keys| {
                for key in keys {
                    self.s.key_pressed(key)
                }
            });

            let linear_buffer = &self.buffer.pixels().into_iter().map(| pixel| {
                let color: u32 = (pixel[3] as u32) << 24 
                    | (pixel[0] as u32) << 16 
                    | (pixel[1] as u32) << 8 
                    | pixel[2] as u32;
                color
            }).collect::<Vec<u32>>();

            self.update();

            let w = self.buffer.width() as usize;
            let h = self.buffer.height() as usize;
            self.window.update_with_buffer(&linear_buffer, w, h).unwrap();
        }
    }
}
