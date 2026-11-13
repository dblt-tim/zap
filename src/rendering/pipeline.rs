use wgpu::{
    BlendState, ColorTargetState, ColorWrites, Device, Face, FragmentState, FrontFace,
    MultisampleState, PipelineCompilationOptions, PipelineLayout, PipelineLayoutDescriptor,
    PolygonMode, PrimitiveState, PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor,
    TextureFormat, VertexState,
};

use crate::rendering::shader::load_shader;

fn make_pipeline_layout(device: &Device) -> PipelineLayout {
    let desc = PipelineLayoutDescriptor {
        label: Some("Pipeline Layout"),
        bind_group_layouts: &[],
        immediate_size: 0,
    };

    let layout = device.create_pipeline_layout(&desc);

    return layout;
}

pub fn make_pipeline(device: &Device, format: &TextureFormat) -> RenderPipeline {
    let shader = load_shader(&device, include_str!("shaders/shader.wgsl"));

    let desc = RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&make_pipeline_layout(&device)),

        vertex: VertexState {
            module: &shader,
            entry_point: Some("vert_main"),
            compilation_options: PipelineCompilationOptions::default(),
            buffers: &[],
        },

        fragment: Some(FragmentState {
            module: &shader,
            entry_point: Some("frag_main"),
            compilation_options: PipelineCompilationOptions::default(),
            targets: &[Some(ColorTargetState {
                format: *format,
                blend: Some(BlendState::REPLACE),
                write_mask: ColorWrites::ALL,
            })],
        }),

        primitive: PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: FrontFace::Ccw,
            cull_mode: Some(Face::Back),
            polygon_mode: PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },

        depth_stencil: None,
        multisample: MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview_mask: None,
        cache: None,
    };

    let pipeline = device.create_render_pipeline(&desc);

    return pipeline;
}
