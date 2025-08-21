use bevy::prelude::*;

#[derive(Component)]
struct HelloText;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Rust Test Word Game".into(),
                    resolution: (800.0, 600.0).into(),
                    canvas: Some("#bevy".to_owned()),
                    ..default()
                }),
                ..default()
            }),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, handle_input)
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn a camera
    commands.spawn(Camera2dBundle::default());

    // Spawn the "Hello World" text
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Hello World",
                TextStyle {
                    font_size: 48.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
        HelloText,
    ));
}

fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<HelloText>>,
    windows: Query<&Window>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        let window = windows.single();
        let window_width = window.width();
        let window_height = window.height();
        
        let move_speed = 5.0;
        let text_size = 48.0; // Approximate text width/height for boundary calculation
        let boundary_x = window_width / 2.0 - text_size;
        let boundary_y = window_height / 2.0 - text_size / 2.0;

        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.x -= move_speed;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.x += move_speed;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            direction.y += move_speed;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            direction.y -= move_speed;
        }

        // Apply movement with boundary checking
        let new_x = (transform.translation.x + direction.x).clamp(-boundary_x, boundary_x);
        let new_y = (transform.translation.y + direction.y).clamp(-boundary_y, boundary_y);
        
        transform.translation.x = new_x;
        transform.translation.y = new_y;
    }
}
