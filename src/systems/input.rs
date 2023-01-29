use bevy::{input::mouse::MouseButtonInput, prelude::*, render::camera::RenderTarget};

use crate::components::{Building, MoveOnClick, SlimeGrid, Tile, TilePosition};

pub struct GameInputPlugin;

impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        //app.add_system(pls_entities);
        app.add_system_to_stage(CoreStage::First, move_sources);
    }
}

fn pls_entities(world: &World) {
    println!("----");
    println!("----");
    println!("----");
    println!("----");
    println!("Entities: {:?}", world.entities());
    println!("----");
    println!("----");
    println!("----");
    println!("----");
    //for entity in world.entities().iter() {
    //    println!("Entity: {:?}", entity);
    //}
}

fn move_sources(
    mut spawners: Query<(&mut TilePosition, &MoveOnClick)>,
    mut mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    //camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>, // TODO: handle multiple cameras?
    camera: Query<(&Camera, &GlobalTransform)>,
    camera1: Query<&Camera>,
    camera2: Query<&GlobalTransform>,
) {
    //println!("Camera: {:?}", camera1.get_single());
    //println!("Global transform: {:?}", camera2.get_single());

    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let camera = camera.get_single();
    if (camera.is_err()) {
        println!("Oh no, camera not found: {:?}", camera);
        return; // TODO: bad spaghetti
    };

    let (camera, camera_transform) = camera.expect("Camera should exist");

    // get the window that the camera is displaying to (or the primary window)
    let window = if let RenderTarget::Window(id) = camera.target {
        windows.get(id)
    } else {
        windows.get_primary()
    }
    .expect("Window should be found");

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = window.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        eprintln!("World coords: {}/{}", world_pos.x, world_pos.y);

        let tile_pos = TilePosition::from_floats_floor(world_pos.x, world_pos.y);

        // Game logic once we have the world coords
        if (tile_pos.x >= 0 && tile_pos.x < 10 && tile_pos.y >= 0 && tile_pos.y < 10) {
            // TODO: hardwired world side. Also: make a new method "in world" in tile position?
            for (mut spawner_position, move_on) in &mut spawners {
                if mouse_input.just_pressed(move_on.mouse_button) {
                    *spawner_position = tile_pos;
                }
            }
        }
    }
}
