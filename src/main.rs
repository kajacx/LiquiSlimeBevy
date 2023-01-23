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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_tiles(width: usize, height: usize) -> impl Fn(Commands) {
    move |mut commands| {
        for x in 0..width {
            for y in 0..height {
                let position = TilePosition {
                    x: x as i32,
                    y: y as i32,
                };
                let amount = SlimeAmount((x + y * 10) as u64);
                let sprite = SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2 { x: 10f32, y: 10f32 }),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(Vec3::new(
                        (x * 10) as f32,
                        (y * 10) as f32,
                        0f32,
                    )),
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
