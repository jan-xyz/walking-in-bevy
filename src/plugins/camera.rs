use bevy::{camera::Viewport, prelude::*, window::WindowResized};

use crate::plugins::player::Player;

pub struct CameraPlugin;

#[derive(Component)]
pub struct FollowPlayer(Entity);

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, follow_player);
        app.add_systems(Update, update_viewport);

        app.add_observer(on_player_added);
        app.add_observer(on_camera_added);
    }
}

fn on_player_added(
    trigger: On<Add, Player>,
    mut commands: Commands,
    player: Query<Entity, With<FollowPlayer>>,
) {
    let i = player.count();
    let player = trigger.event_target();
    let camera1 = Camera {
        order: i as isize,
        ..default()
    };
    // Spawn a camera
    commands.spawn((
        Camera3d::default(),
        camera1,
        Transform::default(),
        Name::new(format!("Player Camera {}", i)),
        FollowPlayer(player),
    ));
}

fn on_camera_added(
    _trigger: On<Add, FollowPlayer>,
    windows: Query<&Window>,
    mut cameras: Query<&mut Camera, With<FollowPlayer>>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let num_cameras = cameras.iter().len() as u32;

    cameras
        .iter_mut()
        .zip(calculate_viewports(window, num_cameras))
        .for_each(|(mut camera, viewport)| {
            camera.viewport = Some(viewport);
        });
}

fn update_viewport(
    mut resize_events: MessageReader<WindowResized>,
    windows: Query<&Window>,
    mut cameras: Query<&mut Camera, With<FollowPlayer>>,
) {
    if let Some(event) = resize_events.read().last() {
        let Ok(window) = windows.get(event.window) else {
            return;
        };
        let num_cameras = cameras.iter().len() as u32;

        cameras
            .iter_mut()
            .zip(calculate_viewports(window, num_cameras))
            .for_each(|(mut camera, viewport)| {
                camera.viewport = Some(viewport);
            });
    }
}

fn follow_player(
    time: Res<Time>,
    mut camera: Query<(&mut Transform, &FollowPlayer), Without<Player>>,
    players: Query<&Transform, With<Player>>,
) {
    for (mut cam_transform, player_follow) in camera.iter_mut() {
        let Ok(player_transform) = players.get(player_follow.0) else {
            continue;
        };
        let distance = 30.0;
        let height = 10.0;

        let target_translation =
            player_transform.translation + *player_transform.back() * distance + Vec3::Y * height;

        let target_rotation = Transform::from_translation(cam_transform.translation)
            .looking_at(player_transform.translation, Vec3::Y)
            .rotation;

        // lerp and slerp smoothen the movement of the camera by interpolating and moving toward the
        // desired location instead of snapping.
        // higher = snappier; lower = smoother
        let smoothing = 15.0;
        let t = time.delta_secs() * smoothing;
        cam_transform.translation = cam_transform.translation.lerp(target_translation, t);
        cam_transform.rotation = cam_transform.rotation.slerp(target_rotation, t)
    }
}

fn calculate_viewports(window: &Window, num_cameras: u32) -> Vec<Viewport> {
    let width = window.physical_width();
    let height = window.physical_height() / num_cameras;

    (0..num_cameras)
        .map(|i| Viewport {
            physical_size: UVec2::new(width, height),
            physical_position: UVec2::new(0, height * i),
            ..default()
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use std::time::Duration;

    use super::*;

    fn viewport_eq(a: &Viewport, b: &Viewport) -> bool {
        a.physical_size == b.physical_size && a.physical_position == b.physical_position
    }

    #[test]
    fn test_follow_player() {
        struct TestCase {
            name: &'static str,
            input_player_pos: Vec<Transform>,
            want_cam_pos: Vec<Vec3>,
        }

        let tests = [
            TestCase {
                name: "single player",
                input_player_pos: vec![Transform::from_xyz(100.0, 0.0, 0.0)],
                want_cam_pos: vec![Vec3::new(100.0, 10.0, 30.0)],
            },
            TestCase {
                name: "two player",
                input_player_pos: vec![
                    Transform::from_xyz(100.0, 0.0, 0.0),
                    Transform::from_xyz(-100.0, 0.0, 0.0),
                ],
                want_cam_pos: vec![Vec3::new(100.0, 10.0, 30.0), Vec3::new(-100.0, 10.0, 30.0)],
            },
        ];

        for TestCase {
            name,
            input_player_pos,
            mut want_cam_pos,
        } in tests.into_iter()
        {
            // Given
            let mut world = World::new();

            let time = Time::<()>::default();
            world.insert_resource(time);

            // insert all camera and player bundles to follow
            for transform in input_player_pos.into_iter() {
                let player = world.spawn((transform, Player)).id();

                let start = Transform::from_xyz(0.0, 0.0, 0.0);
                world.spawn((start, FollowPlayer(player)));
            }

            // When
            let mut schedule = Schedule::default();
            schedule.add_systems(follow_player);

            // Simulate time at 60fps
            for _ in 0..600 {
                world
                    .resource_mut::<Time<()>>()
                    .advance_by(Duration::from_millis(16));
                schedule.run(&mut world);
            }

            // Then
            let mut query = world.query::<(&Transform, &FollowPlayer)>();
            for (i, (got, _)) in query.iter(&world).enumerate() {
                let want = *want_cam_pos.get_mut(i).unwrap();
                assert!(
                    got.translation.distance(want) < 0.1,
                    "failed {}: want: {}, got: {}",
                    name,
                    want,
                    got.translation,
                );
            }
        }
    }

    #[test]
    fn test_calculate_viewports() {
        struct TestCase {
            name: &'static str,
            input_num_cameras: u32,
            want: Vec<Viewport>,
        }

        let tests = [
            TestCase {
                name: "single pane",
                input_num_cameras: 1,
                want: vec![Viewport {
                    physical_position: UVec2::new(0, 0),
                    physical_size: UVec2::new(800, 600),
                    ..default()
                }],
            },
            TestCase {
                name: "two panes",
                input_num_cameras: 2,
                want: vec![
                    Viewport {
                        physical_position: UVec2::new(0, 0),
                        physical_size: UVec2::new(800, 300),
                        ..default()
                    },
                    Viewport {
                        physical_position: UVec2::new(0, 300),
                        physical_size: UVec2::new(800, 300),
                        ..default()
                    },
                ],
            },
        ];

        for TestCase {
            name,
            input_num_cameras,
            want,
        } in tests.into_iter()
        {
            let window = Window {
                resolution: bevy::window::WindowResolution::new(800, 600),
                ..default()
            };

            let actual = calculate_viewports(&window, input_num_cameras);

            assert_eq!(
                actual.len(),
                want.len(),
                "failed {}, actual: {:?}, want: {:?}",
                name,
                actual,
                want
            );
            assert!(
                actual
                    .iter()
                    .enumerate()
                    .all(|(i, element)| viewport_eq(&want[i], element)),
                "failed {}, actual: {:?}, want: {:?}",
                name,
                actual,
                want,
            );
        }
    }
}
