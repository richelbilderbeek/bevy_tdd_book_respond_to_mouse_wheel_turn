use bevy::input::InputPlugin;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn create_app() -> App {
    let mut app = App::new();

    // Only add this plugin in testing.
    // The main app will assume it to be absent
    if cfg!(test) {
        app.add_plugins(InputPlugin);
    }

    app.add_systems(Startup, add_player);
    app.add_systems(Update, respond_to_mouse_wheel_turn);

    // Do not do update, as this will disallow to do more steps
    // app.update(); //Don't!
    app
}

fn add_player(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                scale: Vec3::new(64.0, 32.0, 1.0),
                ..default()
            },
            ..default()
        },
        Player {},
    ));
}

/*
fn respond_to_mouse_wheel_turn(
    mut query: Query<&mut Transform, With<Player>>,
    input: Res<ButtonInput<MouseButton>>,
) {
    /*
    if input.just_pressed(MouseButton::Other(())) {
        let mut player_position = query.single_mut();
        // Do something
        player_position.translation.y += 16.0;
    }
    */
}
*/

fn respond_to_mouse_wheel_turn(
    mut query: Query<&mut Transform, With<Player>>,
    mut mouse_wheel_event: EventReader<bevy::input::mouse::MouseWheel>,
) {
    for _event in mouse_wheel_event.read() {
        let mut player_position = query.single_mut();
        // Do something
        player_position.translation.x += 16.0;
    }
}

#[cfg(test)]
pub fn count_n_players(app: &App) -> usize {
    let mut n = 0;
    for c in app.world().components().iter() {
        if c.name().contains("::Player") {
            n += 1;
        }
    }
    n
}

#[cfg(test)]
fn get_player_position(app: &mut App) -> Vec3 {
    // Do 'app.update()' before calling this function,
    // else this assert goes off.
    assert_eq!(count_n_players(app), 1);
    let mut query = app.world_mut().query::<(&Transform, &Player)>();
    let (transform, _) = query.single(app.world());
    transform.translation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_testing() {
        assert_eq!(1 + 1, 2)
    }

    #[test]
    fn test_can_create_app() {
        create_app();
    }

    #[test]
    fn test_empty_app_has_no_players() {
        let mut app = App::new();
        assert_eq!(count_n_players(&mut app), 0);
    }

    #[test]
    fn test_create_app_has_a_player() {
        let mut app = create_app();
        app.update();
        assert_eq!(count_n_players(&mut app), 1);
    }

    #[test]
    fn test_player_is_at_origin() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_player_position(&mut app), Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_player_responds_to_mouse_wheel_turn() {
        let mut app = create_app();
        assert!(app.is_plugin_added::<InputPlugin>());
        app.update();

        // Not moved yet
        assert_eq!(Vec3::new(0.0, 0.0, 0.0), get_player_position(&mut app));

        // Scroll the mouse
        app.world_mut().send_event(bevy::input::mouse::MouseWheel {
            unit: MouseScrollUnit::Line,
            x: 10.0,
            y: 10.0,
            window: Entity::PLACEHOLDER,
        });
        app.update();

        // Moved now
        assert_ne!(Vec3::new(0.0, 0.0, 0.0), get_player_position(&mut app));
    }
}
