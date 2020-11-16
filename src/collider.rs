use macroquad::prelude::*;
use crate::player::Player;
trait Collider {
    fn check_collision(&self, player: &mut Player);
}

pub struct RectCollider {
    position: Vec2,
    width: f32,
    height: f32,
}

impl Collider for RectCollider {
    fn check_collision(&self, player: &mut Player) {
        if player.position.x() >= self.position.x()
            && player.position.x() < self.position.x() + self.width
            && player.position.y() >= self.position.y()
            && player.position.y() < self.position.y() + self.height
        {
            player.velocity = Vec2::new(0.0, 0.0);
        }
    }
}
