pub struct AnimatedSprite {
    frames: Vec<Texture>,
    current_frame: usize,
    frame_time: f32,
    time_since_last_frame: f32,
}

impl AnimatedSprite {
    pub fn new(frames: Vec<Texture>, frame_time: f32) -> Self {
        AnimatedSprite {
            frames,
            current_frame: 0,
            frame_time,
            time_since_last_frame: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.time_since_last_frame += delta_time;
        if self.time_since_last_frame >= self.frame_time {
            self.current_frame = (self.current_frame + 1) % self.frames.len();
            self.time_since_last_frame = 0.0;
        }
    }

    pub fn render(&self, framebuffer: &mut Framebuffer, x: usize, y: usize) {
        let texture = &self.frames[self.current_frame];
        // Render the current frame of the sprite
        for j in 0..texture.height {
            for i in 0..texture.width {
                let color = texture.get_pixel(i, j);
                framebuffer.set_current_color(color);
                framebuffer.point(x + i, y + j);
            }
        }
    }
}
