use std::f32::consts::PI;

use macroquad::prelude::*;

// const DR: f32 = 0.0174533;
const FOV: u32 = 60;
const WIDTH_3D: u32 = 1;
const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const BLOCK_SIZE: f32 = 64.0;
const PLAYER_MOVE_SPEED: f32 = 8.;
const PLAYER_TURN_SPEED: f32 = 2.0;
const VIEW_DISTANCE: f32 = 30.0;

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
    let bricks = load_texture("bricksx64.png").await.unwrap();
    
    let plane_dist: f32 = ((WIDTH as f32) / 2.) / f32::tan(f32::to_radians((FOV as f32) / 2.0));

    let mut player = vec2(16., 15.);
    let mut player_angle = 0.;
    let map_size = UVec2::new(32, 30);
    let cell_size = 5;

    let map: Vec<u32> = vec![
        1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
        1,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,1,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
        1,0,0,0,0,0,1,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,1,0,0,0,0,1,1,1,1,0,0,1,1,1,1,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,1,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,1,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,1,0,0,0,0,1,0,0,0,0,0,0,0,0,1,0,0,0,0,1,0,0,0,0,0,1,
        1,1,1,0,0,1,1,0,0,0,0,1,0,0,0,0,0,0,0,0,1,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,1,0,0,0,0,1,0,0,0,0,0,0,0,0,1,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,1,0,0,0,0,1,1,1,1,0,0,1,1,1,1,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,1,1,1,1,1,1,0,0,1,1,1,1,1,1,1,0,0,1,1,1,1,0,0,1,1,1,
        1,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
        1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
        1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
        ];

    // map.resize((map_size.x * map_size.y) as usize, 0);

    loop {
        // updates the players viewing angle
        if is_key_down(KeyCode::A) { player_angle -= PLAYER_TURN_SPEED * get_frame_time(); }
        if is_key_down(KeyCode::D) { player_angle += PLAYER_TURN_SPEED * get_frame_time(); }

        // limits the viewing angle to be between 0-2PI
        if player_angle < 0.0      { player_angle = 2.0 * PI; }
        if player_angle > 2.0 * PI { player_angle = 0.0; }

        // moves the player forward or backwards
        if is_key_down(KeyCode::W) {
            player.x += f32::cos(player_angle) * get_frame_time() * PLAYER_MOVE_SPEED;
            player.y += f32::sin(player_angle) * get_frame_time() * PLAYER_MOVE_SPEED;

            if map[((player.y as u32) * map_size.x + (player.x as u32)) as usize] == 1 {
                player.x -= f32::cos(player_angle) * get_frame_time() * PLAYER_MOVE_SPEED;
                player.y -= f32::sin(player_angle) * get_frame_time() * PLAYER_MOVE_SPEED;
            }
        }
        if is_key_down(KeyCode::S) {
            player.x -= f32::cos(player_angle) * get_frame_time() * PLAYER_MOVE_SPEED;
            player.y -= f32::sin(player_angle) * get_frame_time() * PLAYER_MOVE_SPEED;

            if map[((player.y as u32) * map_size.x + (player.x as u32)) as usize] == 1 {
                player.x += f32::cos(player_angle) * get_frame_time() * PLAYER_MOVE_SPEED;
                player.y += f32::sin(player_angle) * get_frame_time() * PLAYER_MOVE_SPEED;
            }
        }

        let starting_angle = player_angle - f32::to_radians(30 as f32);

        clear_background(BLACK);
        
        // go through 60 degrees around the player's viewing angle to find all collisions in front of them
        for i in 0..=(WIDTH / WIDTH_3D as u32) {

            let angle = starting_angle + f32::to_radians((FOV as f32 / (WIDTH as f32 / WIDTH_3D as f32)) * i as f32);

            let ray_dir = vec2(f32::cos(angle), f32::sin(angle)).normalize_or_zero();
            let ray_start = player.clone();

            let ray_unit_step_size = vec2(
                f32::sqrt(1. + (ray_dir.y / ray_dir.x) * (ray_dir.y / ray_dir.x)),
                f32::sqrt(1. + (ray_dir.x / ray_dir.y) * (ray_dir.x / ray_dir.y))
            );

            let mut map_check = ray_start.as_ivec2();
            #[allow(non_snake_case)]
            let mut ray_length_1D = Vec2::default();

            let mut step: IVec2 = IVec2::default();

            if ray_dir.x < 0. {
                step.x = -1;
                ray_length_1D.x = (ray_start.x - map_check.x as f32) * ray_unit_step_size.x;
            } else {
                step.x = 1;
                ray_length_1D.x = ((map_check.x + 1) as f32 - ray_start.x) * ray_unit_step_size.x;
            }
            
            if ray_dir.y < 0. {
                step.y = -1;
                ray_length_1D.y = (ray_start.y - map_check.y as f32) * ray_unit_step_size.y;
            } else {
                step.y = 1;
                ray_length_1D.y = ((map_check.y + 1) as f32 - ray_start.y) * ray_unit_step_size.y;
            }

            let mut tile_found = false;
            let max_distance = VIEW_DISTANCE;
            let mut distance = 0_f32;
            let mut col_y = false;
            while !tile_found && distance < max_distance {
                // walk
                if ray_length_1D.x < ray_length_1D.y {
                    map_check.x += step.x;
                    distance  = ray_length_1D.x;
                    ray_length_1D.x += ray_unit_step_size.x;
                    col_y = true;
                } else {
                    map_check.y += step.y;
                    distance  = ray_length_1D.y;
                    ray_length_1D.y += ray_unit_step_size.y;
                    col_y = false;
                }

                if map_check.x >= 0 && (map_check.x as u32) < map_size.x && map_check.y >= 0 && (map_check.y as u32) < map_size.y {
                    if map[(map_check.y * map_size.x as i32 + map_check.x) as usize] == 1 {
                        tile_found = true;
                    }
                }
            }

            // println!("{} {}", (ray_dir * distance) + player, col_y);
            // panic!();

            let end_point = ray_dir * distance + player;
            let sub_image = if col_y {
                Rect::new( end_point.y % (end_point.y as u32) as f32 * BLOCK_SIZE, 0.0, 1., BLOCK_SIZE)
            } else {
                Rect::new( end_point.x % (end_point.x as u32) as f32 * BLOCK_SIZE, 0.0, 1., BLOCK_SIZE)
            };

            let shade = 1.0 - distance/VIEW_DISTANCE;
            let color = Color::new(1. * shade, 1.0 * shade, 1.0 * shade, 1.0);

            let mut intersection = Vec2::default();
            if tile_found {
                intersection = ray_start + ray_dir * distance;
            }

            draw_line(player.x * cell_size as f32, player.y * cell_size as f32,  end_point.x * cell_size as f32, end_point.y * cell_size as f32, 1., color);

            if tile_found {
                draw_circle(intersection.x * cell_size as f32, intersection.y * cell_size as f32, 2., YELLOW);
            }

            distance = distance * f32::cos(player_angle - angle);
            
            let line_hight = (BLOCK_SIZE / (distance * BLOCK_SIZE)) * plane_dist;
            let line_offset = (HEIGHT as f32 / 2.) - line_hight/2.0;
            draw_texture_ex(
                bricks, 
                (i * WIDTH_3D) as f32, 
                line_offset,
                color, 
                DrawTextureParams {
                    dest_size: Some(vec2(WIDTH_3D as f32, line_hight)),
                    source: Some(sub_image),
                    ..Default::default()
                });
            // draw_line((i*WIDTH_3D) as f32, line_offset, (i*WIDTH_3D) as f32, line_hight + line_offset, WIDTH_3D as f32, color);
        }
        
        // draws the player
        draw_circle(player.x * cell_size as f32, player.y * cell_size as f32, 2., RED);
        let player_dir_ray = vec2(f32::cos(player_angle), f32::sin(player_angle)) + player;
        draw_line(player.x * cell_size as f32, player.y * cell_size as f32, player_dir_ray.x * cell_size as f32, player_dir_ray.y * cell_size as f32, 2., RED);

        // draw the map
        for y in 0..map_size.y {
            for x in 0..map_size.x {
                let cell = map[(y * map_size.x + x) as usize];
                if cell == 1 {
                    draw_rectangle((x * cell_size) as f32, (y * cell_size) as f32, cell_size as f32, cell_size as f32, BLUE);
                }
            }
        }

        draw_text(format!("FPS: {}", get_fps()).as_str(), 5., HEIGHT as f32 - 5., 60., WHITE);

        next_frame().await;
    }
}
