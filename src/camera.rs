pub struct Camera {

    pub pos_x: f32,
    pub pos_y: f32,
}

impl Camera {

    pub fn new() -> Camera {

        Camera { pos_x: 0.0, pos_y: 0.0 }
    }

    pub fn update(&mut self) {

        if self.pos_x < 0.0 {
            self.pos_x = 0.0;
        }
        if self.pos_y < 0.0 {
            self.pos_y = 0.0;
        }
        if self.pos_x > 1024.0 - 256.0 {
            self.pos_x = 1024.0 - 256.0;
        }
        if self.pos_y > 1024.0 - 192.0 {
            self.pos_y = 1024.0 - 192.0;
        }
        self.pos_y = self.pos_y.floor();
        self.pos_x = self.pos_x.floor();
    }
}