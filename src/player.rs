use macroquad::prelude::*;
use macroquad_platformer::*;


pub struct Player {
    pub collider: Actor,
    pub speed: f32,
    pub direction: Vec2,
    pub sprite_id: u32,
}



pub fn movement(player: &mut Player) {

    if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
        player.direction.x = player.speed;
    } else if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
        player.direction.x = -player.speed;
    } else {
        player.direction.x = 0.;
    }

    if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
        player.direction.y = player.speed;
    } else if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
        player.direction.y = -player.speed;
    } else {
        player.direction.y = 0.;
    }
}