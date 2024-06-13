use macroquad::prelude::*;
use macroquad_platformer::*;

mod resources;
mod ui;
mod player;


#[macroquad::main("RustyByte")]
async fn main() {
    //let error_texture = load_texture("notouch/error.png").await.unwrap();

    let resources = match resources::load_resources().await {
        Ok(res) => {
            println!("Successfully initialized resources");
            res
        }
        Err(e) => {
            eprintln!("Failed to initialize resources: {}", e);
            // Handle the error appropriately
            return;
        }
    };



    let mut world = World::new();
    world.add_static_tiled_layer(resources.colliders, 8., 8., 40, 1);

    let mut player = player::Player {
        collider: world.add_actor(vec2(50.0, 80.0), 8, 8),
        direction: vec2(0., 0.),
        speed: 100.,
        sprite_id: 120
    };

    let camera = Camera2D::from_display_rect(Rect::new(0.0, 152.0, 320.0, -152.0));

    loop {
        clear_background(BLACK);

    //process everything
        set_camera(&camera);

        
        resources.tiled_map.draw_tiles("main layer", Rect::new(0.0, 0.0, 320.0, 152.0), None);


        // draw player
        {

            let pos = world.actor_pos(player.collider);
            if player.direction.x >= 0.0 {
                resources.tiled_map.spr("tileset", player.sprite_id, Rect::new(pos.x, pos.y, 8.0, 8.0));
            } else {
                resources.tiled_map.spr(
                    "tileset",
                    player.sprite_id,
                    Rect::new(pos.x + 8.0, pos.y, -8.0, 8.0),
                );
            }
        }

        // player movement control
        {
            //let pos = world.actor_pos(player.collider);
            //let on_ground = world.collide_check(player.collider, pos + vec2(0., 1.));

            // if on_ground == false {
            //     player.speed.y += 100. * get_frame_time();
            // }

            player::movement(&mut player);

            world.move_h(player.collider, player.direction.x * get_frame_time());
            world.move_v(player.collider, player.direction.y * get_frame_time());
        }

        ui::gui(&mut player);

        next_frame().await
    }
}




// widgets::Window::new(hash!(), vec2(470., 50.), vec2(300., 300.))
// .label("DEV UI")
// .ui(&mut *root_ui(), |ui| {
//     ui.tree_node(hash!(), "input", |ui| {
//         ui.label(None, "Some random text");
//         if ui.button(None, "click me") {
//             println!("hi");
//         }

//         ui.separator();

//         ui.label(None, "Some other random text");
//         if ui.button(None, "other button") {
//             println!("hi2");
//         }
//         ui.separator();

//         ui.input_text(hash!(), "<- input text 1", &mut data0);
//         ui.input_text(hash!(), "<- input text 2", &mut data1);
//         ui.label(
//             None,
//             &format!("Text entered: \"{}\" and \"{}\"", data0, data1),
//         );
//         ui.separator();
//     });
//     ui.tree_node(hash!(), "sliders", |ui| {
//         ui.slider(hash!(), "[-10 .. 10]", -10f32..10f32, &mut number0);
//         ui.slider(hash!(), "[0 .. 100]", 0f32..100f32, &mut number1);
//     });
//     ui.tree_node(hash!(), "editbox 1", |ui| {
//         ui.label(None, "This is editbox!");
//         ui.editbox(hash!(), vec2(285., 165.), &mut text0);
//     });
//     ui.tree_node(hash!(), "editbox 2", |ui| {
//         ui.label(None, "This is editbox!");
//         ui.editbox(hash!(), vec2(285., 165.), &mut text1);
//     });
// });