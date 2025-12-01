use macroquad::prelude::*;
mod game;
mod physics;

use crate::physics::body;
use game::world::World;

// Cambio la configuracion inicial del window
// Por alguna razon no es posible hacer que se centre

fn window_conf() -> Conf {
    Conf {
        window_title: "Simulacion arena".to_owned(),
        window_height: 600,
        window_width: 800,
        window_resizable: false,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut world = World::new();
    loop {
        let dt = get_frame_time();
        let mouse_pos: Vec2 = vec2(mouse_position().0, mouse_position().1);

        clear_background(BLACK);

        // Cambios
        if is_mouse_button_down(MouseButton::Left) {
            let body = body::Body::new(mouse_pos);
            world.add_body(body);
        }
        world.update(dt);

        // Este for debuja un rectangulo del tamaño del body en la posicion deseada
        for body in &world.bodies {
            draw_rectangle(
                body.position.x,
                body.position.y,
                body.size,
                body.size,
                WHITE,
            );
        }
        draw_text(&mouse_pos.to_string(), 10.0, 25.0, 30.0, WHITE);
        draw_text(
            &format!("Bodies: {}", world.bodies.len()),
            150.0,
            25.0,
            30.0,
            WHITE,
        );

        next_frame().await;
    }
}
