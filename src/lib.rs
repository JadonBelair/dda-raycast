/// holds information useful when looking at a casted ray
#[derive(Default)]
pub struct RayData {
    /// the length of the ray from the starting position to when it collided
    pub ray_length: f32,

    /// the value in the map cell the ray collided with, or None if the ray did not collide
    pub hit_val: Option<u32>,

    /// the angle of the ray
    pub ray_angle: f32,

    /// the starting position of the ray
    pub ray_position: (f32, f32),

    /// a unit vector for the direction the ray traveled
    pub ray_direction: (f32, f32),

    /// boolean for if the ray collided with a vertical wall
    pub collided_horizontal: bool,
}

/// ray cast engine to hold a map and allow the user to cast rays from any point in the map
pub struct RayCastEngine {
    pub map: Vec<u32>,
    pub map_size: (usize, usize),
}

impl RayCastEngine {
    /// creates a new engine with the provided map.
    /// maps are 1D vectors so user must provide the size
    /// of the map for use during the ray cast process
    pub fn new(map: Vec<u32>, map_size: (usize, usize)) -> Self {
        Self { map, map_size }
    }

    /// casts a single ray from the given position with the
    /// given angle and returns information about the casted ray
    pub fn cast_ray(&self, pos: (f32, f32), angle: f32, max_distance: f32) -> RayData {
        // option to hold the the value in the map that the ray collided with
        let mut hit_val: Option<u32> = None;

        // makes a normalized vector with the provided angle
        let ray_dir = (f32::cos(angle), f32::sin(angle));

        // calculate each step size for the ray for each unit cell in the map
        let ray_unit_step_size = (
            f32::sqrt(1. + (ray_dir.1 / ray_dir.0) * (ray_dir.1 / ray_dir.0)),
            f32::sqrt(1. + (ray_dir.0 / ray_dir.1) * (ray_dir.0 / ray_dir.1)),
        );

        // initialize information about the ray like its starting position and length
        let mut current_map_cell = (pos.0 as i32, pos.1 as i32);
        #[allow(non_snake_case)]
        let mut ray_length_1D: (f32, f32) = (0., 0.);

        // tuple for storing the grid based x/y movement of the ray
        let mut step: (i32, i32) = (0, 0);

        // does the first step manually since the position
        // can be in a cell instead of on its edges
        if ray_dir.0 < 0. {
            step.0 = -1;
            ray_length_1D.0 = (pos.0 - current_map_cell.0 as f32) * ray_unit_step_size.0;
        } else {
            step.0 = 1;
            ray_length_1D.0 = ((current_map_cell.0 + 1) as f32 - pos.0) * ray_unit_step_size.0;
        }

        if ray_dir.1 < 0. {
            step.1 = -1;
            ray_length_1D.1 = (pos.1 - current_map_cell.1 as f32) * ray_unit_step_size.1;
        } else {
            step.1 = 1;
            ray_length_1D.1 = ((current_map_cell.1 + 1) as f32 - pos.1) * ray_unit_step_size.1;
        }

        // initialize info needed for the casting process
        let mut tile_found = false;
        let mut distance = 0_f32;
        let mut collided_horizontal = false;
        while !tile_found && distance < max_distance {
            // walk 1 unit along the ray
            // and check if the x length
            // or y length are shorter
            if ray_length_1D.0 < ray_length_1D.1 {
                // if the x length is shorter, takes 1
                // step in the x direction on the ray
                current_map_cell.0 += step.0;
                distance = ray_length_1D.0;
                ray_length_1D.0 += ray_unit_step_size.0;
                collided_horizontal = true;
            } else {
                // if the x length is shorter, takes 1
                // step in the y direction on the ray
                current_map_cell.1 += step.1;
                distance = ray_length_1D.1;
                ray_length_1D.1 += ray_unit_step_size.1;
                collided_horizontal = false;
            }

            // checks if the current cell in the map is a wall
            if current_map_cell.0 >= 0
                && current_map_cell.0 < self.map_size.0 as i32
                && current_map_cell.1 >= 0
                && current_map_cell.1 < self.map_size.1 as i32
            {
                let current_cell = self.map
                    [(current_map_cell.1 * self.map_size.0 as i32 + current_map_cell.0) as usize];
                if current_cell > 0 {
                    hit_val = Some(current_cell);
                    tile_found = true;
                }
            } else {
                // we are outside the map, break early
                break;
            }
        }

        RayData {
            ray_length: distance,
            hit_val,
            ray_angle: angle,
            ray_position: pos,
            ray_direction: ray_dir,
            collided_horizontal,
        }
    }
}
