use bevy::{
    camera::{primitives::Frustum, visibility::RenderLayers},
    core_pipeline::prepass::{DepthPrepass, NormalPrepass},
    gltf::GltfNode,
    light::{NotShadowCaster, NotShadowReceiver},
    pbr::ExtendedMaterial,
    prelude::*,
    render::view::{ColorGrading, Hdr},
};
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt},
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
// use bevy_scene_hook::{HookPlugin, HookedSceneBundle, SceneHook};
use bevy_plugin_pixelate::{
    colors,
    pixelating_plugin::{PixelatedCamera, PixelatedPassLayer, PixelatingPlugin},
    rotators::{circle_rotator_system, light_rotator_system, rotator_system, CircleRotate, Rotate},
    PixelatedExtension,
};
use std::f32::consts::{FRAC_PI_4, PI};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            EguiPlugin::default(),
            WorldInspectorPlugin::new(),
            // HookPlugin,
        ))
        .add_plugins(PixelatingPlugin)
        .add_systems(
            Update,
            (
                circle_rotator_system,
                light_rotator_system,
                rotator_system,
                apply_scene_hooks,
            ),
        )
        .init_state::<MyStates>()
        .add_loading_state(
            LoadingState::new(MyStates::AssetLoading)
                .continue_to_state(MyStates::Next)
                .load_collection::<CarAssets>(),
        )
        .add_systems(
            OnEnter(MyStates::Next),
            (setup_camera, setup_scene, setup_lights),
        )
        .run();
}

#[derive(AssetCollection, Resource)]
struct CarAssets {
    // #[asset(path = "car-kit/taxi.glb#Node0")]
    // taxi: Handle<GltfNode>,
    #[asset(path = "car-kit/taxi.glb#Scene0")]
    taxi_scene: Handle<Scene>,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum MyStates {
    #[default]
    AssetLoading,
    Next,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d {
            // clear_color: ClearColorConfig::Custom(
            //     colors::SKY,
            // ),
            ..default()
        },
        Hdr,
        Camera { ..default() },
        Transform::from_translation(Vec3::new(0.0, 10.0, 15.0))
            .looking_at(Vec3::new(0., 4., 0.), Vec3::Y),
        bevy::core_pipeline::tonemapping::Tonemapping::TonyMcMapface,
        ColorGrading {
            // post_saturation: 1.8,
            ..default()
        },
        Projection::Orthographic(OrthographicProjection {
            // near: todo!(),
            // far: todo!(),
            // viewport_origin: todo!(),
            // scaling_mode: todo!(),
            scale: 0.1,
            // area: todo!()
            ..OrthographicProjection::default_3d()
        }),
        Msaa::Off,
        // depth prepass is required for pixelated.wgsl
        DepthPrepass,
        // normal prepass is required for pixelated.wgsl
        NormalPrepass,
        // PixelatedCamera causes this camera to be used to generate the
        // pixelated scene
        PixelatedCamera::default(),
    ));
}

// setup is just responsible for the scene setup
// all camera setup, etc is done by the plugin
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut pixelated: ResMut<Assets<ExtendedMaterial<StandardMaterial, PixelatedExtension>>>,
    pixelated_pass_layer: Res<PixelatedPassLayer>,
    cars: Res<CarAssets>,
) {
    commands.spawn((
        Mesh3d::from(
            meshes.add(
                Mesh::try_from(Plane3d {
                    half_size: Vec2::new(30., 30.),
                    ..default()
                })
                .unwrap(),
            ),
        ),
        Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_rotation_y(FRAC_PI_4)),
        MeshMaterial3d::from(pixelated.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: colors::BASE,
                perceptual_roughness: 1.0,
                ..Default::default()
            },
            extension: PixelatedExtension { quantize_steps: 15 },
        })),
        pixelated_pass_layer.0.clone(),
    ));
    // cubes
    commands.spawn((
        Mesh3d::from(meshes.add(Mesh::from(Cuboid::default()))),
        Transform::from_xyz(6.0, 4., -20.0),
        MeshMaterial3d::from(pixelated.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: colors::RED,
                perceptual_roughness: 1.0,
                ..Default::default()
            },
            extension: PixelatedExtension { quantize_steps: 5 },
        })),
        Rotate,
        pixelated_pass_layer.0.clone(),
    ));
    commands.spawn((
        SceneRoot(cars.taxi_scene.clone()),
        Transform::from_xyz(0., 0., 0.).with_rotation(Quat::from_rotation_y(-FRAC_PI_4)),
        pixelated_pass_layer.0.clone(),
        CircleRotate,
    ));
}

fn setup_lights(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    pixelated_pass_layer: Res<PixelatedPassLayer>,
) {
    // Light
    // NOTE: Currently lights are shared between passes - see https://github.com/bevyengine/bevy/issues/3462
    for i in 0..10 {
        let transform = Transform::from_xyz(i as f32 * 10.0, 4.0, (i as f32 * 3.) - 15.);
        let light_color = Color::Lcha(bevy::color::Lcha::new(1., 1., 360. / 10. * i as f32, 1.));
        commands
            .spawn((
                transform,
                PointLight {
                    intensity: 400000.,
                    color: light_color,
                    shadows_enabled: true,
                    ..default()
                },
                RenderLayers::from_layers(&[0, 1]),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Mesh3d::from(meshes.add(Sphere {
                        radius: 0.5,
                        ..default()
                    })),
                    MeshMaterial3d::from(materials.add(StandardMaterial {
                        base_color: light_color,
                        unlit: true,
                        ..Default::default()
                    })),
                    NotShadowCaster,
                    NotShadowReceiver,
                    pixelated_pass_layer.0.clone(),
                ));
            });
    }

    commands.spawn(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
        affects_lightmapped_meshes: true,
    });

    commands.spawn((
        DirectionalLight {
            illuminance: 5000.,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 20.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.) + Quat::from_rotation_z(-PI),
            ..default()
        },
        // RenderLayers::from_layers(&[0, 1]),
    ));
}

fn apply_scene_hooks(
    mut commands: Commands,
    query: Query<(Entity, Option<&Name>), (With<Mesh3d>, Without<RenderLayers>)>,
    pixelated_pass_layer: Res<PixelatedPassLayer>,
) {
    for (entity, name) in query.iter() {
        if let Some(name) = name {
            info!("Pixelating: {}", name);
        }
        commands
            .entity(entity)
            .insert(pixelated_pass_layer.0.clone());
    }
}
