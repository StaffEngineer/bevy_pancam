use bevy::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};
use rand::prelude::random;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PanCamPlugin::default())
        .add_startup_system(setup)
        .add_system(toggle_key)
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(PanCam::default());

    let n = 20;
    let spacing = 50.;
    let offset = spacing * n as f32 / 2.;
    let custom_size = Some(Vec2::new(spacing, spacing));
    for x in 0..n {
        for y in 0..n {
            let x = x as f32 * spacing - offset;
            let y = y as f32 * spacing - offset;
            let color = Color::hsl(240., random::<f32>() * 0.3, random::<f32>() * 0.3);
            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size,
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.),
                ..default()
            });
        }
    }
}

fn toggle_key(mut query: Query<&mut PanCam>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        for mut pancam in query.iter_mut() {
            pancam.enabled = !pancam.enabled;
        }
    }
}