use avian3d::prelude::*;
use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;

#[derive(Component)]
pub struct PlayerModel;

#[derive(Component)]
pub struct CurrentPlayerModel(pub PlayerModelType);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PlayerModelType {
    Donut,
    Cube,
}

pub fn spawn_player_model(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    model_type: PlayerModelType,
    asset_server: &AssetServer,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    match model_type {
        PlayerModelType::Donut => {
            parent.spawn((
                SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("export.glb"))),
                ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh),
                // A sensor shape is not strictly necessary, but without it we'll get weird results.
                TnuaAvian3dSensorShape(Collider::cylinder(0.49, 0.0)),
                PlayerModel,
                Name::new("PlayerModel"),
            ));
        }
        PlayerModelType::Cube => {
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1., 1., 1.))),
                MeshMaterial3d(materials.add(Color::srgb(0.8, 0.2, 0.2))),
                Collider::cuboid(0.5, 0.5, 0.5),
                TnuaAvian3dSensorShape(Collider::cuboid(0.5, 0.5, 0.5)),
                PlayerModel,
                Name::new("PlayerModel"),
            ));
        }
    }
}
