use macroquad::prelude::*;
use crate::player::Player;

use macroquad::ui::{
    hash, root_ui,
    widgets::{self},
};


pub fn gui(player: &mut Player) {
    let mut data0 = String::new();
    
    
    widgets::Window::new(hash!(), vec2(470., 50.), vec2(300., 300.))
    .label("DEV UI")
    .ui(&mut *root_ui(), |ui| {
        ui.tree_node(hash!(), "Player", |ui| {
            ui.slider(hash!(), "[0 .. 1000]", 0f32..1000f32, &mut player.speed);
            ui.input_text(hash!(), "<- sprite ID", &mut data0);
            if ui.button(None, "Apply") {
                match data0.parse() {
                    Ok(data) => {
                        player.sprite_id = data;
                    }
                    Err(e) => {
                        eprintln!("Failed to convert to u32, {:?}", e);
                    }
                }
            }
        });
    });
    }