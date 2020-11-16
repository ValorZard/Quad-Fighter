use macroquad::prelude::*;
use crate::player::Player;
pub trait Collider {
    fn check_collision(&self, player: &mut Player) -> bool;
}

pub struct RectCollider {
    pub position: Vec2,
    pub width: f32,
    pub height: f32,
}

impl Collider for RectCollider {
    fn check_collision(&self, player: &mut Player) -> bool {
        if player.position.x() + self.width >= self.position.x()
            && player.position.x()  < self.position.x() + self.width
            && player.position.y() + self.height >= self.position.y()
            && player.position.y() < self.position.y() + self.height
        {
            return true;
        }
        false
    }
}
