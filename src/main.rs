use raylib::prelude::*;

pub mod mesh_tools;
pub mod input;

use mesh_tools::VecMesh;
use input::camera_controls::{Player, update_camera};

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

/* example of generating a triangle from raylib examples page */
fn gen_custom_mesh() -> Mesh {
    let mut vmesh = VecMesh::new();

    vmesh.vertices = vec![ 0.0, 0.0, 0.0, /**/ 1.0, 0.0, 2.0, /**/ 2.0, 0.0, 0.0];
    vmesh.normals = vec![ 0.0, 1.0, 0.0, /**/ 0.0, 1.0, 0.0, /**/ 0.0, 1.0, 0.0];
    vmesh.texcoords = vec![0.0, 0.0, /**/ 0.5, 1.0, /**/ 1.0, 0.0];

    let mut mesh = vmesh.to_mesh();

    unsafe { mesh.upload(false); }
    return mesh;
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Hello, world!")
        .vsync()
        .highdpi()
        .build();

    let mut player = Player::new();

    let mut first_click = false;

    let mesh = gen_custom_mesh();
    let material = rl.load_material_default(&thread);

    while !rl.window_should_close() {
        // require a click on the window before updating camera so the camera
        // doesn't fly away when the cursor enters the window at first
        if !first_click {
            if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                first_click = true;
                rl.disable_cursor();
            }
        } else {
            // rl.update_camera(&mut camera, CameraMode::CAMERA_FIRST_PERSON);
            update_camera(&mut player, &mut rl);
        }


        rl.draw(&thread, |mut d| {
            d.clear_background(Color::LIGHTBLUE);

            if !first_click {
                d.draw_text("WIP: Click to start updating camera", 20, 20, 16, Color::DARKGREEN);
            }

            d.draw_mode3D(player.camera, |mut d2, _camera| {
                d2.draw_mesh(&mesh, material.clone(), Matrix::identity());
            });
        });
    }
}
