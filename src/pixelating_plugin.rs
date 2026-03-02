use crate::PixelatedExtension;
use bevy::{
    asset::embedded_asset,
    camera::{visibility::RenderLayers, ImageRenderTarget, RenderTarget},
    core_pipeline::tonemapping::Tonemapping,
    image::ImageSampler,
    light::{NotShadowCaster, NotShadowReceiver},
    pbr::{ExtendedMaterial, StandardMaterial},
    prelude::*,
    render::render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
};

/// add PixelatedCamera to your 3d camera to
/// use it as the source of the pixelated effect
#[derive(Component)]
pub struct PixelatedCamera {
    pub resolution: UVec2,
}

impl Default for PixelatedCamera {
    fn default() -> Self {
        Self {
            resolution: UVec2::new(480, 270),
        }
    }
}

impl PixelatedCamera {
    pub fn new(resolution: UVec2) -> Self {
        Self { resolution }
    }
}
pub struct PixelatingPlugin;

impl Plugin for PixelatingPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "pixelated.wgsl");
        // This specifies the layer used for the first pass, which will be attached to the first pass camera and cube.
        let pixelated_pass_layer = RenderLayers::layer(1);
        app.insert_resource(PixelatedPassLayer(pixelated_pass_layer));
        app.add_plugins(
            MaterialPlugin::<ExtendedMaterial<StandardMaterial, PixelatedExtension>> {
                // prepass_enabled: true,
                ..default()
            },
        )
        .add_systems(Update, setup);
        // .add_systems(Update, configure_pixelated_camera);
    }
}

// for saving screenshots of image
#[derive(Resource)]
struct FirstPassImage(Handle<Image>);

#[derive(Resource, Deref, Clone)]
pub struct PixelatedPassLayer(pub RenderLayers);

// Marks the main pass cube, to which the texture is applied.
#[derive(Component)]
struct MainPassDisplay;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut pixelated_cameras: Query<(Entity, &mut Camera, &PixelatedCamera), Added<PixelatedCamera>>,
    pixelated_pass_layer: Res<PixelatedPassLayer>,
) {
    let Ok((entity, mut camera, pixelated_cam)) = pixelated_cameras.single() else {
        return;
    };
    let size = Extent3d {
        width: pixelated_cam.resolution.x,
        height: pixelated_cam.resolution.y,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        sampler: ImageSampler::nearest(),
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);
    commands.insert_resource(FirstPassImage(image_handle.clone()));

    let mesh = Mesh::from(Rectangle::new(16.0 * 1.5, 9.0 * 1.5));

    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(image_handle.clone()),
            unlit: true,
            ..default()
        })),
        Transform::from_xyz(0., 0., 0.),
        NotShadowCaster,
        NotShadowReceiver,
        MainPassDisplay,
        RenderLayers::layer(0),
    ));

    // The main pass camera.
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        Tonemapping::TonyMcMapface,
    ));
    for (entity, mut camera, pixelated_cam) in &mut pixelated_cameras {
        camera.order = -1;
        commands.entity(entity).insert((
            pixelated_pass_layer.0.clone(),
            RenderTarget::Image(ImageRenderTarget {
                handle: image_handle.clone(),
                scale_factor: 1.0,
            }),
        ));
    }
}
