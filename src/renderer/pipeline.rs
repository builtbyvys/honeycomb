use std::borrow::Cow;
use wgpu::{Device, Queue, Surface};
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct RayMarchingUniforms {
    pub view_position: [f32; 4],
    pub screen_size: [f32; 2],
    pub max_steps: u32,
    pub max_distance: f32,
    pub min_distance: f32,
    padding: [u32; 3],
}

pub struct RayMarchingPipeline {
    pipeline: wgpu::ComputePipeline,
    uniform_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    output_texture: wgpu::Texture,
    dimensions: (u32, u32),
}

impl RayMarchingPipeline {
    pub fn new(
        device: &Device,
        width: u32,
        height: u32,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Ray Marching Compute Shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("../../assets/shaders/ray_march.wgsl"))),
        });

        let uniforms = RayMarchingUniforms {
            view_position: [0.0, 0.0, -5.0, 1.0],
            screen_size: [width as f32, height as f32],
            max_steps: 100,
            max_distance: 100.0,
            min_distance: 0.001,
            padding: [0; 3],
        };

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Ray Marching Uniforms"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let output_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Ray Marching Output Texture"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Ray Marching Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::WriteOnly,
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Ray Marching Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Ray Marching Pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: Some("main"),
            compilation_options: None,
            cache: None,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Ray Marching Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(
                        &output_texture.create_view(&wgpu::TextureViewDescriptor::default())
                    ),
                },
            ],
        });

        Self {
            pipeline,
            uniform_buffer,
            bind_group,
            output_texture,
            dimensions: (width, height),
        }
    }

    pub fn update_uniforms(&self, queue: &Queue, uniforms: RayMarchingUniforms) {
        queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[uniforms])
        );
    }

    pub fn resize(&mut self, device: &Device, width: u32, height: u32) {
        self.dimensions = (width, height);
        
        self.output_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Ray Marching Output Texture"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });

        self.bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Ray Marching Bind Group"),
            layout: &self.pipeline.get_bind_group_layout(0),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(
                        &self.output_texture.create_view(&wgpu::TextureViewDescriptor::default())
                    ),
                },
            ],
        });
    }

    pub fn render(
        &self,
        device: &Device,
        queue: &Queue,
        output_texture: &wgpu::Texture,
    ) {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Ray Marching Encoder"),
        });

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Ray Marching Compute Pass"),
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.pipeline);
            compute_pass.set_bind_group(0, &self.bind_group, &[]);
            compute_pass.dispatch_workgroups(
                (self.dimensions.0 + 7) / 8,
                (self.dimensions.1 + 7) / 8,
                1
            );
        }

        encoder.copy_texture_to_texture(
            wgpu::ImageCopyTexture {
                texture: &self.output_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::ImageCopyTexture {
                texture: output_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::Extent3d {
                width: self.dimensions.0,
                height: self.dimensions.1,
                depth_or_array_layers: 1,
            }
        );

        queue.submit(Some(encoder.finish()));
    }
}

#[derive(Debug)]
pub struct Camera {
    pub position: [f32; 3],
    pub target: [f32; 3],
    pub up: [f32; 3],
    pub aspect: f32,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            position: [0.0, 0.0, -5.0],
            target: [0.0, 0.0, 0.0],
            up: [0.0, 1.0, 0.0],
            aspect: width as f32 / height as f32,
            fov: 45.0,
            near: 0.1,
            far: 100.0,
        }
    }

    pub fn build_view_projection_matrix(&self) -> [[f32; 4]; 4] {
        let view = glam::Mat4::look_at_rh(
            glam::Vec3::from_array(self.position),
            glam::Vec3::from_array(self.target),
            glam::Vec3::from_array(self.up),
        );

        let proj = glam::Mat4::perspective_rh(
            self.fov.to_radians(),
            self.aspect,
            self.near,
            self.far,
        );

        (proj * view).to_cols_array_2d()
    }
}

#[derive(Debug, Clone)]
pub struct SceneConfig {
    pub max_steps: u32,
    pub max_distance: f32,
    pub min_distance: f32,
}

impl Default for SceneConfig {
    fn default() -> Self {
        Self {
            max_steps: 100,
            max_distance: 100.0,
            min_distance: 0.001,
        }
    }
}
