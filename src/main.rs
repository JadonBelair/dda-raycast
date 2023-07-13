use macroquad::prelude::*;
// use raycast_dda::{RayCastEngine, Map};
// use serde_json::Value;
// use std::{collections::HashMap, f32::consts::PI};

const WIDTH: i32 = 1280; // window width
const HEIGHT: i32 = 720; // window height
const VIEW_DISTANCE: f32 = 30.; // how many grid spaces the camera can see up to
const BLOCK_SIZE: f32 = 64.; // the size of the textures used for the walls
const PLAYER_MOVE_SPEED: f32 = 8.; // the players move speed
const PLAYER_TURN_SPEED: f32 = 2.; // the player turn speed
const FOV: f32 = 60.; // the cameras fov in degrees

fn window_conf() -> Conf {
    Conf {
        window_title: "RayCast Test".to_owned(),
        window_width: WIDTH,
        window_height: HEIGHT,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let bricks = load_texture("bricks.png").await.unwrap();
    let bricks_image = bricks.get_texture_data();

    let blackstone = load_texture("polished_blackstone_bricks.png").await.unwrap();
    let blackstone_image = blackstone.get_texture_data();

    let planks = load_texture("oak_planks.png").await.unwrap();
    let plank_image = planks.get_texture_data();

    bricks.set_filter(FilterMode::Nearest);
    blackstone.set_filter(FilterMode::Nearest);
    planks.set_filter(FilterMode::Nearest);

    // let textures: HashMap<u32, (Texture2D, Color)> = HashMap::from([
    //     (1, (bricks, get_average_texture_color(&bricks))),
    //     (2, (blackstone, get_average_texture_color(&blackstone))),
    //     (3, (planks, get_average_texture_color(&planks))),
    // ]);

    // initial player position and rotation
    let mut player = vec2(1.5, 1.5);
    let mut player_angle: f32 = 0.0;

    // the map that the can player move through and look around in
    let map: Vec<u32> = vec!(
        2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,
        1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,
        2,2,2,2,2,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,
        2,0,0,0,0,0,2,0,0,0,0,0,2,0,0,0,0,0,0,0,2,
        2,0,2,2,2,2,2,0,2,2,2,0,2,0,2,2,2,2,2,0,2,
        2,0,2,0,0,0,0,0,0,0,2,0,2,0,2,0,0,0,0,0,2,
        2,0,2,0,2,2,2,2,2,2,2,0,2,0,2,2,2,2,2,2,2,
        2,0,2,0,0,0,0,0,2,0,0,0,2,0,2,0,0,0,0,0,2,
        2,0,2,2,2,0,2,2,2,0,2,0,2,0,2,2,2,0,2,0,2,
        2,0,0,0,2,0,2,0,0,0,2,0,2,0,0,0,2,0,2,0,2,
        2,0,2,2,2,0,2,0,2,0,2,2,2,2,2,0,2,0,2,0,2,
        2,0,0,0,0,0,2,0,2,0,2,0,0,0,0,0,2,0,2,0,2,
        2,0,2,2,2,2,2,0,2,0,2,0,2,2,2,2,2,2,2,0,2,
        2,0,2,0,0,0,2,0,2,0,2,0,0,0,0,0,2,0,0,0,2,
        2,0,2,0,2,0,2,0,2,2,2,2,2,2,2,0,2,0,2,2,2,
        2,0,0,0,2,0,2,0,0,0,0,0,2,0,0,0,2,0,0,0,2,
        2,2,2,2,2,0,2,2,2,0,2,2,2,0,2,2,2,2,2,0,2,
        2,0,2,0,0,0,2,0,2,0,0,0,0,0,2,0,0,0,0,0,2,
        2,0,2,0,2,2,2,0,2,2,2,2,2,2,2,2,2,0,2,0,2,
        2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,0,2,
        2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,3,2,
    );

    loop {
        clear_background(BLACK);
        
        // draws the map
        for (i, &val) in map.iter().enumerate() {
            let x = i % 21;
            let y = i / 21;

            if val != 0 {
                draw_rectangle(x as f32 * 20.0, y as f32 * 20.0, 19.0, 19.0, WHITE);
            }
        }

        // draws the player
        draw_circle(player.x * 20.0, player.y * 20.0, 6.0, YELLOW);

        // draws a line to show players current angle
        let player_dir = vec2(player_angle.cos(), player_angle.sin()).normalize();
        draw_line(player.x * 20.0, player.y * 20.0, (player.x + player_dir.x) * 20.0 , (player.y + player_dir.y) * 20.0, 3.0, RED);

        // moves and rotates player with user input
        if is_key_down(KeyCode::W) {
            player.x += player_dir.x * PLAYER_MOVE_SPEED * get_frame_time();
            player.y += player_dir.y * PLAYER_MOVE_SPEED * get_frame_time();
        }
        if is_key_down(KeyCode::S) {
            player.x -= player_dir.x * PLAYER_MOVE_SPEED * get_frame_time();
            player.y -= player_dir.y * PLAYER_MOVE_SPEED * get_frame_time();
        }
        
        if is_key_down(KeyCode::A) {
            player_angle -= PLAYER_TURN_SPEED * get_frame_time();
        }
        if is_key_down(KeyCode::D) {
            player_angle += PLAYER_TURN_SPEED * get_frame_time();
        }

        next_frame().await;
    }
}
