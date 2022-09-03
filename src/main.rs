use egui::{Vec2, Pos2};

struct mapMaker {
    slider1: f32,
}

impl eframe::App for mapMaker {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        
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
    fn new(s1: f32) -> Self {
        mapMaker { slider1: s1}
    }
}

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(240.0, 1080.0));
    native_options.initial_window_pos = Some(Pos2::new(-7.5,0.0));
    eframe::run_native("MapMaker", native_options, Box::new(|cc| Box::new(mapMaker::new(0.0))));
}