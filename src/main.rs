use macroquad::prelude::*;

const GRAVITY: f32 = 1000.0;
const FLY_FORCE: f32 = 2000.0;

struct Player {
    x: f32,
    y: f32,
    vy: f32,
}

struct Rocket {
    x: f32,
    y: f32,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Jetpack-Quad".into(),
        window_width: 1120,
        window_height: 630,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    //game init
    let mut player = Player {
        x: 200.0,
        y: screen_height() / 2.,
        vy: 0.0,
    };

    loop {
        //update game
        player.y += player.vy * get_frame_time();
        player.vy += GRAVITY * get_frame_time();

        if player.y > screen_height() - 50.0 {
            player.y = screen_height() - 50.0;
            player.vy = 0.;
        }

        if player.y < 50.0 {
            player.y = 50.;
            player.vy = 0.;
        }

        if is_key_down(KeyCode::Space) || is_mouse_button_down(MouseButton::Left) {
            if player.vy > 0. {
                player.vy += -FLY_FORCE * 3. * get_frame_time();
            } else {
                player.vy += -FLY_FORCE * get_frame_time();
            }
        }

        //TODO: test 2d circle collision detection

        //draw world
        clear_background(BLACK);

        draw_rectangle(0., screen_height() - 75., screen_width(), 75.0, ORANGE);
        draw_rectangle(0., 0., screen_width(), 75.0, ORANGE);
        draw_circle(player.x, player.y, 20.0, BLUE);

        next_frame().await
    }
}
