#[allow(dead_code)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[allow(dead_code)]
impl Vec2 {
    pub fn length(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }

    pub fn normalize(&self) -> Vec2 {
        let len = self.length();
        Vec2 {
            x: self.x / len,
            y: self.y / len,
        }
    }

    pub fn subtract(&self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn add(&self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn scale(&self, factor: f32) -> Vec2 {
        Vec2 {
            x: self.x * factor,
            y: self.y * factor,
        }
    }

    pub fn point_in_direction(&self, angle: f64, distance: f64) -> Vec2 {
        let x_point = angle.cos() * distance;
        let y_point = angle.sin() * distance;
        self.add(&Vec2 {
            x: x_point as f32,
            y: y_point as f32,
        })
    }
}
