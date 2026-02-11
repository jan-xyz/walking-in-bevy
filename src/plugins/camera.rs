use bevy::{camera::Viewport, prelude::*, window::WindowResized};

use crate::plugins::player::{Player, Player1, Player2};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_systems(PostUpdate, (follow_player1, follow_player2));
        app.add_systems(Update, update_viewport);
    }
}

#[derive(Component)]
struct PlayerCamera1;

#[derive(Component)]
struct PlayerCamera2;

fn setup_camera(mut commands: Commands) {
    let camera1 = Camera {
        order: 0,
        ..default()
    };
    // Spawn a camera
    commands.spawn((
        Camera3d::default(),
        camera1,
        Transform::from_xyz(0., 16., 40.).looking_at(Vec3::new(0., 10., 0.), Vec3::Y),
        PlayerCamera1,
        Name::new("Player Camera 1"),
    ));

    let camera2 = Camera {
        order: 1,
        ..default()
    };

    // Spawn a camera
    commands.spawn((
        Camera3d::default(),
        camera2,
        Transform::from_xyz(0., 16., 40.).looking_at(Vec3::new(10., 10., 0.), Vec3::Y),
        PlayerCamera2,
        Name::new("Player Camera 2"),
    ));
}

fn update_viewport(
    mut resize_events: MessageReader<WindowResized>,
    windows: Query<&Window>,
    mut camera1: Query<&mut Camera, With<PlayerCamera1>>,
    mut camera2: Query<&mut Camera, (With<PlayerCamera2>, Without<PlayerCamera1>)>,
) {
    if let Some(event) = resize_events.read().last() {
        let Ok(window) = windows.get(event.window) else {
            return;
        };
        let width = window.physical_width();
        let height = window.physical_height();

        // Update Player1Camera viewport (top half)
        let Ok(mut camera1) = camera1.single_mut() else {
            return;
        };
        camera1.viewport = Some(Viewport {
            physical_size: UVec2::new(width, height / 2),
            physical_position: UVec2::new(0, height / 2),
            ..Default::default()
        });
        // Update Player2Camera viewport (bottom half)
        let Ok(mut camera2) = camera2.single_mut() else {
            return;
        };
        camera2.viewport = Some(Viewport {
            physical_size: UVec2::new(width, height / 2),
            physical_position: UVec2::new(0, 0),
            ..Default::default()
        });
    }
}

fn follow_player1(
    time: Res<Time>,
    player: Query<&Transform, With<Player1>>,
    mut camera: Query<&mut Transform, (With<PlayerCamera1>, Without<Player>, Without<Player1>)>,
) {
    let Ok(player_transform) = player.single() else {
        return;
    };
    let Ok(mut cam_transform) = camera.single_mut() else {
        return;
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

fn follow_player2(
    time: Res<Time>,
    player: Query<&Transform, With<Player2>>,
    mut camera: Query<&mut Transform, (With<PlayerCamera2>, Without<Player>, Without<Player2>)>,
) {
    let Ok(player_transform) = player.single() else {
        return;
    };
    let Ok(mut cam_transform) = camera.single_mut() else {
        return;
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
