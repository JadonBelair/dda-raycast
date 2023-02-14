use macroquad::prelude::*;
use raycast_dda::{RayCastEngine, Map};
// use serde_json::Value;
use std::{collections::HashMap, f32::consts::PI};//, fs, io::Read};

const WIDTH: i32 = 1280; // window width
const HEIGHT: i32 = 720; // window height
const WIDTH_3D: f32 = 1.; // the width of each column when casting the rays and drawing the columns, the lower the value, the higher the resolution
const VIEW_DISTANCE: f32 = 30.; // how many grid spaces the camera can see up to
const BLOCK_SIZE: f32 = 64.; // the size of the textures used for the walls
const PLAYER_MOVE_SPEED: f32 = 8.; // the players move speed
const PLAYER_TURN_SPEED: f32 = 2.; // the player turn speed
const FOV: f32 = 60.; // the cameras fov in degrees

struct World {
    map: Vec<u32>,
    floor: Vec<u32>,
    ceil: Vec<u32>,
    map_size: (usize, usize),
}

impl World {
    pub fn get_floor(&self, x: usize, y: usize) -> Option<u32> {
        let index = y * self.map_size.0 + x;
        self.floor.get(index).copied()
    }
    
    pub fn get_ceil(&self, x: usize, y: usize) -> Option<u32> {
        let index = y * self.map_size.0 + x;
        self.ceil.get(index).copied()
    }
}

impl Map for World {
    fn get_cell(&self, x: usize, y: usize) -> Option<u32> {
        let index = y * self.map_size.0 + x;
        self.map.get(index).copied()
    }

    fn get_size(&self) -> (usize, usize) {
        self.map_size
    }
}

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
    // let mut level_data = String::new();
    // fs::File::open("test_level.ldtk")
    //     .unwrap()
    //     .read_to_string(&mut level_data)
    //     .unwrap();

    // let level: Value = serde_json::from_str(level_data.as_str()).unwrap();
    // let level: &Value = &level["levels"][0]["layerInstances"];

    let sky = load_texture("sky.png").await.unwrap();
    let sky_width = screen_width();
    let sky_height = sky_width / 2.;

    let bricks = load_texture("bricks.png").await.unwrap();
    let bricks_image = bricks.get_texture_data();

    let blackstone = load_texture("polished_blackstone_bricks.png").await.unwrap();
    let blackstone_image = blackstone.get_texture_data();

    let planks = load_texture("oak_planks.png").await.unwrap();
    let plank_image = planks.get_texture_data();

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
        1.5,//level[0]["entityInstances"][0]["__grid"][0].as_f64().unwrap() as f32 + 0.5,
        1.5//level[0]["entityInstances"][0]["__grid"][1].as_f64().unwrap() as f32 + 0.5,
    );
    let mut player_angle = 0.;

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
        2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,3,2
    );

    let floor: Vec<u32> = vec!(
        3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,
        3,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
        3,3,3,3,3,1,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,
        3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,
        3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,
        3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,
        3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,
        3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,
        3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,
        3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,
        3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,
        3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,
        3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,
        3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,
        3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,
        3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,2,3,
        3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,2,3,
        3,2,3,3,3,3,3,3,3,3,3,3,3,3,3,2,2,2,2,2,3,
        3,2,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,2,3,2,3,
        3,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,3,2,3,
        3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3
    );

    let ceil: Vec<u32> = vec!(
        2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,
        1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,
        2,2,2,2,2,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,
        2,2,0,0,0,0,2,0,0,0,0,0,2,0,0,0,0,0,0,0,2,
        2,2,2,2,2,2,2,0,2,2,2,0,2,0,2,2,2,2,2,0,2,
        2,2,2,0,0,0,0,0,0,0,2,0,2,0,2,0,0,0,0,0,2,
        2,2,2,0,2,2,2,2,2,2,2,0,2,0,2,2,2,2,2,2,2,
        2,2,2,0,0,0,0,0,2,0,0,0,2,0,2,0,0,0,0,0,2,
        2,2,2,2,2,0,2,2,2,0,2,0,2,0,2,2,2,0,2,0,2,
        2,0,0,0,2,0,2,0,0,0,2,0,2,0,0,0,2,0,2,0,2,
        2,0,2,2,2,0,2,0,2,0,2,2,2,2,2,0,2,0,2,0,2,
        2,0,0,0,0,0,2,0,2,0,2,0,0,0,0,0,2,0,2,0,2,
        2,1,2,2,2,2,2,0,2,0,2,0,2,2,2,2,2,2,2,0,2,
        2,1,2,2,2,2,2,0,2,0,2,0,0,0,0,0,2,0,0,0,2,
        2,1,2,2,2,0,2,0,2,2,2,2,2,2,2,0,2,0,2,2,2,
        2,2,2,2,2,0,2,0,0,0,0,0,2,0,0,0,2,3,3,3,2,
        2,2,2,2,2,0,2,2,2,0,2,2,2,0,2,2,2,2,2,3,2,
        2,1,2,3,3,3,2,3,2,0,0,0,0,0,2,1,1,1,1,3,2,
        2,1,2,3,2,2,2,3,2,2,2,2,2,2,2,2,2,1,2,3,2,
        2,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,2,3,2,
        2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,3,2
    );
    //level[1]["intGridCsv"]
        // .as_array()
        // .unwrap()
        // .iter()
        // .map(|v| v.as_u64().unwrap() as u32)
        // .collect();

    // the size of the map
    let map_size = (
        21,//level[1]["__cWid"].as_u64().unwrap() as usize,
        21//level[1]["__cHei"].as_u64().unwrap() as usize,
    );

    let map = World {
        map,
        floor,
        ceil,
        map_size
    };

    // precomputed distance of the render plane from the camera
    let plane_dist: f32 = (screen_width() / 2.) / (FOV / 2.).to_radians().tan();

    // the total number of rays/columns to draw
    let total_num_of_cols = screen_width() / WIDTH_3D as f32;

    let engine = RayCastEngine::new(map, map_size);

    // the texture to draw each column
    let mut texture;

    let mut floor_image = Image::gen_image_color(WIDTH as u16, HEIGHT as u16, Color::new(0., 0., 0., 0.));
    let floor_tex = Texture2D::from_image(&floor_image);

    // used in some way for rendering the floor
    // im not super clear on what this does and the formula
    // ive come up with doesnt seem to be perfect, but
    // its close enough for the most part so ill leave it for now
    let ar = (plane_dist / 4.).floor();

    loop {
        floor_image = Image::gen_image_color(WIDTH as u16, HEIGHT as u16, Color::new(0., 0., 0., 0.));
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

            if engine.map.get_cell(target_x as usize, player.1 as usize).unwrap() == 0 {//[(player.1 as usize) * engine.map_size.0 + (target_x as usize)] == 0 {
                player.0 = target_x;
            } else {
                player.0 = target_x.floor() + if player.0 < target_x { -0.01 } else { 1.01 };
            }

            if engine.map.get_cell(player.0 as usize, target_y as usize).unwrap() == 0 {//[(target_y as usize) * engine.map_size.0 + (player.0 as usize)] == 0 {
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

            if engine.map.get_cell(target_x as usize, player.1 as usize).unwrap() == 0 {
                player.0 = target_x;
            } else {
                player.0 = target_x.floor() + if player.0 < target_x { -0.01 } else { 1.01 };
            }

            if engine.map.get_cell(player.0 as usize, target_y as usize).unwrap() == 0 {
                player.1 = target_y;
            } else {
                player.1 = target_y.floor() + if player.1 < target_y { -0.01 } else { 1.01 };
            }
        }

        let sky_start_x = ((correct_angle(-player_angle*5.5) / (2. * PI)) * sky_width) - sky_width;
        draw_texture_ex(sky, sky_start_x, 0., WHITE, DrawTextureParams { dest_size: Some(vec2(sky_width, sky_height)), ..Default::default() });

        if sky_start_x > 0. {
            draw_texture_ex(sky, sky_start_x - sky_width, 0., WHITE, DrawTextureParams { dest_size: Some(vec2(sky_width, sky_height)), ..Default::default() });
        } else {
            draw_texture_ex(sky, sky_start_x + sky_width, 0., WHITE, DrawTextureParams { dest_size: Some(vec2(sky_width, sky_height)), ..Default::default() });
        }


        for i in 0..(total_num_of_cols as u32) {
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
            let coord = if ray_data.collided_vertical {
                end_point.1
            } else {
                end_point.0
            };

            // checks if the texture will need to be flipped depending on what direction the ray is facing
            let flip_check = if ray_data.collided_vertical {
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

            let line_hight = plane_dist / distance;
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

            // draws the floor for the current column
            for y in (line_offset as u32 + line_hight as u32)..(screen_height() as u32) {
                let dy = (y - (HEIGHT as u32 / 2)) as f32;
                let mut tx = ((player.0 * BLOCK_SIZE) / 2. + angle.cos() * ar * 64. / dy / rel_angle.cos()) * 2.;
                let mut ty = ((player.1 * BLOCK_SIZE) / 2. + angle.sin() * ar * 64. / dy / rel_angle.cos()) * 2.;

                if tx < 0. {
                    tx += 64.;
                }
                if ty < 0. {
                    ty += 64.;
                }

                let floor_col = if let Some(v) = engine.map.get_floor((tx / 64.) as usize, (ty / 64.) as usize) {
                    match v {
                        1 => bricks_image.get_pixel(tx as u32 % 64, ty as u32 % 64),
                        2 => blackstone_image.get_pixel(tx as u32 % 64, ty as u32 % 64),
                        3 => plank_image.get_pixel(tx as u32 % 64, ty as u32 % 64),
                        _ => plank_image.get_pixel(tx as u32 % 64, ty as u32 % 64)
                    }
                } else {
                    plank_image.get_pixel(tx as u32 % 64, ty as u32 % 64)
                };

                let ceil_col = if let Some(v) = engine.map.get_ceil((tx / 64.) as usize, (ty / 64.) as usize) {
                    match v {
                        1 => bricks_image.get_pixel(tx as u32 % 64, ty as u32 % 64),
                        2 => blackstone_image.get_pixel(tx as u32 % 64, ty as u32 % 64),
                        3 => plank_image.get_pixel(tx as u32 % 64, ty as u32 % 64),
                        _ => plank_image.get_pixel(tx as u32 % 64, ty as u32 % 64)
                    }
                } else {
                    plank_image.get_pixel(tx as u32 % 64, ty as u32 % 64)
                };

                // adds shading to the floor/ceiling depending on how close it is to the center of the screen
                let shade = ((y as f32 - screen_height() / 2.) / (screen_height() / 2.) + 0.3).clamp(0., 1.);
                let floor_col = Color::new(floor_col.r * shade, floor_col.g * shade, floor_col.b * shade, 1.);
                let ceil_col = Color::new(ceil_col.r * shade, ceil_col.g * shade, ceil_col.b * shade, 1.);

                // draws the found floor color to the screen
                floor_image.set_pixel(i, y, floor_col);

                // only draws the ceiling if there was a texture to draw
                // otherwise it is left blank for the sky to show
                if engine.map.get_ceil((tx / 64.) as usize, (ty / 64.) as usize).unwrap() != 0 {
                    floor_image.set_pixel(i, screen_height() as u32 - y, ceil_col);
                }
            }
        }

        // updates the texture with all the floor pixels
        floor_tex.update(&floor_image);

        // draws the generated floor
        draw_texture_ex(floor_tex, 0., 0., WHITE, DrawTextureParams {
            dest_size: Some(vec2(screen_width() * WIDTH_3D, screen_height())),
            ..Default::default()
        });

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
        let angle = angle - (2. * PI);
        correct_angle(angle)
    } else if angle < 0. {
        let angle = angle + (2. * PI);
        correct_angle(angle)
    } else {
        angle
    }
}
