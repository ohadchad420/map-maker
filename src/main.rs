use egui::{Vec2, Pos2};
use sdl2::{render::Canvas, video::Window, pixels::Color};

struct mapMaker {
    sdl2: sdl2::Sdl,
    canvas: Canvas<Window>,
    slider1: f32,
}

impl eframe::App for mapMaker {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        if !self.close_window() {
            let col = (self.slider1 * 255.0) as u8;
            self.canvas.set_draw_color(Color::RGB(col,col,col));
            println!("{:?}", Color::RGB(col,col,col));
            self.canvas.clear();
            self.canvas.present();
        }
        

        
        let text1 = format!("slider1: {}", self.slider1);
        let s1 = egui::Slider::new(&mut self.slider1, 0.0..=1.0).show_value(false);
        

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Map Generator");
            ui.label("using SDL2 and EFRAME");
            ui.label("-------------------------------------------------");
            ui.label(text1);
            ui.add(s1);
        });

    }
}

impl mapMaker {
    fn new(sdl2: sdl2::Sdl, canvas: Canvas<Window>, s1: f32) -> Self {
        mapMaker { sdl2, canvas, slider1: s1}
    }

    fn close_window(&mut self) -> bool {
        let mut event_pump = self.sdl2.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} |
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => {
                    return true;
                    },
                _ => {}
            }
        }
        return false;

    }
}

fn main() {

    let sdl2 = sdl2::init().unwrap();
    let video = sdl2.video().unwrap();

    let window = video.window("Life", 1000, 1000).build()
    .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build().expect("could not make a canvas");

    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(240.0, 720.0));
    native_options.initial_window_pos = Some(Pos2::new(-7.5,0.0));
    eframe::run_native("MapMaker", native_options, Box::new(|cc| Box::new(mapMaker::new(sdl2, canvas, 0.0))));


}