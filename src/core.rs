use vulkano::device::DeviceExtensions;
use vulkano::pipeline::graphics::viewport::Viewport;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::{Event, WindowEvent};

use crate::VglRenderer;

pub mod parameters;
use parameters::VglRendererParameters;

pub mod validation_layers;
use validation_layers::VglValidationLayers;

pub mod instance;
use instance::VglInstance;

pub mod surface;
use surface::VglSurface;

pub mod physical_device;
use physical_device::VglPhysicalDevice;

pub mod logical_device;
use logical_device::VglLogicalDevice;

pub mod swapchain;
use swapchain::VglSwapchain;

pub mod render_pass;
use render_pass::VglRenderPass;

pub mod pipeline;
use pipeline::VglPipeline;

pub mod framebuffers;
use framebuffers::VglFramebuffers;


pub mod swapchain_image;
use swapchain_image::VglSwapchainImage;

pub mod command_buffer;
use command_buffer::VglCommandBuffer;

pub mod future;
use future::VglFuture;

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: "
    #version 450
    layout(location = 0) in vec2 position;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }
      "
    }
}

mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        src: "
    #version 450
    layout(location = 0) out vec4 outColor;

    void main() {
        outColor = vec4(0.6, 0.6, 0.6, 1.0);
    }
            "
    }
}

impl VglRenderer {
    pub fn new(
        parameters: VglRendererParameters,
    ) -> Self {
        let mut validation_layers = VglValidationLayers::new();

        let instance = VglInstance::new(&validation_layers);

        validation_layers.setup_debug_callback(&instance);

        let event_loop = EventLoop::new();

        let surface = VglSurface::new(
            &instance,
            &event_loop,
        );

        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::none()
        };

        let physical_device = VglPhysicalDevice::new(
            &instance,
            &surface,
            &device_extensions,
        );

        let logical_device = VglLogicalDevice::new(
            &device_extensions,
            &physical_device,
        );

        let swapchain = VglSwapchain::new(
            &surface,
            &physical_device,
            &logical_device,
        );

        let vs = vs::load(logical_device.clone_logical_device()).unwrap();
        let fs = fs::load(logical_device.clone_logical_device()).unwrap();

        let render_pass = VglRenderPass::new(
            &logical_device,
            &swapchain,
        );

        let pipeline = VglPipeline::new(
            &logical_device,
            &render_pass,
            &vs,
            &fs,
        );

        let mut viewport = Viewport {
            origin: [0.0, 0.0],
            dimensions: [0.0, 0.0],
            depth_range: 0.0..1.0,
        };

        let framebuffers = VglFramebuffers::new(
            &swapchain,
            &render_pass,
            &mut viewport,
        );

        let future = VglFuture::new(
            &logical_device,
        );

        Self {
            parameters,

            event_loop: Some(event_loop),
            surface,

            logical_device,

            swapchain,

            objects: Vec::new(),

            render_pass,

            pipelines: vec![pipeline],

            viewport,
            framebuffers,

            future,

            recreate_swapchain: false,
        }
    }

    pub fn draw(
        &mut self,
    ) {
        self.future.cleanup();

        if self.recreate_swapchain {
            if self.swapchain.recreate_swapchain(&self.surface) {
                return;
            }

            self.framebuffers.recreate_framebuffers(
                &self.swapchain,
                &self.render_pass,
                &mut self.viewport,
            );

            self.recreate_swapchain = false;
        }


        let swapchain_image = VglSwapchainImage::new(&self.swapchain);
        if swapchain_image.suboptimal() { return; }

        let command_buffer = VglCommandBuffer::new(
            &self.logical_device,
            &self.pipelines,
            &self.viewport,
            &self.framebuffers,
            &swapchain_image,
            &mut self.objects,
        );

        self.future.update_future(
            &self.logical_device,
            &self.swapchain,
            swapchain_image,
            command_buffer,
        );
    }


    pub fn run(mut self) {
        self.event_loop.take().unwrap().run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }
                Event::WindowEvent {
                    event: WindowEvent::Resized(_),
                    ..
                } => {
                    self.recreate_swapchain = true;
                }
                Event::RedrawEventsCleared => {
                    self.draw();
                }
                _ => (),
            }
        });
    }
}