pub struct Timer {
    pub duration: f32,
}

impl Timer {
    pub fn new(duration: f32) -> Self {
        Timer { duration }
    }

    pub fn tick_down(&mut self, delta: f32) {
        self.duration -= delta
    }

    pub fn is_complete(&self) -> bool {
        self.duration <= 0.0
    }

    pub fn reset(&mut self, new_duration: f32) {
        self.duration = new_duration
    }
}
