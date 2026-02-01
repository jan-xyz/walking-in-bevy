use avian3d::prelude::*;
use bevy::{light::DirectionalLightShadowMap, prelude::*};

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DirectionalLightShadowMap { size: 4096 })
            .add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn the ground.
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(128., 128.))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        RigidBody::Static,
        Collider::half_space(Vec3::Y),
        Name::new("Ground"),
    ));

    // Spawn a light
    commands.spawn((
        DirectionalLight {
            illuminance: 4000.,
            shadows_enabled: true,
            ..default()
        },
        Transform::default().looking_at(-Vec3::Y, Vec3::Z),
        Name::new("Light"),
    ));

    commands.spawn((
        PointLight::default(),
        Transform::from_xyz(5., 5., 5.),
        Name::new("Point Light"),
    ));
}
