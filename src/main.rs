use std::{f32::consts::PI, collections::HashMap, fs, io::Read};
use macroquad::prelude::*;
use raycast_dda::{RayCastEngine, BLOCK_SIZE};
use serde_json::Value;

const WIDTH: u32 = 1280; // window width
const HEIGHT: u32 = 720; // window height
const PLAYER_MOVE_SPEED: f32 = 8.; // the players move speed
const PLAYER_TURN_SPEED: f32 = 2.; // the player turn speed

fn window_conf() -> Conf {
    Conf {
        window_title: "RayCast Test".to_owned(),
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let mut level_data = String::new();
    fs::File::open("test_level.ldtk").unwrap().read_to_string(&mut level_data).unwrap();
    let level: Value = serde_json::from_str(level_data.as_str()).unwrap();
    let level: &Value = &level["levels"][0]["layerInstances"];


    let bricks = load_texture("bricks.png").await.unwrap();
    let blackstone = load_texture("polished_blackstone_bricks.png").await.unwrap();
    let planks = load_texture("oak_planks.png").await.unwrap();
    bricks.set_filter(FilterMode::Nearest);
    blackstone.set_filter(FilterMode::Nearest);
    planks.set_filter(FilterMode::Nearest);

    let block_size = BLOCK_SIZE as u32;
    let textures: HashMap<u32, (Texture2D, Color)> = HashMap::from([
        (1, (bricks, get_average_texture_color(&bricks, block_size, block_size))),
        (2, (blackstone, get_average_texture_color(&blackstone, block_size, block_size))),
        (3, (planks, get_average_texture_color(&planks, block_size, block_size)))
    ]);
    
    // initial player position and rotation
    let player = vec2(level[0]["entityInstances"][0]["__grid"][0].as_f64().unwrap() as f32 + 0.5, level[0]["entityInstances"][0]["__grid"][1].as_f64().unwrap() as f32 + 0.5);
    let player_angle = 0.;

    // the size of the map
    let map_size = UVec2::new(level[1]["__cWid"].as_u64().unwrap() as u32, level[1]["__cHei"].as_u64().unwrap() as u32);

    // the map that the can player move through and look around in
    let map: Vec<u32> = level[1]["intGridCsv"].as_array().unwrap().iter().map(|v| v.as_u64().unwrap() as u32).collect();

    let mut engine = RayCastEngine::new(map, map_size, player, player_angle, textures);

    loop {
        clear_background(BLACK);
        let delta_time = get_frame_time();

        // updates the players viewing angle
        if is_key_down(KeyCode::A) { engine.camera_angle -= PLAYER_TURN_SPEED * delta_time; }
        if is_key_down(KeyCode::D) { engine.camera_angle += PLAYER_TURN_SPEED * delta_time; }

        // limits the viewing angle to be between 0-2PI
        if engine.camera_angle < 0.      { engine.camera_angle = 2. * PI; }
        if engine.camera_angle > 2. * PI { engine.camera_angle = 0.; }

        // moves the player forward or backwards
        // with super basic collision detection
        if is_key_down(KeyCode::W) {
            engine.camera.x += f32::cos(engine.camera_angle) * delta_time * PLAYER_MOVE_SPEED;
            engine.camera.y += f32::sin(engine.camera_angle) * delta_time * PLAYER_MOVE_SPEED;

            if engine.map[((engine.camera.y as u32) * engine.map_size.x + (engine.camera.x as u32)) as usize] > 0 {
                engine.camera.x -= f32::cos(engine.camera_angle) * delta_time * PLAYER_MOVE_SPEED;
                engine.camera.y -= f32::sin(engine.camera_angle) * delta_time * PLAYER_MOVE_SPEED;
            }
        }
        if is_key_down(KeyCode::S) {
            engine.camera.x -= f32::cos(engine.camera_angle) * delta_time * PLAYER_MOVE_SPEED;
            engine.camera.y -= f32::sin(engine.camera_angle) * delta_time * PLAYER_MOVE_SPEED;

            if engine.map[((engine.camera.y as u32) * engine.map_size.x + (engine.camera.x as u32)) as usize] > 0 {
                engine.camera.x += f32::cos(engine.camera_angle) * delta_time * PLAYER_MOVE_SPEED;
                engine.camera.y += f32::sin(engine.camera_angle) * delta_time * PLAYER_MOVE_SPEED;
            }
        }

        engine.draw_screen();

        draw_text(format!("FPS: {}", get_fps()).as_str(), 5., HEIGHT as f32 - 5., 60., WHITE);

        next_frame().await;
    }
}

fn get_average_texture_color(texture: &Texture2D, width: u32, height: u32) -> Color {
    let image = texture.get_texture_data();
    let mut total_r = 0.;
    let mut total_g = 0.;
    let mut total_b = 0.;
    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            total_r += pixel.r;
            total_g += pixel.g;
            total_b += pixel.b;
        }
    }

    Color::new(total_r / (width * height) as f32, total_g / (width * height) as f32, total_b / (width * height) as f32, 1.)
}