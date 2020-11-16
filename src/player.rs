use macroquad::prelude::*;
pub struct Player{
    pub position : Vec2,
    pub width : f32,
    pub height : f32,
    pub velocity : Vec2,
}

impl Player {
    pub fn velocity_clamping(&mut self, deceleration: f32, gravity: f32, max_x_speed: f32, max_y_speed: f32){
        if self.velocity.x() >= 0.0 {
            self.velocity.set_x(self.velocity.x() - deceleration);
            if self.velocity.x() <= 0.0 {
                self.velocity.set_x(0.0);
            }
            else if self.velocity.x() > max_x_speed {
                self.velocity.set_x(max_x_speed);
            }
        }
        else
        {
            self.velocity.set_x(self.velocity.x() + deceleration);
            if self.velocity.x() >= 0.0 {
                self.velocity.set_x(0.0);
            }
            else if self.velocity.x() < -max_x_speed {
                self.velocity.set_x(-max_x_speed);
            }
        }
    
        self.velocity.set_y(self.velocity.y() + gravity);
        if self.velocity.y() > max_y_speed {
            self.velocity.set_y(max_y_speed);
        }
    }
}