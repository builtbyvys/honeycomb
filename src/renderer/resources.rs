use wgpu::{Instance, Surface, Device, Queue, Adapter};
use crate::window::EngineWindow;

pub struct GPUResources {
    pub surface: Surface,
    pub device: Device,
    pub queue: Queue,
    pub adapter: Adapter,
}

impl GPUResources {
    pub async fn new(window: &EngineWindow) -> Self {
        let instance = Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            backend_options: Default::default(),
            flags: Default::default(),
        });
        
        let surface = unsafe { instance.create_surface(&window.window) }.unwrap();

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await.unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    memory_hints,
                    label: Some("Primary Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None
            )
            .await
            .unwrap();

        Self { 
            surface,
            device,
            queue,
            adapter,
        }
    }
}

#[derive(Debug)]
pub struct Mesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
}

#[derive(Debug)]
pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

#[derive(Debug)]
pub struct Buffer {
    pub buffer: wgpu::Buffer,
    pub size: wgpu::BufferAddress,
}

impl Buffer {
    pub fn new(
        device: &Device,
        contents: &[u8],
        usage: wgpu::BufferUsages,
        label: Option<&str>
    ) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label,
            contents,
            usage,
        });
        let size = contents.len() as wgpu::BufferAddress;

        Self { buffer, size }
    }
}
