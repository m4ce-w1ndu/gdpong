use godot::prelude::*;
use godot::engine::{Sprite2D, ISprite2D};

/// Side of the paddle.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PaddleSide {
    None,
    Left,
    Right
}

/// Default implementation for PaddleSide.
impl Default for PaddleSide {
    fn default() -> Self {
        PaddleSide::None
    }
}

/// Paddle sprite.
#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct Paddle {
    pub speed: f64,
    pub side: PaddleSide,
    base: Base<Sprite2D>
}

#[godot_api]
impl ISprite2D for Paddle {
    fn init(base: Base<Sprite2D>) -> Self {
        Self {
            speed: 0.0,
            side: PaddleSide::default(),
            base
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let y = self.base().get_position().y;
        let y = y + (self.speed * delta) as f32;
        self.base_mut().translate(Vector2::new(0.0, y));
    }
}