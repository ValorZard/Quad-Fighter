pub struct PlayerConfig {
    pub speed:f32,
    pub max_x_speed:f32,
    pub deceleration:f32,
    pub gravity:f32,
    pub max_y_speed:f32,
    pub jump_force:f32,
}

const PLAYER : PlayerConfig = PlayerConfig{
    speed : 10.0,
    max_x_speed : 15.,
    deceleration : 2.,
    gravity : 9.8,
    max_y_speed : 15.,
    jump_force : 100.,
};

impl Default for PlayerConfig{
    fn default() -> Self {
        PLAYER
    }
}