use bevy::{ecs::system::Command, prelude::*, sprite::MaterialMesh2dBundle};
use components::{Building, SlimeAmount, TilePosition};

mod components;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(spawn_tiles(10, 10))
        .add_system(increase_slime)
        .add_system(render_slime_color)
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    let scale = 0.02f32;
    camera.transform = Transform::from_scale(Vec3::new(scale, scale, scale));
    camera.transform.translation = Vec3::new(5f32, 5f32, 0f32);
    commands.spawn(camera);
}

fn spawn_tiles(width: usize, height: usize) -> impl Fn(Commands) {
    move |mut commands| {
        for x in 0..width {
            for y in 0..height {
                let position = TilePosition {
                    x: x as i32,
                    y: y as i32,
                };
                let amount = SlimeAmount((x + y * 20) as u64);
                let sprite = SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2 { x: 1f32, y: 1f32 }),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(Vec3::new((x) as f32, (y) as f32, 0f32)),
                    ..Default::default()
                };

                commands.spawn((position, amount, sprite));
            }
        }
    }
}

fn increase_slime(mut query: Query<&mut SlimeAmount>) {
    for mut amount in &mut query {
        amount.0 += 1;
    }
}

fn render_slime_color(mut query: Query<(&mut Sprite, &SlimeAmount)>) {
    for (mut color, amount) in &mut query {
        let rgb = amount.0 as u8;
        color.color = Color::rgb_u8(rgb, rgb, rgb);
    }
}
