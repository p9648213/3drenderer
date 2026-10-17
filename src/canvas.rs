pub struct Canvas<'a> {
    buffer: &'a mut [u32],
    width: usize,
    height: usize,
}

impl<'a> Canvas<'a> {
    pub fn new(buffer: &'a mut [u32], width: usize, height: usize) -> Self {
        Self {
            buffer,
            width,
            height,
        }
    }

    pub fn clear(&mut self, color: u32) {
        self.buffer.fill(color);
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = color;
        }
    }

    pub fn draw_grid(&mut self, color: u32) {
        for y in (0..self.height).step_by(10) {
            for x in (0..self.width).step_by(10) {
                self.draw_pixel(x, y, color);
            }
        }
    }

    pub fn draw_rect(
        &mut self,
        x: usize,
        y: usize,
        rect_width: usize,
        rect_height: usize,
        color: u32,
    ) {
        let max_x = (x + rect_width).min(self.width);
        let max_y = (y + rect_height).min(self.height);

        for current_y in y..max_y {
            let row_offset = current_y * self.width;
            for current_x in x..max_x {
                self.buffer[row_offset + current_x] = color;
            }
        }
    }

    pub fn draw_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, color: u32) {
        let delta_x = x1 as f32 - x0 as f32;
        let delta_y = y1 as f32 - y0 as f32;

        let longest_side_length = if delta_x.abs() >= delta_y.abs() {
            delta_x.abs()
        } else {
            delta_y.abs()
        };

        let x_inc = delta_x / longest_side_length;
        let y_inc = delta_y / longest_side_length;

        let mut current_x = x0 as f32;
        let mut current_y = y0 as f32;

        for _ in 0..longest_side_length as u32 {
            self.draw_pixel(
                current_x.round() as usize,
                current_y.round() as usize,
                color,
            );
            current_x += x_inc;
            current_y += y_inc;
        }
    }

    pub fn draw_triangle(
        &mut self,
        x0: usize,
        y0: usize,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
        color: u32,
    ) {
        self.draw_line(x0, y0, x1, y1, color);
        self.draw_line(x1, y1, x2, y2, color);
        self.draw_line(x2, y2, x0, y0, color);
    }
}
