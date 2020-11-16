use macroquad::prelude::*;


// split game logic from rendering
// make this deterministic
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
    let player_max_x_speed:f32 = 15.;
    let player_deceleration = 2.;

    let player_gravity:f32 = 9.8;
    let player_max_y_speed:f32 = 15.;

    let player_jump_force:f32 = 100.;

    let mut player_position:Vec2 = Vec2::new(SCR_W / 2., SCR_H / 2.);
    
    let player_width = 1.;
    let player_height = 1.;

    let player_speed = 10.0;
    let mut player_velocity:Vec2 = Vec2::new(0.0, 0.0);

    loop {
        clear_background(SKYBLUE);

        let delta = get_frame_time();

        // split input to its own code

        if is_key_down(KeyCode::Right) && player_position.x() < SCR_W - player_width / 2. {
            player_velocity.set_x(player_velocity.x() + player_speed);
        }
        if is_key_down(KeyCode::Left) && player_position.x() > player_width / 2. {
            player_velocity.set_x(player_velocity.x() - player_speed);
        }

        /*
        if is_key_down(KeyCode::Down) && player_position.y() < SCR_H - platform_height / 2. {
            player_velocity.set_y(player_velocity.x() + player_speed);
        }
        */
        
        if is_key_pressed(KeyCode::Up) && player_position.y() > player_height / 2. {
            player_velocity.set_y(player_velocity.x() - player_jump_force);
        }
        

        velocity_clamping(&mut player_velocity, player_deceleration, player_gravity,
            player_max_x_speed, player_max_y_speed);

        player_position.set_x(player_position.x() + (player_velocity.x() * delta));
        player_position.set_y(player_position.y() + (player_velocity.y() * delta));

        // split rendering into its own code
        draw_rectangle(
            player_position.x() - player_width / 2.,
            player_position.y() - player_height / 2.,
            player_width,
            player_height,
            DARKPURPLE,
        );

        next_frame().await
    }
}

fn velocity_clamping(velocity: &mut Vec2, deceleration: f32, gravity: f32, max_x_speed: f32, max_y_speed: f32){
    if velocity.x() >= 0.0 {
        velocity.set_x(velocity.x() - deceleration);
        if velocity.x() <= 0.0 {
            velocity.set_x(0.0);
        }
        else if velocity.x() > max_x_speed {
            velocity.set_x(max_x_speed);
        }
    }
    else
    {
        velocity.set_x(velocity.x() + deceleration);
        if velocity.x() >= 0.0 {
            velocity.set_x(0.0);
        }
        else if velocity.x() < -max_x_speed {
            velocity.set_x(-max_x_speed);
        }
    }

    velocity.set_y(velocity.y() + gravity);
    if velocity.y() > max_y_speed {
        velocity.set_y(max_y_speed);
    }
}