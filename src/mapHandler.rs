use egui::{Vec2, Pos2};
use sdl2::{render::Canvas, video::Window, pixels::Color};

use crate::map::map;

pub struct mapMaker {
    sdl2: sdl2::Sdl,
    canvas: Canvas<Window>,

    map: map,

    desert: u32,
    forest: u32,
    ice: u32,
    dry: u32,
}

impl eframe::App for mapMaker {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        if !self.close_window() {
            //let col = (self.slider1 * 255.0) as u8;
            //self.canvas.set_draw_color(Color::RGB(col,col,col));
            //println!("{:?}", Color::RGB(col,col,col));
            //self.canvas.clear();
            //self.canvas.present();

            let cache = vec![self.desert, self.forest, self.ice, self.dry];

            let text1 = format!("desert: {}", self.desert);
            let s1 = egui::Slider::new(&mut self.desert, 0..=125).show_value(false);

            let text2 = format!("forest: {}", self.forest);
            let s2 = egui::Slider::new(&mut self.forest, 0..=125).show_value(false);

            let text3 = format!("ice: {}", self.ice);
            let s3 = egui::Slider::new(&mut self.ice, 0..=125).show_value(false);

            let text4 = format!("dry: {}", self.dry);
            let s4 = egui::Slider::new(&mut self.dry, 0..=125).show_value(false);
            
    
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Map Generator");
                ui.label("using SDL2 and EFRAME");
                ui.label("-------------------------------------------------");
                ui.label(text1);
                ui.add(s1);
                ui.label(text2);
                ui.add(s2);
                ui.label(text3);
                ui.add(s3);
                ui.label(text4);
                ui.add(s4);

                
            });

            if cache != vec![self.desert, self.forest, self.ice, self.dry] {
                //println!("here");
                let biomes = self.map.set_biomes([125 - self.desert,
                                                                    125 - self.forest,
                                                                    125 - self.ice,
                                                                    125 - self.dry]);
                self.canvas.clear();
                for x in 0..500/4 {
                    for y in 0..500/4 {
                        let point_biome = self.map.get_point_biomes(x, y, biomes.clone());
                        self.canvas.set_draw_color(self.map.get_point_color(point_biome));
                        self.canvas.fill_rect(sdl2::rect::Rect::new(x as i32 * 4, y as i32 * 4, 4, 4)).unwrap();
                        //println!("{:?}", canvas.draw_color());
                    }
                    //println!("{:?}", canvas.draw_color());
                }
                self.canvas.present();
            }
        } else {
            frame.close();
        }


    }
}

impl mapMaker {
    pub fn new(sdl2: sdl2::Sdl, canvas: Canvas<Window>, map: map, biomes: [u32; 4]) -> Self {
        mapMaker { sdl2, canvas, map, desert: biomes[0], forest: biomes[1], ice: biomes[2], dry: biomes[3]}
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