use bevy::prelude::*;

use crate::plugins::player::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_systems(PostUpdate, follow_player);
    }
}

fn setup_camera(mut commands: Commands) {
    // Spawn a camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 16., 40.).looking_at(Vec3::new(0., 10., 0.), Vec3::Y),
        Name::new("Main Camera"),
    ));
}

fn follow_player(
    time: Res<Time>,
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
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
    let smoothing = 20.0;
    let t = time.delta_secs() * smoothing;
    cam_transform.translation = cam_transform.translation.lerp(target_translation, t);
    cam_transform.rotation = cam_transform.rotation.slerp(target_rotation, t)
}
