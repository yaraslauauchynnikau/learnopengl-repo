use derive_builder::Builder;
use wgpu::ColorTargetState;
use wgpu::IndexFormat;
use wgpu::PipelineCompilationOptions;
use wgpu::ShaderModule;
use wgpu::VertexBufferLayout;

#[derive(Builder, Clone)]
pub struct ShaderPayload<'a> {
    pub module: &'a ShaderModule,
    #[builder(default)]
    pub entry_point: Option<&'a str>,
    pub compilation_options: PipelineCompilationOptions<'a>,
}

#[derive(Builder)]
pub struct VertexShaderPayload<'a> {
    pub body: ShaderPayload<'a>,
    pub buffers: &'a [VertexBufferLayout<'a>],
}
#[derive(Builder)]
pub struct FragmentShaderPayload<'a> {
    pub body: ShaderPayload<'a>,
    pub targets: &'a [Option<ColorTargetState>],
}

#[derive(Builder)]
pub struct PrimitivePayload {
    pub topology: wgpu::PrimitiveTopology,
    #[builder(default)]
    pub strip_index_format: Option<IndexFormat>,
    pub front_face: wgpu::FrontFace,
    #[builder(default)]
    pub cull_mode: Option<wgpu::Face>,
    pub polygon_mode: wgpu::PolygonMode,
    pub unclipped_depth: bool,
    pub conservative: bool,
}

#[derive(Builder)]
pub struct MultisamplePayload {
    pub count: u32,
    pub mask: u64,
    pub aplha_to_coverage_enabled: bool,
}
