use raylib::prelude::*;

const FORW_KEY: KeyboardKey = KeyboardKey::KEY_W;
const BACK_KEY: KeyboardKey = KeyboardKey::KEY_S;
const LEFT_KEY: KeyboardKey = KeyboardKey::KEY_A;
const RIGH_KEY: KeyboardKey = KeyboardKey::KEY_D;
const UPPP_KEY: KeyboardKey = KeyboardKey::KEY_SPACE;
const DOWN_KEY: KeyboardKey = KeyboardKey::KEY_LEFT_SHIFT;

const SPEED: f32 = 0.01;
const FRICTION: f32 = 0.1;

// "Player". Controls the current position and their momentum.
pub struct Player {
    pub camera: Camera3D,
    pub momentum: Vector3,
}

impl Player {
    pub fn new() -> Player {
        return Player {
            camera: Camera3D::perspective(
        Vector3::new(3.0, 3.0, 3.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
            ),       
            momentum: Vector3{x: 0.0, y: 0.0, z: 0.0},
        };
    }
}

pub fn update_camera(player: &mut Player, rl: &mut RaylibHandle) {
    let pos_update = player.momentum * SPEED;
    player.camera.position += pos_update;
    player.camera.target += pos_update;
    // Now we update the momentum
    if rl.is_key_down(FORW_KEY) {
        player.momentum.z = -1.0;
    } else if rl.is_key_down(BACK_KEY) {
        player.momentum.z = 1.0;
    } else {
        player.momentum.z *= 1.0 - FRICTION;
    }
    if rl.is_key_down(LEFT_KEY) {
        player.momentum.x = -1.0;
    } else if rl.is_key_down(RIGH_KEY) {
        player.momentum.x = 1.0;
    } else {
        player.momentum.x *= 1.0 - FRICTION;
    }
    if rl.is_key_down(UPPP_KEY) {
        player.momentum.y = 1.0;
    } else if rl.is_key_down(DOWN_KEY) {
        player.momentum.y = -1.0;
    } else {
        player.momentum.y *= 1.0 - FRICTION;
    }
}