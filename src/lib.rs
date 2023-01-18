use std::{collections::HashMap, f32::consts::PI};
use macroquad::prelude::*;

const FOV: f32 = 60.; // the cameras fov in degrees
const WIDTH_3D: u32 = 1; // the width of each column when casting the rays and drawing the columns, the lower the value, the higher the resolution
const VIEW_DISTANCE: f32 = 30.; // how many grid spaces the camera can see up to
const MINIMAP_CELL_SIZE: f32 = 5.; // how big each grid space will be in the minimap in pixels
pub const BLOCK_SIZE: f32 = 64.; // the size of the textures used for the walls

pub struct RayCastEngine {
    pub map: Vec<u32>,
    pub map_size: UVec2,
    pub camera: Vec2,
    pub camera_angle: f32,
    pub textures: HashMap<u32, (Texture2D, Color)>,
    pub plane_dist: f32,
    total_num_of_cols: f32,
}

impl RayCastEngine {
    pub fn new(map: Vec<u32>, map_size: UVec2, camera: Vec2, camera_angle: f32, textures: HashMap<u32, (Texture2D, Color)>) -> Self {
        // precomputed distance of the render plane from the camera
        let plane_dist: f32 = ((screen_width() as f32) / 2.) / f32::tan(f32::to_radians(FOV / 2.));

        // the total number of rays/columns to draw
        let total_num_of_cols = (screen_width() as f32) / (WIDTH_3D as f32);

        Self {
            map,
            map_size,
            camera,
            camera_angle,
            textures,
            plane_dist,
            total_num_of_cols,
        }
    }

    pub fn draw_screen(&self) {

        draw_rectangle(0., screen_height() / 2., screen_width(), screen_height(), DARKGRAY);
        draw_rectangle(0., 0., screen_width(), screen_height() / 2., SKYBLUE);
        
        let mut texture;

        // go through the camera's FOV to find all collisions in front of them
        for i in 0..=(self.total_num_of_cols as u32) {
            texture = Texture2D::empty();

            // gets the current angle using the size of each column, the total screen size, and trigonometry
            let dist_from_middle = ((self.total_num_of_cols / 2.) - i as f32)*WIDTH_3D as f32;
            let angle_dist_from_plane = f32::sqrt(dist_from_middle.powi(2) + self.plane_dist.powi(2));
            let mut angle = f32::acos((angle_dist_from_plane.powi(2) + self.plane_dist.powi(2) - dist_from_middle.powi(2)) / (2. * angle_dist_from_plane * self.plane_dist));

            // flips angle on one side so image isn't mirrored down the middle
            if i > (self.total_num_of_cols / 2.) as u32 {
                angle = -angle;
            }

            // offsets the angle by the angle of the camera
            angle = correct_angle(self.camera_angle - angle);

            // makes a normalized vector with the current angle
            let ray_dir = vec2(f32::cos(angle), f32::sin(angle)).normalize_or_zero();

            let ray_unit_step_size = vec2(
                f32::sqrt(1. + (ray_dir.y / ray_dir.x) * (ray_dir.y / ray_dir.x)),
                f32::sqrt(1. + (ray_dir.x / ray_dir.y) * (ray_dir.x / ray_dir.y))
            );

            let mut current_map_cell = self.camera.as_ivec2();
            #[allow(non_snake_case)]
            let mut ray_length_1D = Vec2::default();

            let mut step: IVec2 = IVec2::default();

            if ray_dir.x < 0. {
                step.x = -1;
                ray_length_1D.x = (self.camera.x - current_map_cell.x as f32) * ray_unit_step_size.x;
            } else {
                step.x = 1;
                ray_length_1D.x = ((current_map_cell.x + 1) as f32 - self.camera.x) * ray_unit_step_size.x;
            }
            
            if ray_dir.y < 0. {
                step.y = -1;
                ray_length_1D.y = (self.camera.y - current_map_cell.y as f32) * ray_unit_step_size.y;
            } else {
                step.y = 1;
                ray_length_1D.y = ((current_map_cell.y + 1) as f32 - self.camera.y) * ray_unit_step_size.y;
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
                if current_map_cell.x >= 0 && current_map_cell.x < self.map_size.x as i32 && current_map_cell.y >= 0 && current_map_cell.y < self.map_size.y as i32 {
                    let current_cell = self.map[(current_map_cell.y * self.map_size.x as i32 + current_map_cell.x) as usize];
                    if current_cell > 0 {
                        if let Some(t) = self.textures.get(&current_cell) {
                            texture = t.0;
                        }
                        tile_found = true;
                    }
                }
            }

            let end_point = ray_dir * distance + self.camera;

            // uses how far into a cell the ray collided
            // to decide where to sample the texture from,
            // also flips the textures on certain walls so
            // directional textures work on any surface
            let sub_image = if col_y {
                let end_point = if step.x == -1 {
                    (BLOCK_SIZE - 1.) - (end_point.y % end_point.y.floor() * BLOCK_SIZE).floor()
                } else {
                    (end_point.y % end_point.y.floor() * BLOCK_SIZE).floor()
                };

                Rect::new( end_point, 0., 1., BLOCK_SIZE)
            } else {
                let end_point = if step.y == 1 {
                    (BLOCK_SIZE - 1.) - (end_point.x % end_point.x.floor() * BLOCK_SIZE).floor()
                } else {
                    (end_point.x % end_point.x.floor() * BLOCK_SIZE).floor()
                };
                Rect::new( end_point, 0., 1., BLOCK_SIZE)
            };

            let shade = 1. - distance/VIEW_DISTANCE;
            let color = Color::new(1. * shade, 1. * shade, 1. * shade, 1.);

            // draws a circle at the collision point if a collision occured
            if tile_found {
                let intersection = self.camera + ray_dir * distance;
                draw_circle(intersection.x * MINIMAP_CELL_SIZE as f32, intersection.y * MINIMAP_CELL_SIZE as f32, 2., YELLOW);
            }


            // draws the current raycast line on the minimap
            draw_line(self.camera.x * MINIMAP_CELL_SIZE as f32,
                self.camera.y * MINIMAP_CELL_SIZE as f32,
                end_point.x * MINIMAP_CELL_SIZE as f32,
                end_point.y * MINIMAP_CELL_SIZE as f32,
                1.,
                color);

            // removes the fisheye effect
            let rel_angle = correct_angle(self.camera_angle - angle);
            distance =  distance * f32::cos(rel_angle);
            
            let line_hight = (1. / distance) * self.plane_dist;
            let line_offset = (screen_height() as f32 / 2.) - line_hight / 2.;
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
        
        // draws the camera and its direction on the minimap
        draw_circle(self.camera.x * MINIMAP_CELL_SIZE as f32, self.camera.y * MINIMAP_CELL_SIZE as f32, 2., RED);
        
        let camera_dir_ray = vec2(f32::cos(self.camera_angle), f32::sin(self.camera_angle)) + self.camera;
        draw_line(self.camera.x * MINIMAP_CELL_SIZE as f32,
            self.camera.y * MINIMAP_CELL_SIZE as f32,
            camera_dir_ray.x * MINIMAP_CELL_SIZE as f32,
            camera_dir_ray.y * MINIMAP_CELL_SIZE as f32,
            2.,
            RED);

        // draw the minimap
        for y in 0..self.map_size.y {
            for x in 0..self.map_size.x {
                let cell = self.map[(y * self.map_size.x + x) as usize];
                if cell > 0 {
                    let color = if let Some(c) = self.textures.get(&cell) {
                        c.1
                    } else {
                        BLUE
                    };
                    draw_rectangle(x as f32 * MINIMAP_CELL_SIZE, y as f32 * MINIMAP_CELL_SIZE as f32, MINIMAP_CELL_SIZE, MINIMAP_CELL_SIZE, color);
                }
            }
        }
    }
}

pub fn correct_angle(angle: f32) -> f32 {
    if angle > 2. * PI { angle - (2. * PI) }
    else if angle < 0. { angle + (2. * PI) }
    else { angle }
}