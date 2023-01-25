use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    ecs::system::Command,
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use components::{SlimeAmount, SlimeGrid, SlimeSource, Tile, TilePosition};

mod components;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_startup_system(spawn_tiles(10, 10))
        .add_system_to_stage(CoreStage::PreUpdate, spawn_slime)
        .add_system_to_stage(CoreStage::Update, spread_slime)
        .add_system_to_stage(CoreStage::Last, render_slime_color)
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    let scale = 0.02f32;
    let position = 5f32;

    camera.transform = Transform::from_scale(Vec3::new(scale, scale, scale));
    camera.transform.translation = Vec3::new(position, position, 0f32);

    commands.spawn(camera);
}

fn spawn_tiles(width: usize, height: usize) -> impl Fn(Commands) {
    move |mut commands| {
        let mut slime_grid = SlimeGrid::new(width, height);

        for x in 0..width {
            for y in 0..height {
                let position = TilePosition {
                    x: x as i32,
                    y: y as i32,
                };
                let sprite = SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2 { x: 1f32, y: 1f32 }),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(Vec3::new((x) as f32, (y) as f32, 0f32)),
                    ..Default::default()
                };

                commands.spawn((position, sprite));

                let amount = SlimeAmount(((x + y * 20) as i64) * 1_000_000_000);
                slime_grid.set_amount(x, y, amount);
            }
        }

        commands.spawn(slime_grid);

        let spawner = SlimeSource {
            amount: SlimeAmount(10_000_000_000),
        };
        let position = TilePosition::new(2, 5);
        commands.spawn((spawner, position));

        let spawner = SlimeSource {
            amount: SlimeAmount(-10_000_000_000),
        };
        let position = TilePosition::new(6, 8);
        commands.spawn((spawner, position));
    }
}

fn render_slime_color(
    grid_query: Query<&SlimeGrid>,
    mut tile_query: Query<(&mut Sprite, &TilePosition)>,
) {
    let slime_grid = grid_query.single();
    for (mut sprite, position) in &mut tile_query {
        let amount = slime_grid.get_amount(position.x as usize, position.y as usize);
        let rgb = (amount.0 / 1_000_000_000) as u8;
        sprite.color = Color::rgb_u8(rgb, rgb, rgb);
    }
}

fn spread_slime(mut query: Query<&mut SlimeGrid>) {
    let mut slime_grid = query.single_mut();
    slime_grid.prepare_slime_spread();
    slime_grid.spread_slime();
}

fn spawn_slime(
    mut grid_query: Query<&mut SlimeGrid>,
    spawner_query: Query<(&SlimeSource, &TilePosition)>,
) {
    let mut slime_grid = grid_query.single_mut();
    for (spawner, position) in &spawner_query {
        slime_grid.add_amount(position.x as usize, position.y as usize, spawner.amount);
    }
}
