use dioxus_devtools::subsecond;
use egor::{Color, KeyCode, app::App};

fn main() {
    let mut player_x = 0.0;
    let mut player_y = 0.0;
    // initialize subsecond for hot reloading
    dioxus_devtools::connect_subsecond();

    App::init(|ctx| {
        ctx.set_title("Minimal Shooter");
    })
    .run(move |t, g, i| {
        // Use subsecond to run the game loop with hot reloading
        subsecond::call(|| {
            g.clear(Color::WHITE);

            let dx = i.keys_held(&[KeyCode::KeyD, KeyCode::ArrowRight]) as i8
                - i.keys_held(&[KeyCode::KeyA, KeyCode::ArrowLeft]) as i8;
            let dy = i.keys_held(&[KeyCode::KeyS, KeyCode::ArrowDown]) as i8
                - i.keys_held(&[KeyCode::KeyW, KeyCode::ArrowUp]) as i8;
            //let moving = dx != 0 || dy != 0;

            player_x += dx as f32 * 200.0 * t.delta;
            player_y += dy as f32 * 200.0 * t.delta;
            g.camera().target(player_x, player_y);

            g.rect()
                .at(player_x, player_y)
                .size(64., 64.)
                .color(Color::BLUE);

            g.rect().at(50.0, 105.0).size(64., 64.).color(Color::RED);
        });
    });
}
