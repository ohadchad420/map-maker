use egui::{Vec2, Pos2};
use sdl2::{render::Canvas, video::Window, pixels::Color};

mod mapHandler;
mod map;

fn main() {

    let sdl2 = sdl2::init().unwrap();
    let video = sdl2.video().unwrap();

    let mut rng = rand::thread_rng();

    let WIDTH: i32 = 500;
    let HEIGHT: i32 = 500;

    let window = video.window("Map", WIDTH as u32, HEIGHT as u32).build()
    .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build().expect("could not make a canvas");

    let seed: u32 = rand::Rng::gen(&mut rng);
    let mut map = map::map::new(seed);
    let biomes = map.set_biomes([65,55,44,11]);
    for x in 0..WIDTH/4 {
        for y in 0..HEIGHT/4 {
            let point_biome = map.get_point_biomes(x, y, biomes.clone());
            canvas.set_draw_color(map.get_point_color(point_biome));
            canvas.fill_rect(sdl2::rect::Rect::new(x as i32 * 4, y as i32 * 4, 4, 4)).unwrap();
            //println!("{:?}", canvas.draw_color());
        }
        //println!("{:?}", canvas.draw_color());
    }

    canvas.present();

    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(240.0, 720.0));
    native_options.initial_window_pos = Some(Pos2::new(420.0,180.0));
    eframe::run_native("Map Gen", native_options, Box::new(|cc| 
        Box::new(mapHandler::mapMaker::new(sdl2, canvas, map, [65,55,44,11]))));


}