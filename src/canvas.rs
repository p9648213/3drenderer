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
    }

