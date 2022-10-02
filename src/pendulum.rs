const MIDDLE: (f32, f32) = (0.0, 300.0);
const LENGTH: f32 = 400.0;
const SCALE: f32 = 20.0;
const GRAVITY: f32 = -9.81;

pub struct Pendulum {
    pub root: (f32, f32),
    pub position: (f32, f32),
    pub length: f32,
    pub mass: f32,
    angular_velocity: f32,
    angle: f32,
}

impl Pendulum {
    pub fn update(&mut self, delta: f32) {
        let angular_acceleration = GRAVITY / self.length * self.angle.cos();
        self.angular_velocity += angular_acceleration * delta * SCALE;
        self.angle += self.angular_velocity * delta;
        self.position = (
            self.length * self.angle.cos() + MIDDLE.0,
            self.length * self.angle.sin() + MIDDLE.1,
        );
    }
}

pub mod state {
    use super::{Pendulum, LENGTH, MIDDLE};

    pub fn new() -> Pendulum {
        Pendulum {
            root: MIDDLE,
            position: (MIDDLE.0 - LENGTH, MIDDLE.1),
            length: LENGTH,
            mass: 1.0,
            angular_velocity: 0.0,
            angle: 3.14159,
        }
    }
}
