use bytemuck::{Pod, Zeroable};
use cgmath as cg;
use error_stack::{Result, ResultExt};
use mouse_position::mouse_position::Mouse;
use wgpu::{util::DeviceExt, BufferAddress};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
};

const WHITE: [f32; 3] = [1.0, 1.0, 1.0];
const BLACK: [f32; 3] = [0.0, 0.0, 0.0];

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let event_loop = winit::event_loop::EventLoop::new();

    Ok(())

    /*
    event_loop.run(move |event, _, control| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(new_size) => app.resize(new_size),
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => app.resize(*new_inner_size),
            WindowEvent::CloseRequested => control.set_exit(),
            _ => {}
        },
        Event::RedrawRequested(_) => {
            app.update();

            if let Err(err) = app.render() {
                match err.current_context() {
                    wgpu::SurfaceError::Timeout => {}
                    wgpu::SurfaceError::Outdated | wgpu::SurfaceError::Lost => {
                        app.resize(app.size())
                    }
                    wgpu::SurfaceError::OutOfMemory => control.set_exit(),
                }
            }
        }
        Event::MainEventsCleared => window.request_redraw(),
        _ => {}
    });
    */
}

struct App {
    // surface: wgpu::Surface<'static>,
    // surface_config: wgpu::SurfaceConfiguration,
    // device: wgpu::Device,
    // queue: wgpu::Queue,
    // pipeline: wgpu::RenderPipeline,
    // vertex_buffer: wgpu::Buffer,
    // index_buffer: wgpu::Buffer,
    // instance_buffer: wgpu::Buffer,
    // cursor_x: u32,
    // cursor_y: u32,
    // needs_rerender: bool,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        event_loop.create_window(winit::window::WindowAttributes::new());

        todo!()
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        todo!()
    }
}

impl Ap {
    fn new() -> Self {
        Self {}
    }
}

/*
impl App {
    async fn new(window: &winit::window::Window) -> Result<Self, InitModelError> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());

        let surface = unsafe { instance.create_surface(&window) }
            .change_context(InitModelError)
            .attach_printable("creating surface")?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .ok_or(InitModelError)
            .attach_printable("could not obtain an adapter")?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .change_context(InitModelError)
            .attach_printable("requesting device")?;

        let size = window.inner_size();

        let surface_config = surface
            .get_default_config(&adapter, size.width, size.height)
            .ok_or(InitModelError)
            .attach_printable("getting default surface config")?;

        surface.configure(&device, &surface_config);

        let shader = device.create_shader_module(wgpu::include_wgsl!("./shader.wgsl"));

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    Vertex::vertex_buffer_layout(),
                    Instance::vertex_buffer_layout(),
                ],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        let vertex_buffer = Vertex::vertex_buffer(&device);

        let index_buffer = Vertex::index_buffer(&device);

        let instance_buffer = Instance::instance_buffer(&device);

        Ok(Self {
            surface,
            surface_config,
            device,
            queue,
            pipeline,
            vertex_buffer,
            index_buffer,
            instance_buffer,
            cursor_x: 0,
            cursor_y: 0,
            needs_rerender: true,
        })
    }

    fn size(&self) -> PhysicalSize<u32> {
        PhysicalSize {
            width: self.surface_config.width,
            height: self.surface_config.height,
        }
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.needs_rerender = true;

        if new_size.width & new_size.height == 0 {
            return;
        }

        self.surface_config.width = new_size.width;
        self.surface_config.height = new_size.height;

        self.surface.configure(&self.device, &self.surface_config);
    }

    fn update(&mut self) {
        if let Mouse::Position { x, y } = Mouse::get_mouse_position() {
            let x = x as u32;
            let y = y as u32;

            if self.cursor_x == x && self.cursor_y == y {
                return;
            }

            self.cursor_x = x;
            self.cursor_y = y;
            self.needs_rerender = true;

            write_crosshair_instance_buffer(
                &self.queue,
                &self.instance_buffer,
                x,
                y,
                self.size(),
                32,
                128,
            );
        }
    }

    fn render(&self) -> Result<(), wgpu::SurfaceError> {
        if !self.needs_rerender {
            return Ok(());
        }

        let frame = self.surface.get_current_texture()?;

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..Vertex::INDICES.len() as u32, 0, 0..4);

        drop(render_pass);

        let command_buffer = encoder.finish();

        self.queue.submit(Some(command_buffer));
        frame.present();

        Ok(())
    }
}


#[derive(Clone, Copy, Debug, thiserror::Error)]
#[error("failed to initialize app model")]
struct InitModelError;

#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

impl Vertex {
    const VERTICES: [Vertex; 4] = [
        Vertex {
            position: [1.0, 1.0],
            color: WHITE,
        },
        Vertex {
            position: [-1.0, 1.0],
            color: BLACK,
        },
        Vertex {
            position: [-1.0, -1.0],
            color: BLACK,
        },
        Vertex {
            position: [1.0, -1.0],
            color: WHITE,
        },
    ];

    const INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];

    const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x3];

    fn vertex_buffer_layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }

    fn vertex_buffer(device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("vertex buffer"),
            contents: bytemuck::cast_slice(&Self::VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }

    fn index_buffer(device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("index buffer"),
            contents: bytemuck::cast_slice(&Self::INDICES),
            usage: wgpu::BufferUsages::INDEX,
        })
    }
}

#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod)]
struct Instance {
    transform: [f32; 4 * 4],
}

impl Default for Instance {
    fn default() -> Self {
        let transform = cgmath::Matrix4::from_translation([0.0, 0.0, 0.0].into());

        Self {
            transform: *transform.as_ref(),
        }
    }
}

impl Instance {
    const ATTRIBUTES: [wgpu::VertexAttribute; 4] = wgpu::vertex_attr_array![
      2 => Float32x4,
      3 => Float32x4,
      4 => Float32x4,
      5 => Float32x4,
    ];

    fn vertex_buffer_layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBUTES,
        }
    }

    fn instance_buffer(device: &wgpu::Device) -> wgpu::Buffer {
        let data = [Self::default(); 4];

        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("instance buffer"),
            contents: bytemuck::cast_slice(&data),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        })
    }
}

fn write_crosshair_instance_buffer(
    queue: &wgpu::Queue,
    instance_buffer: &wgpu::Buffer,
    x: u32,
    y: u32,
    window_size: PhysicalSize<u32>,
    crosshair_thickness: u32,
    crosshair_gap: u32,
) {
    let PhysicalSize { width, height } = window_size;

    let x = x as f32;
    let y = y as f32;
    let width = width as f32;
    let height = height as f32;
    let crosshair_thickness = crosshair_thickness as f32;
    let crosshair_gap = crosshair_gap as f32;

    let x_thickness_scale_mat =
        cgmath::Matrix4::from_nonuniform_scale(1.0, crosshair_thickness / height, 1.0);

    let y_thickness_scale_mat =
        cgmath::Matrix4::from_nonuniform_scale(crosshair_thickness / width, 1.0, 1.0);

    let x_translation_mat = cgmath::Matrix4::from_translation([1.0, 0.0, 0.0].into());

    let left_x_translation_mat =
        cgmath::Matrix4::from_translation([-1.0, 1.0 - 2.0 * y / height, 0.0].into());

    let left_width_scale_mat =
        cgmath::Matrix4::from_nonuniform_scale((x - crosshair_gap) / width, 1.0, 1.0);

    let left_instance = Instance {
        transform: *(left_x_translation_mat
            * left_width_scale_mat
            * x_translation_mat
            * x_thickness_scale_mat)
            .as_ref(),
    };

    let right_width_scale_mat =
        cgmath::Matrix4::from_nonuniform_scale((width - x - crosshair_gap) / width, 1.0, 1.0);

    let right_x_translation_mat =
        cgmath::Matrix4::from_translation([-1.0, -(1.0 - 2.0 * y / height), 0.0].into());

    let right_instance = Instance {
        transform: *(cgmath::Matrix4::from_angle_z(cgmath::Deg(180.0))
            * right_x_translation_mat
            * right_width_scale_mat
            * x_translation_mat
            * x_thickness_scale_mat)
            .as_ref(),
    };

    let top_instance = Instance {
        transform: *(cgmath::Matrix4::from_translation(
            (-(1.0 - 2.0 * x / width), 1.0, 0.0).into(),
        ) * cgmath::Matrix4::from_nonuniform_scale(
            1.0,
            (y - crosshair_gap) / height,
            1.0,
        ) * cgmath::Matrix4::from_translation([0.0, -1.0, 0.0].into())
            * y_thickness_scale_mat
            * cgmath::Matrix4::from_angle_z(cgmath::Deg(-90.0)))
        .as_ref(),
    };

    let bottom_instance = Instance {
        transform: *(cgmath::Matrix4::from_translation(
            (-(1.0 - 2.0 * x / width), -1.0, 0.0).into(),
        ) * cgmath::Matrix4::from_nonuniform_scale(
            1.0,
            1.0 - (y + crosshair_gap) / height,
            1.0,
        ) * cgmath::Matrix4::from_translation([0.0, 1.0, 0.0].into())
            * y_thickness_scale_mat
            * cgmath::Matrix4::from_angle_z(cgmath::Deg(90.0)))
        .as_ref(),
    };

    let instances = [left_instance, right_instance, top_instance, bottom_instance];

    queue.write_buffer(instance_buffer, 0, bytemuck::cast_slice(&instances));
}
*/
