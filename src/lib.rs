use bevy::{
    pbr::MaterialExtension, prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef,
};
pub mod colors;
pub mod pixelating_plugin;
pub mod rotators;

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct PixelatedExtension {
    // We need to ensure that the bindings of the base material and the extension do not conflict,
    // so we start from binding slot 100, leaving slots 0-99 for the base material.
    #[uniform(100)]
    pub quantize_steps: u32,
}

impl MaterialExtension for PixelatedExtension {
    fn fragment_shader() -> ShaderRef {
        "pixelated.wgsl".into()
    }

    fn enable_prepass() -> bool {
        true
    }

    fn enable_shadows() -> bool {
        true
    }
}
