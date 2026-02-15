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
