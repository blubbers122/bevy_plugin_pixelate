use bevy::{
    camera::visibility::RenderLayers,
    core_pipeline::prepass::{DepthPrepass, NormalPrepass},
    light::{NotShadowCaster, NotShadowReceiver},
    pbr::{ExtendedMaterial, OpaqueRendererMethod},
    prelude::*,
    render::view::Hdr,
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use gen_04_pixels::{
    colors,
    pixelating_plugin::{PixelatedCamera, PixelatedPassLayer, PixelatingPlugin},
    rotators::{circle_rotator_system, light_rotator_system, rotator_system, Rotate},
    PixelatedExtension,
};
use std::f32::consts::{FRAC_PI_4, FRAC_PI_8, PI};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            EguiPlugin::default(),
            WorldInspectorPlugin::new(),
        ))
        .add_plugins(PixelatingPlugin)
        .add_systems(PostStartup, (setup_camera, setup_scene))
        .add_systems(
            Update,
            (circle_rotator_system, light_rotator_system, rotator_system),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Hdr,
        Camera {
            // hdr: true,
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 10.0, 15.0))
            .looking_at(Vec3::new(0., 4., 0.), Vec3::Y),
        bevy::core_pipeline::tonemapping::Tonemapping::TonyMcMapface,
        Projection::Orthographic(OrthographicProjection {
            scale: 0.1,
            ..OrthographicProjection::default_3d()
        }),
        // depth prepass is required for pixelated.wgsl
        DepthPrepass,
        // normal prepass is required for pixelated.wgsl
        NormalPrepass,
        // PixelatedCamera causes this camera to be used to generate the
        // pixelated scene
        PixelatedCamera,
        Msaa::Off,
    ));
}

// setup is just responsible for the scene setup
// all camera setup, etc is done by the plugin
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut pixelated: ResMut<Assets<ExtendedMaterial<StandardMaterial, PixelatedExtension>>>,
    asset_server: Res<AssetServer>,
    pixelated_pass_layer: Res<PixelatedPassLayer>,
) {
    // cubes
    commands.spawn((
        Mesh3d::from(meshes.add(Cuboid::from_size(Vec3::splat(1.0)))),
        Transform::from_xyz(6.0, 4., -20.0),
        MeshMaterial3d::from(pixelated.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: colors::RED,
                // can be used in forward or deferred mode.
                opaque_render_method: OpaqueRendererMethod::Auto,
                // in deferred mode, only the PbrInput can be modified (uvs, color and other material properties),
                // in forward mode, the output can also be modified after lighting is applied.
                // see the fragment shader `extended_material.wgsl` for more info.
                // Note: to run in deferred mode, you must also add a `DeferredPrepass` component to the camera and either
                // change the above to `OpaqueRendererMethod::Deferred` or add the `DefaultOpaqueRendererMethod` resource.
                perceptual_roughness: 1.0,
                ..Default::default()
            },
            extension: PixelatedExtension { quantize_steps: 5 },
        })),
        Rotate,
        pixelated_pass_layer.0.clone(),
    ));
    commands.spawn((
        Mesh3d::from(meshes.add(Cuboid::from_size(Vec3::splat(2.0)))),
        Transform::from_xyz(0.0, 0., 0.0),
        MeshMaterial3d::from(pixelated.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: colors::GREEN,
                // can be used in forward or deferred mode.
                opaque_render_method: OpaqueRendererMethod::Auto,
                // in deferred mode, only the PbrInput can be modified (uvs, color and other material properties),
                // in forward mode, the output can also be modified after lighting is applied.
                // see the fragment shader `extended_material.wgsl` for more info.
                // Note: to run in deferred mode, you must also add a `DeferredPrepass` component to the camera and either
                // change the above to `OpaqueRendererMethod::Deferred` or add the `DefaultOpaqueRendererMethod` resource.
                perceptual_roughness: 1.0,
                ..Default::default()
            },
            extension: PixelatedExtension { quantize_steps: 15 },
        })),
        Rotate,
        pixelated_pass_layer.0.clone(),
    ));
    commands.spawn((
        Mesh3d::from(meshes.add(Torus {
            major_radius: 4.,
            minor_radius: 2.,
            ..default()
        })),
        Transform::from_xyz(0.0, 0., 0.0),
        MeshMaterial3d::from(pixelated.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: colors::GREEN,
                // can be used in forward or deferred mode.
                opaque_render_method: OpaqueRendererMethod::Auto,
                // in deferred mode, only the PbrInput can be modified (uvs, color and other material properties),
                // in forward mode, the output can also be modified after lighting is applied.
                // see the fragment shader `extended_material.wgsl` for more info.
                // Note: to run in deferred mode, you must also add a `DeferredPrepass` component to the camera and either
                // change the above to `OpaqueRendererMethod::Deferred` or add the `DefaultOpaqueRendererMethod` resource.
                perceptual_roughness: 1.0,
                ..Default::default()
            },
            extension: PixelatedExtension { quantize_steps: 15 },
        })),
        Rotate,
        pixelated_pass_layer.0.clone(),
    ));
    commands.spawn((
        Mesh3d::from(meshes.add(Cylinder {
            radius: 2.,
            half_height: 2.,
            ..default()
        })),
        Transform::from_xyz(-15.0, 2., 0.0),
        MeshMaterial3d::from(pixelated.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: colors::SAPPHIRE,
                // can be used in forward or deferred mode.
                opaque_render_method: OpaqueRendererMethod::Auto,
                // in deferred mode, only the PbrInput can be modified (uvs, color and other material properties),
                // in forward mode, the output can also be modified after lighting is applied.
                // see the fragment shader `extended_material.wgsl` for more info.
                // Note: to run in deferred mode, you must also add a `DeferredPrepass` component to the camera and either
                // change the above to `OpaqueRendererMethod::Deferred` or add the `DefaultOpaqueRendererMethod` resource.
                perceptual_roughness: 1.0,
                emissive: colors::SAPPHIRE.into(),
                ..Default::default()
            },
            extension: PixelatedExtension { quantize_steps: 15 },
        })),
        // Rotate,
        pixelated_pass_layer.0.clone(),
    ));
    commands.spawn((
        Mesh3d::from(meshes.add(Cuboid::from_size(Vec3::splat(2.0)))),
        Transform::from_xyz(5.0, 4., -5.0),
        MeshMaterial3d::from(pixelated.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: colors::RED,
                // can be used in forward or deferred mode.
                opaque_render_method: OpaqueRendererMethod::Auto,
                // in deferred mode, only the PbrInput can be modified (uvs, color and other material properties),
                // in forward mode, the output can also be modified after lighting is applied.
                // see the fragment shader `extended_material.wgsl` for more info.
                // Note: to run in deferred mode, you must also add a `DeferredPrepass` component to the camera and either
                // change the above to `OpaqueRendererMethod::Deferred` or add the `DefaultOpaqueRendererMethod` resource.
                perceptual_roughness: 1.0,
                ..Default::default()
            },
            extension: PixelatedExtension { quantize_steps: 15 },
        })),
        Rotate,
        pixelated_pass_layer.0.clone(),
    ));

    for i in 0..10 {
        commands.spawn((
            Mesh3d::from(meshes.add(Cuboid::from_size(Vec3::splat(2.0)))),
            Transform::from_xyz(-8.0, 2. * i as f32 + 0.5, -4.0)
                .with_rotation(Quat::from_rotation_y(i as f32 * FRAC_PI_8)),
            MeshMaterial3d::from(pixelated.add(ExtendedMaterial {
                base: StandardMaterial {
                    base_color: colors::LAVENDER,
                    // can be used in forward or deferred mode.
                    opaque_render_method: OpaqueRendererMethod::Auto,
                    // in deferred mode, only the PbrInput can be modified (uvs, color and other material properties),
                    // in forward mode, the output can also be modified after lighting is applied.
                    // see the fragment shader `extended_material.wgsl` for more info.
                    // Note: to run in deferred mode, you must also add a `DeferredPrepass` component to the camera and either
                    // change the above to `OpaqueRendererMethod::Deferred` or add the `DefaultOpaqueRendererMethod` resource.
                    perceptual_roughness: 1.0,
                    ..Default::default()
                },
                extension: PixelatedExtension { quantize_steps: 15 },
            })),
            // Rotate,
            pixelated_pass_layer.0.clone(),
        ));
    }
    commands.spawn((
        Mesh3d::from(meshes.add(Sphere {
            radius: 1.,
            ..default()
        })),
        Transform::from_xyz(6.0, 4., 0.0),
        MeshMaterial3d::from(pixelated.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: colors::RED,
                // can be used in forward or deferred mode.
                opaque_render_method: OpaqueRendererMethod::Auto,
                // in deferred mode, only the PbrInput can be modified (uvs, color and other material properties),
                // in forward mode, the output can also be modified after lighting is applied.
                // see the fragment shader `extended_material.wgsl` for more info.
                // Note: to run in deferred mode, you must also add a `DeferredPrepass` component to the camera and either
                // change the above to `OpaqueRendererMethod::Deferred` or add the `DefaultOpaqueRendererMethod` resource.
                perceptual_roughness: 1.0,
                ..Default::default()
            },
            extension: PixelatedExtension { quantize_steps: 5 },
        })),
        Rotate,
        pixelated_pass_layer.0.clone(),
    ));

    commands.spawn((
        Mesh3d::from(meshes.add(Plane3d::default().mesh().size(30., 30.).build())),
        Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_rotation_y(FRAC_PI_4)),
        MeshMaterial3d::from(pixelated.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: colors::BASE,
                // can be used in forward or deferred mode.
                opaque_render_method: OpaqueRendererMethod::Auto,
                // in deferred mode, only the PbrInput can be modified (uvs, color and other material properties),
                // in forward mode, the output can also be modified after lighting is applied.
                // see the fragment shader `extended_material.wgsl` for more info.
                // Note: to run in deferred mode, you must also add a `DeferredPrepass` component to the camera and either
                // change the above to `OpaqueRendererMethod::Deferred` or add the `DefaultOpaqueRendererMethod` resource.
                perceptual_roughness: 1.0,
                ..Default::default()
            },
            extension: PixelatedExtension { quantize_steps: 15 },
        })),
        pixelated_pass_layer.0.clone(),
    ));

    let parallax_material = pixelated.add(ExtendedMaterial {
        base: StandardMaterial {
            perceptual_roughness: 0.4,
            base_color: colors::LAVENDER,
            // base_color_texture: Some(
            //     asset_server
            //         .load("parallax/cube_color.png"),
            // ),
            normal_map_texture: Some(asset_server.load("parallax/cube_normal.png")),
            // The depth map is a greyscale texture where black is the highest level and
            // white the lowest.
            depth_map: Some(asset_server.load("parallax/cube_depth.png")),
            parallax_depth_scale: 0.09,
            parallax_mapping_method: ParallaxMappingMethod::Occlusion,
            max_parallax_layer_count: 5.0_f32.exp2(),
            ..default()
        },
        extension: PixelatedExtension { quantize_steps: 15 },
    });
    commands.spawn((
        Mesh3d::from(
            meshes.add(
                // NOTE: for normal maps and depth maps to work, the mesh
                // needs tangents generated.
                Mesh::from(Cuboid::from_size(Vec3::splat(4.0)))
                    .with_generated_tangents()
                    .unwrap(),
            ),
        ),
        MeshMaterial3d::from(parallax_material.clone()),
        pixelated_pass_layer.0.clone(),
        Rotate,
    ));

    // Light
    // NOTE: Currently lights are shared between passes - see https://github.com/bevyengine/bevy/issues/3462
    for i in 0..10 {
        let transform = Transform::from_xyz(i as f32 * 10.0, 4.0, (i as f32 * 3.) - 15.);
        let light_color = Color::Lcha(bevy::color::Lcha::new(1., 1., 360. / 10. * i as f32, 1.));
        commands
            .spawn((
                transform,
                PointLight {
                    // intensity: (),
                    // range: (),
                    radius: 0.5,
                    color: light_color,
                    shadows_enabled: true,
                    ..default()
                },
                RenderLayers::layer(0).with(1),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Mesh3d::from(meshes.add(Sphere {
                        radius: 0.5,
                        ..default()
                    })),
                    MeshMaterial3d::from(pixelated.add(ExtendedMaterial {
                        base: StandardMaterial {
                            base_color: light_color,
                            unlit: true,
                            ..Default::default()
                        },
                        extension: PixelatedExtension { quantize_steps: 5 },
                    })),
                    NotShadowCaster,
                    NotShadowReceiver,
                    pixelated_pass_layer.0.clone(),
                ));
            });
    }

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
        affects_lightmapped_meshes: true, //??
    });
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 20.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.) + Quat::from_rotation_z(-PI),
            ..default()
        },
        // pixelated_pass_layer.0.clone(),
    ));
}
