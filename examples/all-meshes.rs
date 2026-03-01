use bevy::{
    asset::RenderAssetUsages,
    camera::visibility::RenderLayers,
    core_pipeline::prepass::{DepthPrepass, NormalPrepass},
    pbr::ExtendedMaterial,
    prelude::*,
    render::{
        // render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
        view::{ColorGrading, Hdr},
    },
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use gen_04_pixels::{
    colors,
    pixelating_plugin::{PixelatedCamera, PixelatedPassLayer, PixelatingPlugin},
    rotators::{circle_rotator_system, light_rotator_system, rotator_system},
    PixelatedExtension,
};
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            EguiPlugin::default(),
            WorldInspectorPlugin::new(),
        ))
        .add_plugins(PixelatingPlugin)
        .add_systems(PostStartup, setup)
        .add_systems(
            Update,
            (
                circle_rotator_system,
                light_rotator_system,
                rotator_system,
                rotate,
            ),
        )
        .run();
}

#[derive(Component)]
struct Shape;

const X_EXTENT: f32 = 14.5;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    pixelated_pass_layer: Res<PixelatedPassLayer>,
    mut pixelated: ResMut<Assets<ExtendedMaterial<StandardMaterial, PixelatedExtension>>>,
) {
    // let debug_material = materials.add(StandardMaterial {
    //     base_color_texture: Some(
    //         images.add(uv_debug_texture()),
    //     ),
    //     ..default()
    // });

    let debug_material = pixelated.add(ExtendedMaterial {
        base: StandardMaterial {
            // base_color: colors::RED,
            base_color_texture: Some(images.add(uv_debug_texture())),
            perceptual_roughness: 1.0,
            ..Default::default()
        },
        extension: PixelatedExtension { quantize_steps: 5 },
    });
    let shapes = [
        meshes.add(Cuboid::default()),
        meshes.add(Cuboid::default()),
        meshes.add(Capsule3d::default()),
        meshes.add(Torus::default()),
        meshes.add(Cylinder::default()),
        meshes.add(Sphere::default()),
    ];

    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        commands.spawn((
            Mesh3d::from(shape),
            MeshMaterial3d::from(debug_material.clone()),
            Transform::from_xyz(
                -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                2.0,
                0.0,
            )
            .with_rotation(Quat::from_rotation_x(-PI / 4.)),
            Shape,
            pixelated_pass_layer.0.clone(),
        ));
    }

    commands.spawn((
        PointLight {
            intensity: 4500000.0,
            range: 1000.,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0),
        RenderLayers::from_layers(&[0, 1]),
    ));

    // ground plane
    commands.spawn((
        Mesh3d::from(meshes.add(Plane3d {
            half_size: Vec2::new(50.0, 50.0),
            ..default()
        })),
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

    commands.spawn((
        Camera3d::default(),
        Hdr,
        Camera {
            // hdr: true,
            ..default()
        },
        Msaa::Off,
        Transform::from_xyz(0.0, 6., 12.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        bevy::core_pipeline::tonemapping::Tonemapping::TonyMcMapface,
        ColorGrading {
            // post_saturation: 1.2,
            ..default()
        },
        // depth prepass is required for pixelated.wgsl
        DepthPrepass,
        // normal prepass is required for pixelated.wgsl
        NormalPrepass,
        // PixelatedCamera causes this camera to be used to generate the
        // pixelated scene
        PixelatedCamera,
    ));
}

fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs() / 2.);
    }
}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::all(),
    )
}
