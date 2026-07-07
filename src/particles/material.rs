use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{
        AsBindGroup, BlendComponent, BlendFactor, BlendOperation, BlendState, ColorWrites,
        RenderPipelineDescriptor, SpecializedMeshPipelineError,
    },
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dKey},
    mesh::MeshVertexBufferLayoutRef,
};

/// Custom material for particles with radial gradient shader and additive blending
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct ParticleMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
}

impl ParticleMaterial {
    pub fn new(color: Color) -> Self {
        Self {
            color: color.to_linear(),
        }
    }
}

impl Material2d for ParticleMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/particle_glow.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}