use avian3d::prelude::*;
use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*, scene::SceneInstanceReady};
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;

#[derive(Component)]
pub struct PlayerModel;

#[derive(Component)]
pub struct PlayerColor(pub Color);

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
    color: Color,
) {
    match model_type {
        PlayerModelType::Donut => {
            parent
                .spawn((
                    SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("export.glb"))),
                    ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh),
                    // A sensor shape is not strictly necessary, but without it we'll get weird results.
                    TnuaAvian3dSensorShape(Collider::cylinder(0.49, 0.0)),
                    PlayerModel,
                    Name::new("PlayerModel"),
                ))
                .observe(
                    move |trigger: On<SceneInstanceReady>,
                          children_query: Query<&Children>,
                          material_query: Query<&MeshMaterial3d<StandardMaterial>>,
                          mut commands: Commands,
                          mut materials: ResMut<Assets<StandardMaterial>>| {
                        // Collect all descendants recursively
                        let mut to_visit = vec![trigger.event_target()];
                        while let Some(entity) = to_visit.pop() {
                            // Check if this entity has a material
                            if let Ok(material_handle) = material_query.get(entity) {
                                if let Some(material) = materials.get(material_handle) {
                                    let mut new_material = material.clone();
                                    new_material.base_color = color;
                                    let new_handle = materials.add(new_material);
                                    commands.entity(entity).insert(MeshMaterial3d(new_handle));
                                }
                            }
                            // Queue this entity's children for visiting
                            if let Ok(children) = children_query.get(entity) {
                                to_visit.extend(children.iter());
                            }
                        }
                    },
                );
        }
        PlayerModelType::Cube => {
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1., 1., 1.))),
                MeshMaterial3d(materials.add(color)),
                Collider::cuboid(0.5, 0.5, 0.5),
                TnuaAvian3dSensorShape(Collider::cuboid(0.5, 0.5, 0.5)),
                PlayerModel,
                Name::new("PlayerModel"),
            ));
        }
    }
}
