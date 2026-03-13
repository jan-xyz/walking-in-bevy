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
    let tile_size = 64.0;
    let colors = [
        Color::srgb(0.8, 0.2, 0.2),
        Color::srgb(0.2, 0.8, 0.2),
        Color::srgb(0.2, 0.2, 0.8),
        Color::srgb(0.8, 0.8, 0.2),
    ];
    let offsets = [
        Vec3::new(-tile_size / 2.0, 0.0, -tile_size / 2.0),
        Vec3::new(tile_size / 2.0, 0.0, -tile_size / 2.0),
        Vec3::new(-tile_size / 2.0, 0.0, tile_size / 2.0),
        Vec3::new(tile_size / 2.0, 0.0, tile_size / 2.0),
    ];
    for (color, offset) in colors.iter().zip(offsets.iter()) {
        commands.spawn((
            Mesh3d(meshes.add(Plane3d::default().mesh().size(tile_size, tile_size))),
            MeshMaterial3d(materials.add(*color)),
            Transform::from_translation(*offset),
            Name::new("Ground Tile"),
        ));
    }
    commands.spawn((
        RigidBody::Static,
        Collider::half_space(Vec3::Y),
        Name::new("Ground Collider"),
    ));

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
