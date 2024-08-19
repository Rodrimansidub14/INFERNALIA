use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::texture::Texture;
use crate::raycaster::cast_ray;
use std::f32::consts::PI;

 fn render_floor(
    framebuffer: &mut Framebuffer,
    floor_texture: &Texture,
    player: &Player,  // Ajustamos para incluir al jugador
) {
    let half_height = framebuffer.height as f32 / 2.0;

    for y in (half_height as usize)..framebuffer.height {
        for x in 0..framebuffer.width {
            // Aquí calculamos las coordenadas de la textura basándonos en la posición del jugador
            let texture_x = ((player.pos.x + x as f32) % floor_texture.width as f32) as usize;
            let texture_y = ((player.pos.y + y as f32) % floor_texture.height as f32) as usize;

            // Obtenemos el color de la textura en la posición calculada
            let color = floor_texture.data[texture_y * floor_texture.width + texture_x];
            framebuffer.set_current_color(color);
            framebuffer.point(x, y);
        }
    }
}

fn render_skybox(
    framebuffer: &mut Framebuffer,
    sky_texture: &Texture,
    player: &Player,
) {
    let half_height = framebuffer.height as f32 / 2.0;

    for y in 0..(half_height as usize) {
        let row_distance = player.pos.y / (half_height - y as f32);

        for x in 0..framebuffer.width {
            let sky_x = ((x as f32 / framebuffer.width as f32) * sky_texture.width as f32) as usize % sky_texture.width;
            let sky_y = (row_distance * sky_texture.height as f32) as usize % sky_texture.height;

            let color = sky_texture.data[sky_y * sky_texture.width + sky_x];
            framebuffer.set_current_color(color);
            framebuffer.point(x, y);
        }
    }
}

pub fn render3d(
    framebuffer: &mut Framebuffer,
    player: &mut Player,
    maze: &Vec<Vec<char>>,
    wall_texture: &Texture,
    floor_texture: &Texture,
    sky_texture: &Texture,
) {
    render_skybox(framebuffer, sky_texture, player);
    render_floor(framebuffer, floor_texture, player);

    let num_rays = framebuffer.width;
    let hh = framebuffer.height as f32 / 2.0;

    framebuffer.set_current_color(0xFFFFFF);

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let mut a = player.a - (player.fov / 2.0) + (player.fov * current_ray);

        // Normalizar el ángulo `a` dentro del rango de 0 a 2*PI
        if a < 0.0 {
            a += 2.0 * PI;
        } else if a >= 2.0 * PI {
            a -= 2.0 * PI;
        }

        let intersect = cast_ray(
            framebuffer,
            wall_texture,
            maze,
            player,
            a,
            false,
        );

        let distance_to_wall = intersect.distance;

        if distance_to_wall <= 0.0 || distance_to_wall.is_nan() {
            println!("Advertencia: distancia inválida para el rayo {}", i);
            continue;
        }

        let distance_to_projection_plane = 1.0;

        let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;

        let stake_top = (hh - (stake_height / 2.0)) as usize;
        let stake_bottom = (hh + (stake_height / 2.0)) as usize;

        if stake_top >= framebuffer.height || stake_bottom > framebuffer.height {
            continue;
        }

        // Mapear la textura correctamente
        let wall_x = if intersect.hit_x % 1.0 > intersect.hit_y % 1.0 {
            intersect.hit_x % 1.0
        } else {
            intersect.hit_y % 1.0
        };

        let texture_x = (wall_x * wall_texture.width as f32) as usize % wall_texture.width;

        for y in stake_top..stake_bottom {
            let texture_y = ((y as f32 - hh + stake_height / 2.0) / stake_height * wall_texture.height as f32) as usize % wall_texture.height;
            let color = wall_texture.data[texture_y * wall_texture.width + texture_x];
            framebuffer.set_current_color(color);
            framebuffer.point(i, y);
        }
    }

    // Llamar a la función que renderiza el minimapa
    render_minimap(framebuffer, player, maze);
}

pub fn render_minimap(framebuffer: &mut Framebuffer, player: &Player, maze: &Vec<Vec<char>>) {
    // Tamaño y posición del minimapa
    let minimap_size = 200;
    let minimap_x_offset = 10;
    let minimap_y_offset = 10;

    let maze_rows = maze.len();
    let maze_cols = maze[0].len();

    let block_size_x = minimap_size / maze_cols;
    let block_size_y = minimap_size / maze_rows;

    framebuffer.set_current_color(0x000000);
    for row in 0..maze_rows {
        for col in 0..maze_cols {
            let x0 = minimap_x_offset + col * block_size_x;
            let y0 = minimap_y_offset + row * block_size_y;

            match maze[row][col] {
                '#' => framebuffer.set_current_color(0xFFFFFF), // Color para las paredes
                'g' => framebuffer.set_current_color(0xFFFF00), // Color para el objetivo (goal)
                _ => framebuffer.set_current_color(0x000000),   // Color para espacios vacíos
            }

            for x in x0..(x0 + block_size_x) {
                for y in y0..(y0 + block_size_y) {
                    framebuffer.point(x, y);
                }
            }
        }
    }

    // Renderizar la posición del jugador en el minimapa
    let player_x = minimap_x_offset + ((player.pos.x * block_size_x as f32) as usize);
    let player_y = minimap_y_offset + ((player.pos.y * block_size_y as f32) as usize);

    framebuffer.set_current_color(0xFF0000);
    for x in player_x..(player_x + 3) {
        for y in player_y..(player_y + 3) {
            framebuffer.point(x, y);
        }
    }
}


pub fn render_2d(framebuffer: &mut Framebuffer, player: &Player, maze: &Vec<Vec<char>>) {
    let maze_rows = maze.len();
    let maze_cols = maze[0].len();

    let block_size_x = framebuffer.width / maze_cols;
    let block_size_y = framebuffer.height / maze_rows;

    let block_size = block_size_x.min(block_size_y);

    let offset_x = (framebuffer.width - (block_size * maze_cols)) / 2;
    let offset_y = (framebuffer.height - (block_size * maze_rows)) / 2;

    for row in 0..maze_rows {
        for col in 0..maze_cols {
            if row < maze.len() && col < maze[row].len() {
                let cell = maze[row][col];
                let x0 = offset_x + col * block_size;

                if cell == ' ' {
                    framebuffer.set_current_color(0xCCCCCC);
                } else if cell == 'p' {
                    framebuffer.set_current_color(0x00FF00);
                } else {
                    framebuffer.set_current_color(0xFF0000);
                }

                draw_cell(framebuffer, x0, offset_y + row * block_size, block_size, block_size, cell);
            } else {
                let x0 = offset_x + col * block_size;
                framebuffer.set_current_color(0x333333);
                draw_cell(framebuffer, x0, offset_y + row * block_size, block_size, block_size, ' ');
            }
        }
    }

    let player_x = offset_x + ((player.pos.x - 0.25) * block_size as f32) as usize;
    let player_y = offset_y + ((player.pos.y - 0.25) * block_size as f32) as usize;

    if player_x < framebuffer.width && player_y < framebuffer.height {
        framebuffer.set_current_color(0xFF0000);
        for x in player_x..(player_x + block_size / 4) {
            for y in player_y..(player_y + block_size / 4) {
                framebuffer.point(x, y);
            }
        }
    } else {
        println!("Player position out of bounds: x = {}, y = {}", player_x, player_y);
    }
}

fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size_x: usize, block_size_y: usize, cell: char) {
    if cell == ' ' {
        return;
    }

    framebuffer.set_current_color(0xFFDDDD);

    for x in xo..xo + block_size_x {
        if x >= framebuffer.width {
            println!("Warning: x coordinate {} out of bounds!", x);
            break;
        }
        for y in yo..yo + block_size_y {
            if y >= framebuffer.height {
                println!("Warning: y coordinate {} out of bounds!", y);
                break;
            }
            framebuffer.point(x, y);
        }
    }
}
