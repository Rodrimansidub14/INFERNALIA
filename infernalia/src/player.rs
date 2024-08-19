use nalgebra_glm::Vec2;
use std::f32::consts::PI;
use std::time::Instant;

pub struct Player {
    pub pos: Vec2,  // Posición del jugador en el mundo
    pub a: f32,     // Ángulo al que está mirando el jugador (en radianes)
    pub fov: f32,   // Campo de visión
    pub speed: f32, // Velocidad de movimiento
    pub rot_speed: f32, // Velocidad de rotación
    pub collision_radius: f32, // Radio de colisión
    pub sensitivity: f32, // Sensibilidad del ratón
    pub last_update: Instant, // Última vez que se actualizó el jugador
}

impl Player {
    pub fn new(x: f32, y: f32, angle: f32, fov: f32, sensitivity: f32) -> Self {
        Player {
            pos: Vec2::new(x, y),
            a: angle,
            fov,
            speed: 2.0, // Velocidad de movimiento estándar ajustada para usar delta time
            rot_speed: 0.02, // Velocidad de rotación ajustada a un valor menor
            collision_radius: 0.25, // Radio de colisión
            sensitivity, // Sensibilidad del ratón
            last_update: Instant::now(), // Inicializar el tiempo
        }
    }

    // Método para calcular el tiempo transcurrido desde la última actualización (delta time)
    fn delta_time(&self) -> f32 {
        let now = Instant::now();
        let delta = now.duration_since(self.last_update).as_secs_f32();
        delta
    }

    // Método para mover al jugador hacia adelante o atrás con detección de colisiones
    pub fn move_forward(&mut self, direction: f32, maze: &Vec<Vec<char>>) {
        let delta = self.delta_time();
        let distance = self.speed * direction * delta;
        
        let new_x = self.pos.x + distance * self.a.cos();
        let new_y = self.pos.y + distance * self.a.sin();

        // Verificar si la nueva posición está dentro de una pared
        if !self.is_collision(new_x, new_y, maze) {
            self.pos.x = new_x;
            self.pos.y = new_y;
        }

        self.last_update = Instant::now(); // Actualizar el tiempo
    }

    // Método para hacer strafe del jugador a la izquierda o derecha con detección de colisiones
    pub fn strafe(&mut self, direction: f32, maze: &Vec<Vec<char>>) {
        let delta = self.delta_time();
        let distance = self.speed * direction * delta;

        let new_x = self.pos.x + distance * self.a.sin();
        let new_y = self.pos.y - distance * self.a.cos();

        // Verificar si la nueva posición está dentro de una pared
        if !self.is_collision(new_x, new_y, maze) {
            self.pos.x = new_x;
            self.pos.y = new_y;
        }

        self.last_update = Instant::now(); // Actualizar el tiempo
    }

    // Método para verificar colisiones con el laberinto
    fn is_collision(&self, x: f32, y: f32, maze: &Vec<Vec<char>>) -> bool {
        let left = (x - self.collision_radius).floor() as isize;
        let right = (x + self.collision_radius).floor() as isize;
        let top = (y - self.collision_radius).floor() as isize;
        let bottom = (y + self.collision_radius).floor() as isize;
    
        if left < 0 || right >= maze[0].len() as isize || top < 0 || bottom >= maze.len() as isize {
            println!("Advertencia: Jugador fuera de los límites (x: {}, y: {}).", x, y);
            return true; // Considera cualquier salida fuera de los límites como una colisión
        }
    
        if maze[top as usize][left as usize] != ' ' || maze[top as usize][right as usize] != ' ' ||
            maze[bottom as usize][left as usize] != ' ' || maze[bottom as usize][right as usize] != ' ' {
            return true;
        }
    
        false
    }
    

    // Método para rotar al jugador
    pub fn rotate(&mut self, mouse_dx: f32) {
        let sensitivity_multiplier = 1.0; // Reducir el multiplicador para una rotación más lenta
        self.a += mouse_dx * self.rot_speed * sensitivity_multiplier;

        // Normalizar el ángulo dentro del rango de 0 a 2*PI
        if self.a < 0.0 {
            self.a += 2.0 * PI;
        } else if self.a >= 2.0 * PI {
            self.a -= 2.0 * PI;
        }

    }


    pub fn is_out_of_bounds(&self, maze: &Vec<Vec<char>>) -> bool {
        self.pos.x < 0.0 || self.pos.y < 0.0 || self.pos.x as usize >= maze[0].len() || self.pos.y as usize >= maze.len()
    }

    pub fn respawn(&mut self, maze: &Vec<Vec<char>>) {
        for (y, row) in maze.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == ' ' {  // Find the first empty space
                    self.pos.x = x as f32 + 0.5;
                    self.pos.y = y as f32 + 0.5;
                    println!("Jugador respawneado en (x: {}, y: {})", self.pos.x, self.pos.y);
                    return;
                }
            }
        }
        println!("Advertencia: No se encontró un espacio vacío para respawnear al jugador.");
    }

    pub fn correct_position_if_out_of_bounds(&mut self, maze: &Vec<Vec<char>>) {
        if self.is_out_of_bounds(maze) {
            println!("Advertencia: Jugador fuera de los límites (x: {}, y: {}). Respawneando...", self.pos.x, self.pos.y);
            self.respawn(maze);
        }
    }
    pub fn move_to_safe_position(&mut self, maze: &Vec<Vec<char>>) {
        // Attempt to move the player to a safe position
        for (row_index, row) in maze.iter().enumerate() {
            for (col_index, &cell) in row.iter().enumerate() {
                if cell == ' ' {
                    // Move player to the first available safe position
                    self.pos.x = col_index as f32 + 0.5;
                    self.pos.y = row_index as f32 + 0.5;
                    println!("Jugador movido a una posición segura: (x: {}, y: {})", self.pos.x, self.pos.y);
                    return;
                }
            }
        }

        // Fallback if no safe position found (this is very unlikely)
        println!("Error crítico: No se encontró ninguna posición segura. El jugador sigue fuera de los límites.");
    }
    pub fn check_for_win(&self, maze: &Vec<Vec<char>>) -> bool {
        let player_x = self.pos.x as usize;
        let player_y = self.pos.y as usize;
        
        if maze[player_y][player_x] == 'g' {
            return true;
        }
        false
    }
}
