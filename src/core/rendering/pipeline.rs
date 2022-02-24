use wgpu::{
    RenderPipeline,

    ShaderModuleDescriptor,
    PipelineLayoutDescriptor,
    RenderPipelineDescriptor,
};

use crate::core::{
    Engine,
    rendering::vertex_buffer::Vertex,
};

pub fn ignite_pipeline(engine: &mut Engine, shaders: ShaderModuleDescriptor) -> RenderPipeline {
    let shader = engine.gpu.device.create_shader_module(&shaders);

    let pipeline_layout = engine.gpu.device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let pipeline = engine.gpu.device.create_render_pipeline(&RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[
                Vertex::layout(),
            ],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[wgpu::ColorTargetState { 
                format: engine.window.config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            }],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    });

    pipeline
}