pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl Vec2 {
    pub fn length (&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }

    pub fn normalize(&self) -> Vec2 {
        let len = self.length();
        Vec2 { x: self.x / len, y: self.y / len }
    }

    pub fn subtract(&self, other: &Vec2) -> Vec2 {
        Vec2 { x: self.x - other.x, y: self.y - other.y }
    }

    pub fn add(&self, other: &Vec2) -> Vec2 {
        Vec2 { x: self.x + other.x, y: self.y + other.y }
    }

    pub fn scale(&self, factor: f32) -> Vec2 {
        Vec2 { x: self.x * factor, y: self.y * factor }
    }
}