mod framebuffer;
mod maze;
mod player;
mod raycaster;
mod map;
mod texture;
mod render;
mod sfx;

use minifb::{Key, Window, WindowOptions};
use std::f32::consts::PI;
use std::time::{Duration, Instant};
use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::maze::generate_and_save_maze;
use crate::map::load_maze;
use crate::texture::Texture;
use crate::render::{render_2d, render3d};
use std::fs::{self};
use rusttype::{Font, Scale};
use sfx::SoundManager;

enum ViewMode {
    View2D,
    View3D,
}

fn find_start_position(maze: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            if cell == 'p' && maze[row_index][col_index] == ' ' {
                return Some((col_index, row_index));
            }
        }
    }
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            if cell == ' ' {
                return Some((col_index, row_index));
            }
        }
    }
    None
}

fn draw_overlay(framebuffer: &mut Framebuffer, opacity: u8) {
    let overlay_color = (opacity as u32) << 24; // Semitransparente negro

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            framebuffer.buffer[y * framebuffer.width + x] = overlay_color;
        }
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width: usize = 800;
    let framebuffer_height = 600;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Maze Runner",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    framebuffer.set_background_color(0x333355);

    // Load the font
    let font_data = fs::read("assets/fonts/AGaramondPro-Regular.otf").expect("Unable to read font file");
    let font = Font::try_from_vec(font_data).expect("Error constructing Font");

    // Initialize SoundManager and play ambient sound
    let sound_manager = SoundManager::new();
    sound_manager.play_ambient("assets/music/ambience.mp3");

    // Generar y guardar el mapa proceduralmente
    let maze_filename = "./assets/generated_maze.txt";
    let (maze_width, maze_height, goal_x, goal_y) = generate_and_save_maze(10, 8, maze_filename, 5); // Dimensiones del laberinto
    let mut maze = load_maze(maze_filename);  // Hacer `maze` mutable
    println!("Maze generated and loaded successfully.");

    // Load textures
    let wall_texture = match Texture::load_from_file("./assets/textures/walls.jpg") {
        Ok(texture) => {
            println!("Wall texture loaded: {}x{}", texture.width, texture.height);
            texture
        },
        Err(e) => {
            println!("Failed to load wall texture: {}", e);
            return;
        }
    };

    let floor_texture = match Texture::load_from_file("./assets/textures/floor.jpg") {
        Ok(texture) => {
            println!("Floor texture loaded: {}x{}", texture.width, texture.height);
            texture
        },
        Err(e) => {
            println!("Failed to load floor texture: {}", e);
            return;
        }
    };

    let sky_texture = match Texture::load_from_file("./assets/textures/sky.jpg") {
        Ok(texture) => {
            println!("Sky texture loaded: {}x{}", texture.width, texture.height);
            texture
        },
        Err(e) => {
            println!("Failed to load sky texture: {}", e);
            return;
        }
    };

    let success_radius = 1.5; // Radio de éxito aumentado

    if let Some((p_row, p_col)) = find_start_position(&maze) {
        println!("Start position found at: row = {}, column = {}", p_row, p_col);

        let initial_x = p_col as f32 + 0.5;
        let initial_y = p_row as f32 + 0.5;
        let initial_angle = PI / 3.0;
        let initial_fov = PI / 3.0;

        let mut player = Player::new(initial_x, initial_y, initial_angle, initial_fov, 0.01);

        println!("Initial player position: x = {}, y = {}", player.pos.x, player.pos.y);

        let mut last_mouse_x = window.get_mouse_pos(minifb::MouseMode::Pass).unwrap_or((window_width as f32 / 2.0, window_height as f32 / 2.0)).0;

        let mut view_mode = ViewMode::View3D;
        let mut level_completed = false; // Flag to indicate if the player has reached the goal
        let mut show_initial_text = true; // Flag for initial text overlay
        let mut show_final_text = false; // Flag for final text overlay

        let mut previous_time = Instant::now();
        let mut frame_count = 0;

        while window.is_open() && !window.is_key_down(Key::Escape) {
            let current_time = Instant::now();
            let delta_time = current_time.duration_since(previous_time).as_secs_f32();
            previous_time = current_time;

            framebuffer.clear();

            if show_initial_text {
                // Dibujar el overlay semitransparente
                draw_overlay(&mut framebuffer, 50); // 128 es el valor de opacidad (0-255)

                // Render initial overlay text
                let title_x = (framebuffer.width - 300) / 2;
                let title_y = framebuffer.height / 2 - 100;

                let press_anywhere_x = (framebuffer.width - 200) / 2;
                let press_anywhere_y = framebuffer.height / 2;

                framebuffer.set_current_color(0xFFFF00); // White color for text
                framebuffer.draw_text(title_x, title_y, "INFERNALIA", &font, 48.0);
                framebuffer.draw_text(press_anywhere_x, press_anywhere_y, "Press Anywhere to Start", &font, 24.0);

                // Wait for input to start the game
                if window.get_mouse_down(minifb::MouseButton::Left) || window.is_key_down(Key::Enter) {
                    show_initial_text = false;
                }
            } else if show_final_text {
                // Dibujar el overlay semitransparente
                draw_overlay(&mut framebuffer, 50);

                // Render final overlay text
                let title_x = (framebuffer.width - 500) / 2;
                let title_y = framebuffer.height / 2 - 100;

                framebuffer.set_current_color(0xFFFF00); // Yellow color for text
                framebuffer.draw_text(title_x, title_y, "LABRYNTH FELLED!", &font, 48.0);

            } else {
                // Normal gameplay logic

                let mut moving = false;

                if window.is_key_down(Key::W) {
                    player.move_forward(1.0, &maze);
                    moving = true;
                }
                if window.is_key_down(Key::S) {
                    player.move_forward(-1.0, &maze);
                    moving = true;
                }
                if window.is_key_down(Key::A) {
                    player.strafe(1.0, &maze);
                    moving = true;
                }
                if window.is_key_down(Key::D) {
                    player.strafe(-1.0, &maze);
                    moving = true;
                }

                if moving {
                    sound_manager.play_footsteps("assets/music/steps.mp3");
                } else {
                    sound_manager.stop_footsteps();
                }

                // Check if the player is within the success radius of the goal
                let distance_to_goal = ((player.pos.x - goal_x as f32 + 0.5).powi(2) + (player.pos.y - goal_y as f32 + 0.5).powi(2)).sqrt();

                if distance_to_goal <= success_radius {
                    level_completed = true;
                    show_final_text = true; // Show the final overlay text
                }

                // Verificación de si el jugador está fuera de los límites
                if player.is_out_of_bounds(&maze) {
                    player.respawn(&maze);
                }
                if window.is_key_down(Key::Key1) {
                    view_mode = ViewMode::View2D;
                } else if window.is_key_down(Key::Key2) {
                    view_mode = ViewMode::View3D;
                }

                // Captura el movimiento del mouse solo si está dentro de los límites de la ventana
                if let Some(mouse_pos) = window.get_mouse_pos(minifb::MouseMode::Pass) {
                    let mouse_dx = mouse_pos.0 - last_mouse_x;

                    if mouse_pos.0 >= 0.0 && mouse_pos.0 <= window_width as f32 &&
                        mouse_pos.1 >= 0.0 && mouse_pos.1 <= window_height as f32 {
                        if mouse_dx.abs() > 0.0 {
                            let sensitivity_multiplier = 2.0;
                            player.rotate(mouse_dx * sensitivity_multiplier);
                        }
                    }

                    last_mouse_x = mouse_pos.0;
                }

                if player.is_out_of_bounds(&maze) {
                    println!("Advertencia: Jugador fuera de los límites (x: {}, y: {}). Respawneando...", player.pos.x, player.pos.y);
                    player.respawn(&maze);
                }

                // Render the maze and player view
                match view_mode {
                    ViewMode::View2D => {
                        render_2d(&mut framebuffer, &player, &maze);
                    }
                    ViewMode::View3D => {
                        render3d(&mut framebuffer, &mut player, &maze, &wall_texture, &floor_texture, &sky_texture);
                    }
                }

                // Calculate FPS
                frame_count += 1;
                if delta_time > 0.0 {
                    let fps = (1.0 / delta_time) as usize;
                    framebuffer.set_current_color(0xFFFFFFFF); // Set text color to white
                    framebuffer.draw_text(framebuffer.width - 100, 30, &format!("FPS: {}", fps), &font, 24.0);
                }
            }

            window
                .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
                .unwrap();

            std::thread::sleep(frame_delay);
        }
    } else {
        println!("Error: Start position 'p' not found in the maze.");
    }
}
