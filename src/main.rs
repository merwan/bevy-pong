use bevy::prelude::*;

const PADDLE_X: f32 = -600.0;
const PADDLE_Y: f32 = 0.0;

#[derive(Component)]
struct Paddle;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    // Paddle
    commands.spawn((
        Paddle,
        Sprite {
            color: Color::srgb(1.0, 1.0, 1.0),
            custom_size: Some(Vec2::new(20.0, 100.0)),
            ..default()
        },
        Transform::from_xyz(PADDLE_X, PADDLE_Y, 0.0),
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}
