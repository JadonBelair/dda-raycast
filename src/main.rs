use macroquad::prelude::*;
use raycast_dda::RayCastEngine;
use serde_json::Value;
use std::{collections::HashMap, f32::consts::PI, fs, io::Read};

const WIDTH: i32 = 1280; // window width
const HEIGHT: i32 = 720; // window height
const WIDTH_3D: f32 = 1.; // the width of each column when casting the rays and drawing the columns, the lower the value, the higher the resolution
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
    let mut level_data = String::new();
    fs::File::open("test_level.ldtk")
        .unwrap()
        .read_to_string(&mut level_data)
        .unwrap();

    let level: Value = serde_json::from_str(level_data.as_str()).unwrap();
    let level: &Value = &level["levels"][0]["layerInstances"];

    let bricks = load_texture("bricks.png").await.unwrap();
    let blackstone = load_texture("polished_blackstone_bricks.png").await.unwrap();
    let planks = load_texture("oak_planks.png").await.unwrap();

    bricks.set_filter(FilterMode::Nearest);
    blackstone.set_filter(FilterMode::Nearest);
    planks.set_filter(FilterMode::Nearest);

    let textures: HashMap<u32, (Texture2D, Color)> = HashMap::from([
        (1, (bricks, get_average_texture_color(&bricks))),
        (2, (blackstone, get_average_texture_color(&blackstone))),
        (3, (planks, get_average_texture_color(&planks))),
    ]);

    // initial player position and rotation
    let mut player = (
        level[0]["entityInstances"][0]["__grid"][0].as_f64().unwrap() as f32 + 0.5,
        level[0]["entityInstances"][0]["__grid"][1].as_f64().unwrap() as f32 + 0.5,
    );
    let mut player_angle = 0.;

    // the map that the can player move through and look around in
    let map: Vec<u32> = level[1]["intGridCsv"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u32)
        .collect();

    // the size of the map
    let map_size = (
        level[1]["__cWid"].as_u64().unwrap() as usize,
        level[1]["__cHei"].as_u64().unwrap() as usize,
    );

    // precomputed distance of the render plane from the camera
    let plane_dist: f32 = (screen_width() / 2.) / (FOV / 2.).to_radians().tan();

    // the total number of rays/columns to draw
    let total_num_of_cols = screen_width() / WIDTH_3D as f32;

    let engine = RayCastEngine::new(map, map_size);

    // the texture to draw each column
    let mut texture;
    loop {
        clear_background(BLACK);
        let delta_time = get_frame_time();

        // updates the players viewing angle
        if is_key_down(KeyCode::A) {
            player_angle -= PLAYER_TURN_SPEED * delta_time;
        }
        if is_key_down(KeyCode::D) {
            player_angle += PLAYER_TURN_SPEED * delta_time;
        }

        // limits the viewing angle to be between 0-2PI
        if player_angle < 0. {
            player_angle = 2. * PI;
        }
        if player_angle > 2. * PI {
            player_angle = 0.;
        }

        // moves the player forward or backwards
        // with super basic collision detection
        if is_key_down(KeyCode::W) {
            let x_move = player_angle.cos() * delta_time * PLAYER_MOVE_SPEED;
            let y_move = player_angle.sin() * delta_time * PLAYER_MOVE_SPEED;

            let target_x = player.0 + x_move;
            let target_y = player.1 + y_move;

            if engine.map[(player.1 as usize) * engine.map_size.0 + (target_x as usize)] == 0 {
                player.0 = target_x;
            } else {
                player.0 = target_x.floor() + if player.0 < target_x { -0.01 } else { 1.01 };
            }

            if engine.map[(target_y as usize) * engine.map_size.0 + (player.0 as usize)] == 0 {
                player.1 = target_y;
            } else {
                player.1 = target_y.floor() + if player.1 < target_y { -0.01 } else { 1.01 };
            }
        }
        if is_key_down(KeyCode::S) {
            let x_move = player_angle.cos() * delta_time * PLAYER_MOVE_SPEED;
            let y_move = player_angle.sin() * delta_time * PLAYER_MOVE_SPEED;

            let target_x = player.0 - x_move;
            let target_y = player.1 - y_move;

            if engine.map[(player.1 as usize) * engine.map_size.0 + (target_x as usize)] == 0 {
                player.0 = target_x;
            } else {
                player.0 = target_x.floor() + if player.0 < target_x { -0.01 } else { 1.01 };
            }

            if engine.map[(target_y as usize) * engine.map_size.0 + (player.0 as usize)] == 0 {
                player.1 = target_y;
            } else {
                player.1 = target_y.floor() + if player.1 < target_y { -0.01 } else { 1.01 };
            }
        }

        draw_rectangle(0., 0., screen_width(), screen_height() / 2., SKYBLUE);
        draw_rectangle(
            0.,
            screen_height() / 2.,
            screen_width(),
            screen_height() / 2.,
            DARKGRAY,
        );

        for i in 0..=(total_num_of_cols as u32) {
            texture = Texture2D::empty();

            // gets the current angle using the size of each column, the total screen size, and trigonometry
            let dist_from_middle = ((total_num_of_cols / 2.) - i as f32) * WIDTH_3D as f32;
            let angle_dist_from_plane = (dist_from_middle.powi(2) + plane_dist.powi(2)).sqrt();
            let mut angle = (dist_from_middle / angle_dist_from_plane).asin();

            // offsets the angle by the angle of the camera
            angle = correct_angle(player_angle - angle);

            let ray_data = engine.cast_ray(player, angle, VIEW_DISTANCE);

            // uses the rays direction and length to calculate where the collision occured
            let end_point = (
                ray_data.ray_direction.0 * ray_data.ray_length + ray_data.ray_position.0,
                ray_data.ray_direction.1 * ray_data.ray_length + ray_data.ray_position.1,
            );

            // checks if the ray collided with
            // anything and if so, get its texture
            match ray_data.hit_val {
                Some(val) => {
                    texture = textures.get(&val).unwrap().0;
                }
                None => (),
            }

            // uses how far into a cell the ray collided
            // to decide where to sample the texture from,
            // also flips the textures on certain walls so
            // directional textures work on any surface

            // gets the x or y coord depending on if the collision was on the y-axis or not
            let coord = if ray_data.collided_horizontal {
                end_point.1
            } else {
                end_point.0
            };

            // checks if the texture will need to be flipped depending on what direction the ray is facing
            let flip_check = if ray_data.collided_horizontal {
                ray_data.ray_direction.0 < 0.
            } else {
                ray_data.ray_direction.1 > 0.
            };

            // gets the column that the sub image will occupy
            let texture_col = if flip_check {
                // gets the column from the back if the texture needs to be flipped
                (BLOCK_SIZE - 1.) - (coord % coord.floor() * BLOCK_SIZE).floor()
            } else {
                // gets column from front otherwise
                (coord % coord.floor() * BLOCK_SIZE).floor()
            };

            // builds a 1 pixel column for the texture sampling later
            let sub_image = Rect::new(texture_col, 0., 1., BLOCK_SIZE);

            // makes the texture drawn darker the farther away it is
            let shade = 1. - ray_data.ray_length / VIEW_DISTANCE;
            let color = Color::new(1. * shade, 1. * shade, 1. * shade, 1.);

            // removes the fisheye effect
            let rel_angle = correct_angle(player_angle - angle);
            let distance = ray_data.ray_length * f32::cos(rel_angle);

            let line_hight = (1. / distance) * plane_dist;
            let line_offset = (screen_height() as f32 / 2.) - line_hight / 2.;
            draw_texture_ex(
                texture,
                i as f32 * WIDTH_3D,
                line_offset,
                color,
                DrawTextureParams {
                    dest_size: Some(vec2(WIDTH_3D as f32, line_hight)),
                    source: Some(sub_image),
                    ..Default::default()
                },
            );
        }

        draw_text(
            format!("FPS: {}", get_fps()).as_str(),
            5.,
            HEIGHT as f32 - 5.,
            60.,
            WHITE,
        );

        next_frame().await;
    }
}

fn get_average_texture_color(texture: &Texture2D) -> Color {
    let width = texture.width() as u32;
    let height = texture.height() as u32;

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

    Color::new(
        total_r / (width * height) as f32,
        total_g / (width * height) as f32,
        total_b / (width * height) as f32,
        1.,
    )
}

pub fn correct_angle(angle: f32) -> f32 {
    if angle > 2. * PI {
        angle - (2. * PI)
    } else if angle < 0. {
        angle + (2. * PI)
    } else {
        angle
    }
}
