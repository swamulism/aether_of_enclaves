/**
    Map generates and manages the tileset for the map using Perlin and Worley generations.
*/

use noise::*;
use rand::{thread_rng, Rng};
use tile::{Tile, TileType};
use constants::*;
use piston_window::*;
use std::cmp;
use std::collections::HashMap;

const STEP_SIZE: f64 = 0.1;

pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let air = Tile::new(TileType::Air);
        let grass_floor = Tile::new(TileType::GrassFloor);
        let water = Tile::new(TileType::Water);
        let dirt_floor = Tile::new(TileType::DirtFloor);
        let stone_wall = Tile::new(TileType::StoneWall);

        let mut map_tiles = vec![vec![air.clone(); height]; width];
        let worley_arr = generate_worley(width, height, STEP_SIZE);
        let perlin_arr = generate_perlin(width, height, STEP_SIZE);

        for i in 0..map_tiles.len() {
            for j in 0..map_tiles[i].len() {
                let num = worley_arr[i][j] * perlin_arr[i][j];
                if num <= -0.3 {
                    map_tiles[i][j] = water.clone();
                } else if num <= 0.0 {
                    map_tiles[i][j] = dirt_floor.clone();
                } else if num <= 0.3 {
                    map_tiles[i][j] = grass_floor.clone();
                } else {
                    map_tiles[i][j] = stone_wall.clone();
                }
            }
        }
        Map { tiles: map_tiles }
    }
    pub fn draw(
        &mut self,
        textures: &HashMap<String, G2dTexture>,
        context: &Context,
        graphics: &mut G2d,
        w_width: f64,
        w_height: f64,
        player_x: f64,
        player_y: f64,
        trans_x: f64,
        trans_y: f64,
    ) {
        // Draw map.
        let draw_start_i = ((player_x - w_width / 2.0) - IMAGE_SIZE) / IMAGE_SIZE;
        let draw_start_j = ((player_y - w_height / 2.0) - IMAGE_SIZE) / IMAGE_SIZE;
        let draw_start_i = cmp::max(0, draw_start_i as i32) as usize;
        let draw_start_j = cmp::max(0, draw_start_j as i32) as usize;
        for i in draw_start_i..self.tiles.len() {
            if i as f64 * IMAGE_SIZE > player_x + w_width / 2.0 {
                break;
            }
            for j in draw_start_j..self.tiles[i].len() {
                if j as f64 * IMAGE_SIZE > player_y + w_height / 2.0 {
                    break;
                }
                match self.tiles[i][j].tile_type {
                    TileType::Water => {
                        let img = IMG_WATER;
                        image(
                            textures.get(img).expect(&format!("Not found: {:?}", img)),
                            context
                                .transform
                                .trans(i as f64 * IMAGE_SIZE, j as f64 * IMAGE_SIZE)
                                .trans(trans_x, trans_y),
                            graphics,
                        );
                    }
                    TileType::StoneWall => {
                        let img = IMG_STONE_WALL;
                        image(
                            textures.get(img).expect(&format!("Not found: {:?}", img)),
                            context
                                .transform
                                .trans(i as f64 * IMAGE_SIZE, j as f64 * IMAGE_SIZE)
                                .trans(trans_x, trans_y),
                            graphics,
                        );
                    }
                    TileType::GrassFloor => {
                        let img = IMG_GRASS_FLOOR;
                        image(
                            textures.get(img).expect(&format!("Not found: {:?}", img)),
                            context
                                .transform
                                .trans(i as f64 * IMAGE_SIZE, j as f64 * IMAGE_SIZE)
                                .trans(trans_x, trans_y),
                            graphics,
                        );
                    }
                    TileType::DirtFloor => {
                        let img = IMG_DIRT_FLOOR;
                        image(
                            textures.get(img).expect(&format!("Not found: {:?}", img)),
                            context
                                .transform
                                .trans(i as f64 * IMAGE_SIZE, j as f64 * IMAGE_SIZE)
                                .trans(trans_x, trans_y),
                            graphics,
                        );
                    }
                    _ => {}
                }
            }
        }
    }
}

fn generate_perlin(width: usize, height: usize, step: f64) -> Vec<Vec<f64>> {
    let mut rng = thread_rng();
    let noise = Perlin::new().set_seed(rng.gen::<u32>());
    let mut xpos = 0.0;
    let mut ypos = 0.0;
    let mut arr = vec![vec![0.0; height]; width];
    for i in 0..width {
        for j in 0..height {
            arr[i][j] = noise.get([xpos, ypos]);
            xpos += step;
        }
        xpos = 0.0;
        ypos += step;
    }
    arr
}

fn generate_worley(width: usize, height: usize, step: f64) -> Vec<Vec<f64>> {
    let mut rng = thread_rng();
    let noise = Worley::new().set_seed(rng.gen::<u32>()).enable_range(true);
    let mut xpos = 0.0;
    let mut ypos = 0.0;
    let mut arr = vec![vec![0.0; height]; width];
    for i in 0..width {
        for j in 0..height {
            arr[i][j] = noise.get([xpos, ypos]);
            xpos += step;
        }
        xpos = 0.0;
        ypos += step;
    }
    arr
}

// fn snip_off(vec: Vec<Vec<f64>>) -> Vec<Vec<f64>>{
//     let mut arr = vec![vec![0.0;vec[0].len()]; vec.len()];
//     for i in 0..vec.len() {
//         for j in 0..vec[i].len() {
//             if vec[i][j] > 0.0 {
//                 arr[i][j] = vec[i][j];
//             }
//         }
//     }
//     arr
// }

// enum IslandType {
//     Plains,
//     Water,
//     // Ice,
//     // Lavae,
//     // Civilization,
// }

// struct Island {
//     pub island_type: IslandType,
//     pub tiles: Vec<Vec<Tile>>,
//     pub x: f64,
//     pub y: f64,
// }

// impl Island {
//     fn new() -> Self {
//         Island {
//             island_type: IslandType::Plains,
//             tiles: vec![vec![]],
//             x: 0.0,
//             y: 0.0
//         }
//     }
// }
// need to fix so edge is weighted 0 and middle is weighted 1
// fn generate_weighted_circle(size: usize) -> Vec<Vec<f64>> {
//     let mut circle_arr = vec![vec![0.0; size]; size];
//     let sizef = size as f64;
//     let middle: f64 = sizef / 2.0;
//     for i in 0..size {
//         for j in 0..size {
//             let x: f64 = middle - i as f64;
//             let x = x * x;
//             let y: f64 = middle - j as f64;
//             let y = y * y;
//             circle_arr[i][j] = (sizef - (x + y).sqrt()) / sizef;
//         }
//     }
//     circle_arr
// }

// fn generate_island_size() -> usize {
//     let mut normal = distributions::normal::Normal::new(ISLAND_MEAN, ISLAND_STANDERD_DEV);
//     let mut island_size = normal.sample(&mut thread_rng());
//     while island_size <= ISLAND_LOWERBOUND && island_size >= ISLAND_UPPERBOUND {
//         island_size = normal.sample(&mut thread_rng());
//     }
//     let island_size = island_size as usize;
//     island_size
// }
