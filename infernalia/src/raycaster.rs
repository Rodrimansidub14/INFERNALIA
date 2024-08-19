use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::texture::Texture;
use std::f32::consts::PI;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
    pub hit_x: f32,  // Coordenada x del impacto en el mundo
    pub hit_y: f32,  // Coordenada y del impacto en el mundo
}

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    wall_texture: &Texture,
    maze: &Vec<Vec<char>>,
    player: &Player,
    ray_angle: f32,
    draw_line: bool,
) -> Intersect {
    let mut distance = 0.0;
    let mut hit = false;
    let mut impact = ' ';
    let (cos_angle, sin_angle) = (ray_angle.cos(), ray_angle.sin());

    let mut hit_x = 0.0;
    let mut hit_y = 0.0;

    while !hit {
        distance += 0.1;
        let x = player.pos.x + cos_angle * distance;
        let y = player.pos.y + sin_angle * distance;

        if x as usize >= maze[0].len() || y as usize >= maze.len() {
            break; // El rayo ha salido de los límites del laberinto
        }

        impact = maze[y as usize][x as usize];
        if impact != ' ' {
            hit = true;
            hit_x = x;
            hit_y = y;
        }

        if draw_line {
            framebuffer.point(x as usize, y as usize); // Opcional para dibujar la línea en el minimapa
        }
    }

    Intersect {
        distance,
        impact,
        hit_x,
        hit_y,
    }
}

fn render_skybox(framebuffer: &mut Framebuffer, sky_texture: &Texture, player_angle: f32) {
    let sky_width = sky_texture.width as f32;
    let sky_height = sky_texture.height as f32;

    for x in 0..framebuffer.width {
        let sky_x = (((player_angle + (x as f32 / framebuffer.width as f32) * 2.0 * PI) % (2.0 * PI)) / (2.0 * PI) * sky_width) as usize;
        for y in 0..(framebuffer.height / 2) {
            let sky_y = (y as f32 / (framebuffer.height as f32 / 2.0) * sky_height) as usize;
            let color = sky_texture.data[sky_y * sky_texture.width + sky_x];
            framebuffer.set_current_color(color);
            framebuffer.point(x, y);
        }
    }
}

fn render_floor(framebuffer: &mut Framebuffer, floor_texture: &Texture, player: &Player) {
    let floor_width = floor_texture.width as f32;
    let floor_height = floor_texture.height as f32;
    let half_height = framebuffer.height / 2;

    for y in half_height..framebuffer.height {
        let distance = (player.pos.y as f32) / ((y as f32 - half_height as f32) / half_height as f32);
        let ray_dir_x0 = player.a - player.fov / 2.0;
        let ray_dir_x1 = player.a + player.fov / 2.0;

        for x in 0..framebuffer.width {
            let camera_x = 2.0 * x as f32 / framebuffer.width as f32 - 1.0;
            let ray_dir_x = ray_dir_x0 + ray_dir_x1 * camera_x;

            let floor_x = (player.pos.x + distance * ray_dir_x) % floor_width;
            let floor_y = (player.pos.y + distance * ray_dir_x) % floor_height;

            let texture_x = (floor_x * floor_width as f32) as usize % floor_texture.width;
            let texture_y = (floor_y * floor_height as f32) as usize % floor_texture.height;

            let color = floor_texture.data[texture_y * floor_texture.width + texture_x];
            framebuffer.set_current_color(color);
            framebuffer.point(x, y);
        }
    }
    
}


