use std::cmp::max;

use noise::{Perlin, Seedable};
use rand::{SeedableRng, rngs::StdRng};
use sdl2::pixels::Color;

pub struct map {
    seed: u32,
    rng: StdRng,
    perlin: Perlin
}

impl map {
    pub fn new(seed: u32) -> Self {
        map { seed, rng: rand::rngs::StdRng::seed_from_u64(seed as u64), perlin: Perlin::new().set_seed(seed) }
    }

    pub fn set_biomes(&mut self, biomeSizes: [u32; 4]) -> Vec<Biome> {
        let mut res: Vec<Biome> = vec![];
        for i in 0..4 {
            res.push(Biome {
                BiomeType: i as u8,
                x: rand::Rng::gen_range(&mut self.rng, 0..=125),
                y: rand::Rng::gen_range(&mut self.rng, 0..=125),
                r: biomeSizes[i] as f32,
            });
            //println!("{:?}", res[i]);
        }
        res
    }

    pub fn get_point_biomes(&mut self, x: i32, y: i32, biomes: Vec<Biome>) -> Vec<f32> {
        let mut res: Vec<f32> = vec![];
        for i in biomes {
            res.push(((
                (i.x - x)*(i.x - x) / 4 + (i.y - y)*(i.y - y) * 4) as f32)
                .sqrt() * rand::Rng::gen_range(&mut self.rng, 0.75..=1.0))
        }
        res
    }

    pub fn get_point_color(&mut self, biome: Vec<f32>) -> Color {
        let darkness = rand::Rng::gen_range(&mut self.rng, 0.95..=1.05);
        let sum = biome[0] + biome[1] + biome[2] + biome[3];
        /*colors:
        desert:         235, 230, 180
        forest:         40,  145, 75
        ice:            200, 235, 235
        dry:            70,  60,  30
        */
        let max = biome.iter().cloned().fold(0./0., f32::max);
        let mut col: Color = Color::BLACK;
        for i in 0..biome.len() {
            if biome[i] == max {
                match i {
                    0 => { //desert
                        col = Color::RGB((235.0 * darkness) as u8, (230.0 * darkness) as u8, (180.0 * darkness) as u8);
                    }
                    1 => { //forest
                        col = Color::RGB((40.0 * darkness) as u8, (145.0 * darkness) as u8, (75.0 * darkness) as u8);
                    }
                    2 => { //ice
                        col = Color::RGB((200.0 * darkness) as u8, (235.0 * darkness) as u8, (235.0 * darkness) as u8);
                    }
                    3 => { //dry
                        col = Color::RGB((70.0 * darkness) as u8, (60.0 * darkness) as u8, (30.0 * darkness) as u8);
                    }
                    _ => {}
                }
            }
        }
        col
        
    }

    pub fn seed(&self) -> u32 {
        self.seed
    }
}

/*enum BiomeType {
    desert, forest, ice, dry
}*/

//biome types:
//0 => desert
//1 => forest
//2 => ice
//3 => dry
#[derive(Debug, Clone)]
pub struct Biome {
    BiomeType: u8,
    x: i32, y: i32,
    r: f32
}
