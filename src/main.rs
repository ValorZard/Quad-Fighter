use macroquad::prelude::*;
mod player;
use player::{Player};
mod player_resources;
use player_resources::{PlayerConfig};
mod collider;
use collider::RectCollider;
// split game logic from rendering
// make this deterministic
// write the code so that you can rip out a lot of this and put it in another engine, and it would still work
#[macroquad::main("Quad_Figher")]
async fn main() {
    //Screen stuff
    const SCR_W: f32 = 20.0;
    const SCR_H: f32 = 20.0;

    // build camera with following coordinate system:
    // (0., 0)     .... (SCR_W, 0.)
    // (0., SCR_H) .... (SCR_W, SCR_H)
    set_camera(Camera2D {
        zoom: vec2(1. / SCR_W * 2., -1. / SCR_H * 2.),
        target: vec2(SCR_W / 2., SCR_H / 2.),
        ..Default::default()
    });

    //player stuff

    let mut player = Player{
        position : Vec2::new(SCR_W / 2., SCR_H / 2.),
        velocity : Vec2::new(0.0, 0.0),
        width : 1.,
        height : 1.,
    };

    let player_config: PlayerConfig = Default::default();


    loop {
        clear_background(SKYBLUE);

        let delta = get_frame_time();

        // split input to its own code

        if is_key_down(KeyCode::Right) && player.position.x() < SCR_W - player.width / 2. {
            player.velocity.set_x(player.velocity.x() + player_config.speed);
        }
        if is_key_down(KeyCode::Left) && player.position.x() > player.width / 2. {
            player.velocity.set_x(player.velocity.x() - player_config.speed);
        }

        /*
        if is_key_down(KeyCode::Down) && player_position.y() < SCR_H - platform_height / 2. {
            player_velocity.set_y(player_velocity.x() + player_speed);
        }
        */
        
        if is_key_pressed(KeyCode::Up) && player.position.y() > player.height / 2. {
            player.velocity.set_y(player.velocity.x() - player_config.jump_force);
        }
        

        //put this in a physics rendering function
        player.velocity_clamping(player_config.deceleration, player_config.gravity,
            player_config.max_x_speed, player_config.max_y_speed);

        player.position.set_x(player.position.x() + (player.velocity.x() * delta));
        player.position.set_y(player.position.y() + (player.velocity.y() * delta));

        // split rendering into its own code
        draw_rectangle(
            player.position.x() - player.width / 2.,
            player.position.y() - player.height / 2.,
            player.width,
            player.height,
            DARKPURPLE,
        );

        //make this a collider part of the collider class
        draw_rectangle(
            0.,
            19.,
            100.,
            1.,
            DARKPURPLE,
        );

        next_frame().await
    }
}

