use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop, platform::wayland::WindowAttributesExtWayland, window::Window
};

use wgpu::{
    BackendOptions, Backends, Device, DeviceDescriptor, Instance, InstanceDescriptor,
    InstanceFlags, MemoryBudgetThresholds, Queue, RequestAdapterOptions, Surface, SurfaceConfiguration, TextureUsages,
};

use pollster::FutureExt;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
enum GraphicsContextErrors {
    #[error("Failed to capture adapter")]
    AdapterCaptureFailed,

    #[error("Failed to capture device")]
    DeviceCaptureFailed,

    #[error("Could not create surface")]
    SurfaceCreationFailed,

    #[error("Could not get the window handler")]
    WindowHandlerError,
}

#[derive(Debug)]
struct GraphicsContext {
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
}

impl GraphicsContext {
    fn new(window: &Arc<Window>) -> Result<GraphicsContext, GraphicsContextErrors> {
        let desc = InstanceDescriptor {
            backends: Backends::VULKAN,
            flags: InstanceFlags::DEBUG,
            memory_budget_thresholds: MemoryBudgetThresholds::default(),
            backend_options: BackendOptions::default(),
            display: None,
        };
        let instance = Instance::new(desc);
        let local_window = Arc::clone(&window);
        let surface = instance
            .create_surface(local_window)
            .map_err(|_| GraphicsContextErrors::SurfaceCreationFailed);

        match surface {
            Ok(s) => {
                let adapter_future = instance.request_adapter(&RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::LowPower,
                    compatible_surface: Some(&s),
                    force_fallback_adapter: false,
                });
                let adapter = adapter_future.block_on();
                let device_future = adapter
                    .expect("Failed to get the adapter")
                    .request_device(&DeviceDescriptor::default());
                let device_result = device_future.block_on();
                match device_result {
                    Ok((device, queue)) => {
                        let surface_config = SurfaceConfiguration {
                            usage: TextureUsages::RENDER_ATTACHMENT,
                            format: wgpu::TextureFormat::Bgra8Unorm,
                            width: window.inner_size().width,
                            height: window.inner_size().height,
                            present_mode: wgpu::PresentMode::Fifo,
                            desired_maximum_frame_latency: 2,
                            alpha_mode: wgpu::CompositeAlphaMode::Auto,
                            view_formats: vec![],
                        };
                        s.configure(&device, &surface_config);
                        Ok(GraphicsContext {
                            surface: s,
                            device: device,
                            queue: queue,
                        })
                    }
                    Err(_) => {
                        return Err(GraphicsContextErrors::DeviceCaptureFailed);
                    }
                }
            }
            Err(_) => return Err(GraphicsContextErrors::SurfaceCreationFailed),
        }
    }
}

#[derive(Error, Debug)]
enum AppErrors {
    #[error("Could not create window")]
    WindowCreationError,
}

#[derive(Default, Debug)]
pub struct App {
    window: Option<Arc<Window>>,
    graphics: Option<GraphicsContext>,
}

impl App {
    pub fn new() -> Self {
        App {
            window: None,
            graphics: None,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) -> () {
        // protects rewriting the window if another is already created
        if self.window.is_none() {
            let window_attrs = Window::default_attributes()
                .with_name("Utsuroi", "Utsuroi")
                .with_title("Utsuroi");

            self.window = Some(
                event_loop
                    .create_window(window_attrs)
                    .unwrap()
                    .into()
            );
            match &self.window {
                Some(window) => {
                    let local_window = Arc::clone(&window);
                    self.graphics = GraphicsContext::new(&local_window).ok(); // TODO implement good
                                                                              // error handling
                    println!("Graphics setup!");
                }
                None => (),
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use winit::event_loop::EventLoop;

    use super::*;

}
