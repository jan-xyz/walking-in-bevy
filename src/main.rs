use avian3d::prelude::*;
use bevy::{pbr::DirectionalLightShadowMap, prelude::*};

use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::*;

fn main() {
    App::new()
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            TnuaControllerPlugin::new(FixedUpdate),
            TnuaAvian3dPlugin::new(FixedUpdate),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            apply_controls.in_set(TnuaUserControlsSystemSet),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Spawn the ground.
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(128., 128.))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        RigidBody::Static,
        Collider::half_space(Vec3::Y),
    ));

    // Add a physics body using Avian 3D
    commands.spawn((
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("export.glb"))),
        Transform::from_xyz(0., 2., 0.),
        // The player character needs to be configured as a dynamic rigid body of the physics
        // engine.
        RigidBody::Dynamic,
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh),
        // This is Tnua's interface component.
        TnuaController::default(),
        // A sensor shape is not strictly necessary, but without it we'll get weird results.
        TnuaAvian3dSensorShape(Collider::cylinder(0.49, 0.0)),
        // Tnua can fix the rotation, but the character will still get rotated before it can do so.
        // By locking the rotation we can prevent this.
        LockedAxes::ROTATION_LOCKED,
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

    // Spawn a camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 16., 40.).looking_at(Vec3::new(0., 10., 0.), Vec3::Y),
    ));

    commands.spawn((PointLight::default(), Transform::from_xyz(5., 5., 5.)));
}

// Movement System
fn apply_controls(keyboard: Res<ButtonInput<KeyCode>>, mut query: Query<&mut TnuaController>) {
    let Ok(mut controller) = query.get_single_mut() else {
        return;
    };

    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::ArrowUp) {
        direction -= Vec3::Z;
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        direction += Vec3::Z;
    }
    if keyboard.pressed(KeyCode::ArrowLeft) {
        direction -= Vec3::X;
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        direction += Vec3::X;
    }

    // Feed the basis every frame. Even if the player doesn't move - just use `desired_velocity:
    // Vec3::ZERO`. `TnuaController` starts without a basis, which will make the character collider
    // just fall.
    controller.basis(TnuaBuiltinWalk {
        // The `desired_velocity` determines how the character will move.
        desired_velocity: direction.normalize_or_zero() * 10.0,
        // The `float_height` must be greater (even if by little) from the distance between the
        // character's center and the lowest point of its collider.
        float_height: 1.5,
        // `TnuaBuiltinWalk` has many other fields for customizing the movement - but they have
        // sensible defaults. Refer to the `TnuaBuiltinWalk`'s documentation to learn what they do.
        ..Default::default()
    });

    // Feed the jump action every frame as long as the player holds the jump button. If the player
    // stops holding the jump button, simply stop feeding the action.
    if keyboard.pressed(KeyCode::Space) {
        controller.action(TnuaBuiltinJump {
            // The height is the only mandatory field of the jump button.
            height: 4.0,
            // `TnuaBuiltinJump` also has customization fields with sensible defaults.
            ..Default::default()
        });
    }
}
