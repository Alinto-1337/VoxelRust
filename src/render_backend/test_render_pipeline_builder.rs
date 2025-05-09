use std::env::current_dir;
use std::{default, fs};

use wgpu::{PipelineCache, PipelineCompilationOptions};

pub struct TestRenderPipelineBuilder 
{
    shader_filename: String,
    vertex_entry: String,
    fragment_entry: String,
    pixel_format: wgpu::TextureFormat,
}


impl TestRenderPipelineBuilder {
    
    pub fn new() -> Self 
    {
        TestRenderPipelineBuilder 
        {
            shader_filename: "dummy".to_string(),
            vertex_entry: "dummy".to_string(),
            fragment_entry: "dummy".to_string(),
            pixel_format: wgpu::TextureFormat::Rgba8Unorm,
        }
    }


    pub fn set_shader_module(&mut self, shader_filename: &str, vertex_entry: &str, fragment_entry: &str) 
    {
        self.shader_filename = shader_filename.to_string();
        self.vertex_entry = vertex_entry.to_string();
        self.fragment_entry = fragment_entry.to_string();
    }

    pub fn set_pixel_format(&mut self, pixel_format: wgpu::TextureFormat) 
    {
        self.pixel_format = pixel_format;
    }


    pub fn build_pipeline(&self, device: &wgpu::Device) -> wgpu::RenderPipeline
    {
        let mut filepath = current_dir().unwrap();
        filepath.push("src/");
        filepath.push(self.shader_filename.as_str());
        let filepath = filepath.into_os_string().into_string().unwrap();
        let source_code = fs::read_to_string(filepath).expect("Can't read source code!");


        // Make a shader module
        let shader_module_descriptor = wgpu::ShaderModuleDescriptor {
            label: Some("Shader Module"),
            source: wgpu::ShaderSource::Wgsl(source_code.into()),
        };
        let shader_module = device.create_shader_module(shader_module_descriptor);


        // Make a pipeline Layout
        let pipeline_layout_descriptor = wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        };
        let pipeline_layout = device.create_pipeline_layout(&pipeline_layout_descriptor);


        // set Render Targets
        let render_targets = [Some(wgpu::ColorTargetState {
            format: self.pixel_format,
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL,
        })];


        // Create Render pipeline
        let render_pipeline_descriptor = wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            cache: None,

            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: Some(self.vertex_entry.as_str()),
                buffers: &[],
                compilation_options: PipelineCompilationOptions::default(),
            },

            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },

            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: Some(&self.fragment_entry),
                targets: &render_targets,
                compilation_options: PipelineCompilationOptions::default(),
            }),

            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        };

        device.create_render_pipeline(&render_pipeline_descriptor)

    }



}
