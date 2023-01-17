use std::f32::consts::PI;
use macroquad::prelude::*;

const FOV: f32 = 60.; // the players fov in degrees
const WIDTH_3D: u32 = 1; // the width of each column when casting the rays and drawing the columns, the lower the value, the higher the resolution
const WIDTH: u32 = 1920; // window width
const HEIGHT: u32 = 1080; // window height
const BLOCK_SIZE: f32 = 64.; // the size of the textures used for the walls
const PLAYER_MOVE_SPEED: f32 = 8.; // the players move speed
const PLAYER_TURN_SPEED: f32 = 2.; // the player turn speed
const VIEW_DISTANCE: f32 = 30.; // how many grid spaces the player can see up to
const MINIMAP_CELL_SIZE: f32 = 5.; // how big each grid space will be in the minimap
const TOTAL_NUM_OF_COLS: f32 = (WIDTH as f32) / (WIDTH_3D as f32); // the total number of colums, uses the width of the window and the size of each column
const ANGLE_INCREMENT: f32 = FOV / TOTAL_NUM_OF_COLS; // determines the angle increment between each column

fn window_conf() -> Conf {
    Conf {
        window_title: "RayCast Test".to_owned(),
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        fullscreen: true,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let bricks = load_texture("bricks.png").await.unwrap();
    let blackstone = load_texture("polished_blackstone_bricks.png").await.unwrap();
    let planks = load_texture("oak_planks.png").await.unwrap();
    
    bricks.set_filter(FilterMode::Nearest);
    blackstone.set_filter(FilterMode::Nearest);
    planks.set_filter(FilterMode::Nearest);

    let cols: Vec<Color> = vec![
        get_average_texture_color(&bricks, BLOCK_SIZE, BLOCK_SIZE),
        get_average_texture_color(&blackstone, BLOCK_SIZE, BLOCK_SIZE),
        get_average_texture_color(&planks, BLOCK_SIZE, BLOCK_SIZE),
    ];
    
    // precomputes the distance of the render plane from the player
    let plane_dist: f32 = ((WIDTH as f32) / 2.) / f32::tan(f32::to_radians(FOV / 2.));

    // initial player position and rotation
    let mut player = vec2(16., 15.);
    let mut player_angle = 0.;

    // the size of the map
    let map_size = UVec2::new(32, 30);

    // the map that the player move through and look around in
    let map: Vec<u8> = vec![
        3,3,3,3,3,3,3,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
        3,0,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,
        3,0,0,0,0,0,3,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
        3,0,0,0,0,0,3,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,
        3,0,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,
        3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,
        3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,
        3,0,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,
        3,0,0,3,3,3,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,3,0,0,0,0,2,2,2,2,0,0,2,2,2,2,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,0,2,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,0,2,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,1,0,0,0,0,2,0,0,0,0,0,0,0,0,2,0,0,0,0,1,0,0,0,0,0,1,
        1,1,1,0,0,1,1,0,0,0,0,2,0,0,0,0,0,0,0,0,2,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,1,0,0,0,0,2,0,0,0,0,0,0,0,0,2,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,1,0,0,0,0,2,2,2,2,0,0,2,2,2,2,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,2,2,2,2,2,2,0,0,2,2,2,2,3,3,3,0,0,3,3,3,1,0,0,1,1,1,
        1,0,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,3,0,0,0,0,0,1,
        1,0,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,0,0,0,0,0,1,
        1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
        1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
        1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,3,1,1,1,1,1,1,
        ];

    loop {
        clear_background(BLACK);
        let delta_time = get_frame_time();

        // updates the players viewing angle
        if is_key_down(KeyCode::A) { player_angle -= PLAYER_TURN_SPEED * delta_time; }
        if is_key_down(KeyCode::D) { player_angle += PLAYER_TURN_SPEED * delta_time; }

        // limits the viewing angle to be between 0-2PI
        if player_angle < 0.      { player_angle = 2. * PI; }
        if player_angle > 2. * PI { player_angle = 0.; }

        // moves the player forward or backwards
        // with super basic collision detection
        if is_key_down(KeyCode::W) {
            player.x += f32::cos(player_angle) * delta_time * PLAYER_MOVE_SPEED;
            player.y += f32::sin(player_angle) * delta_time * PLAYER_MOVE_SPEED;

            if map[((player.y as u32) * map_size.x + (player.x as u32)) as usize] > 0 {
                player.x -= f32::cos(player_angle) * delta_time * PLAYER_MOVE_SPEED;
                player.y -= f32::sin(player_angle) * delta_time * PLAYER_MOVE_SPEED;
            }
        }
        if is_key_down(KeyCode::S) {
            player.x -= f32::cos(player_angle) * delta_time * PLAYER_MOVE_SPEED;
            player.y -= f32::sin(player_angle) * delta_time * PLAYER_MOVE_SPEED;

            if map[((player.y as u32) * map_size.x + (player.x as u32)) as usize] > 0 {
                player.x += f32::cos(player_angle) * delta_time * PLAYER_MOVE_SPEED;
                player.y += f32::sin(player_angle) * delta_time * PLAYER_MOVE_SPEED;
            }
        }

        // subtracts half the FOV from the current player angle 
        let starting_angle = player_angle - f32::to_radians(FOV / 2.);
        // go through the player's FOV to find all collisions in front of them
        for i in 0..=(TOTAL_NUM_OF_COLS as u32) {
            let mut texture = Texture2D::empty();
            
            // gets the current angle using the size of each column and the total screen size
            let angle = starting_angle + f32::to_radians(ANGLE_INCREMENT * i as f32);

            // makes a normalized vector with the current angle
            let ray_dir = vec2(f32::cos(angle), f32::sin(angle)).normalize_or_zero();

            let ray_unit_step_size = vec2(
                f32::sqrt(1. + (ray_dir.y / ray_dir.x) * (ray_dir.y / ray_dir.x)),
                f32::sqrt(1. + (ray_dir.x / ray_dir.y) * (ray_dir.x / ray_dir.y))
            );

            let mut current_map_cell = player.as_ivec2();
            #[allow(non_snake_case)]
            let mut ray_length_1D = Vec2::default();

            let mut step: IVec2 = IVec2::default();

            if ray_dir.x < 0. {
                step.x = -1;
                ray_length_1D.x = (player.x - current_map_cell.x as f32) * ray_unit_step_size.x;
            } else {
                step.x = 1;
                ray_length_1D.x = ((current_map_cell.x + 1) as f32 - player.x) * ray_unit_step_size.x;
            }
            
            if ray_dir.y < 0. {
                step.y = -1;
                ray_length_1D.y = (player.y - current_map_cell.y as f32) * ray_unit_step_size.y;
            } else {
                step.y = 1;
                ray_length_1D.y = ((current_map_cell.y + 1) as f32 - player.y) * ray_unit_step_size.y;
            }

            let mut tile_found = false;
            let max_distance = VIEW_DISTANCE;
            let mut distance = 0_f32;
            let mut col_y = false;
            while !tile_found && distance < max_distance {
                // walk 1 unit along the ray
                // and check if the x length
                // or y length are longer
                if ray_length_1D.x < ray_length_1D.y {
                    current_map_cell.x += step.x;
                    distance  = ray_length_1D.x;
                    ray_length_1D.x += ray_unit_step_size.x;
                    col_y = true;
                } else {
                    current_map_cell.y += step.y;
                    distance  = ray_length_1D.y;
                    ray_length_1D.y += ray_unit_step_size.y;
                    col_y = false;
                }

                // checks if the current cell in the map is a wall
                if current_map_cell.x >= 0 && current_map_cell.x < map_size.x as i32 && current_map_cell.y >= 0 && current_map_cell.y < map_size.y as i32 {
                    let current_cell = map[(current_map_cell.y * map_size.x as i32 + current_map_cell.x) as usize];
                    if current_cell > 0 {
                        match current_cell {
                            1 => texture = bricks,
                            2 => texture = blackstone,
                            3 => texture = planks,
                            _ => ()
                        }
                        tile_found = true;
                    }
                }
            }

            let end_point = ray_dir * distance + player;

            // uses how far into a cell the ray collided to decide where to sample the texture from
            let sub_image = if col_y {
                Rect::new( (end_point.y % end_point.y.floor() * BLOCK_SIZE).floor(), 0., 1., BLOCK_SIZE)
            } else {
                Rect::new( (end_point.x % end_point.x.floor() * BLOCK_SIZE).floor(), 0., 1., BLOCK_SIZE)
            };

            let shade = 1. - distance/VIEW_DISTANCE;
            let color = Color::new(1. * shade, 1. * shade, 1. * shade, 1.);

            // draws a circle at the collision point if a collision occured
            if tile_found {
                let intersection = player + ray_dir * distance;
                draw_circle(intersection.x * MINIMAP_CELL_SIZE as f32, intersection.y * MINIMAP_CELL_SIZE as f32, 2., YELLOW);
            }


            // draws the current raycast line on the minimap
            draw_line(player.x * MINIMAP_CELL_SIZE as f32,
                player.y * MINIMAP_CELL_SIZE as f32,
                end_point.x * MINIMAP_CELL_SIZE as f32,
                end_point.y * MINIMAP_CELL_SIZE as f32,
                1.,
                color);

            // removes the fisheye effect
            distance = distance * f32::cos(player_angle - angle);
            
            let line_hight = (1. / distance) * plane_dist;
            let line_offset = (HEIGHT as f32 / 2.) - line_hight / 2.;
            draw_texture_ex(
                texture, 
                (i * WIDTH_3D) as f32, 
                line_offset,
                color, 
                DrawTextureParams {
                    dest_size: Some(vec2(WIDTH_3D as f32, line_hight)),
                    source: Some(sub_image),
                    ..Default::default()
                });
        }
        
        // draws the player and its direction on the minimap
        draw_circle(player.x * MINIMAP_CELL_SIZE as f32, player.y * MINIMAP_CELL_SIZE as f32, 2., RED);
        
        let player_dir_ray = vec2(f32::cos(player_angle), f32::sin(player_angle)) + player;
        draw_line(player.x * MINIMAP_CELL_SIZE as f32,
            player.y * MINIMAP_CELL_SIZE as f32,
            player_dir_ray.x * MINIMAP_CELL_SIZE as f32,
            player_dir_ray.y * MINIMAP_CELL_SIZE as f32,
            2.,
            RED);

        // draw the minimap
        for y in 0..map_size.y {
            for x in 0..map_size.x {
                let cell = map[(y * map_size.x + x) as usize];
                if cell > 0 {
                    let color = cols[(cell - 1) as usize];
                    draw_rectangle(x as f32 * MINIMAP_CELL_SIZE, y as f32 * MINIMAP_CELL_SIZE as f32, MINIMAP_CELL_SIZE, MINIMAP_CELL_SIZE, color);
                }
            }
        }

        draw_text(format!("FPS: {}", get_fps()).as_str(), 5., HEIGHT as f32 - 5., 60., WHITE);

        next_frame().await;
    }
}

fn get_average_texture_color(texture: &Texture2D, width: f32, height: f32) -> Color {
    let image = texture.get_texture_data();
    let mut total_r = 0.;
    let mut total_g = 0.;
    let mut total_b = 0.;
    for y in 0..(BLOCK_SIZE as u32) {
        for x in 0..(BLOCK_SIZE as u32) {
            let pixel = image.get_pixel(x, y);
            total_r += pixel.r;
            total_g += pixel.g;
            total_b += pixel.b;
        }
    }

    Color::new(total_r / (width * height), total_g / (width * height), total_b / (width * height), 1.)
}