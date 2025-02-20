use bevy::prelude::*;

const PADDLE_X: f32 = -600.0;
const PADDLE_Y: f32 = 0.0;

const PADDLE_SPEED: f32 = 400.0;

const BALL_X: f32 = 0.0;
const BALL_Y: f32 = 0.0;

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_paddle)
        .run();
}

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
    // Ball
    commands.spawn((
        Ball,
        Sprite {
            color: Color::srgb(1.0, 1.0, 1.0),
            custom_size: Some(Vec2::new(15.0, 15.0)),
            ..default()
        },
        Transform::from_xyz(BALL_X, BALL_Y, 0.0),
    ));
}

fn move_paddle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle_transform: Single<&mut Transform, With<Paddle>>,
    time: Res<Time>,
) {
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction -= 1.0;
    }
    let new_paddle_y =
        paddle_transform.translation.y + direction * PADDLE_SPEED * time.delta_secs();

    paddle_transform.translation.y = new_paddle_y.clamp(-300.0, 300.0);
}
