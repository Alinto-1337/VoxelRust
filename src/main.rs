
use glfw::{fail_on_errors, Action, Key, Window, WindowHint, ClientApiHint};
use std::env;
mod render_backend;
use render_backend::test_render_pipeline_builder::TestRenderPipelineBuilder;

struct State<'a> {
    instance: wgpu::Instance,
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: (i32, i32),
    window: &'a mut Window,
    render_pipeline: wgpu::RenderPipeline,
}

impl<'a> State<'a> 
{
    async fn new(window: &'a mut Window) -> Self 
    {
        let size = window.get_framebuffer_size();


        // ---- 🫃 Create WGPU instance 🫃 ----
        let instance_descriptor = wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(), ..Default::default()
        };

        let instance  = wgpu::Instance::default();
        

        // ---- 💻 Create a surface 💻 ----
        // Surface: the area/pixels on the screen in the window
        let surface = instance.create_surface(window.render_context()).unwrap();


        // ---- 📋 Request Adaptor 📋 ----
        // Adaptor: Like a GPU Sellector,  used to search for GPUS
        let adapter_descriptor = wgpu::RequestAdapterOptionsBase {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        };

        let adapter = instance.request_adapter(&adapter_descriptor)
            .await.unwrap()
        ;


        // ---- Request Device + Queue ----
        // Device 🖌️:  interface for GPU.
        // Queue 📨:   Used to send instructions to the GPU
        let device_descriptor = wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            memory_hints: wgpu::MemoryHints::Performance,
            trace: wgpu::Trace::default(),
            label: Some("Device"),
        };

        let (device, queue) = adapter
            .request_device(&device_descriptor)
            .await.unwrap()
        ;

        // Configure the Surface 💻
        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .filter(|f | f.is_srgb())
            .next()
            .unwrap_or(surface_capabilities.formats[0])
        ;

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.0 as u32,
            height: size.1 as u32,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2
        };
        surface.configure(&device, &config);

        // Build Render pipeline
        let mut pipeline_builder = TestRenderPipelineBuilder::new();
        pipeline_builder.set_shader_module("shaders/test_shader1.wgsl", "vs_main", "fs_main");
        pipeline_builder.set_pixel_format(config.format);
        let render_pipeline: wgpu::RenderPipeline = pipeline_builder.build_pipeline(&device);

        // 📥 Return 📥
        Self {
            instance,
            window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline
        }
        
    }

    fn resize(&mut self, new_size: (i32, i32)) 
    { 
        if new_size.0 > 0 && new_size.1 > 0 {
            self.size = new_size;
            self.config.width = new_size.0 as u32;
            self.config.height = new_size.1 as u32;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn update_surface(&mut self) {
        self.surface = self.instance.create_surface(self.window.render_context()).unwrap();
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError>
    { 
        let drawable = self.surface.get_current_texture()?;
        let image_view_descriptor = wgpu::TextureViewDescriptor::default();
        let image_view = drawable.texture.create_view(&image_view_descriptor);

        // ---- 🧾 Create command encoder 🧾 ----
        // Command Encoder: Can record several 'RenderPass'es or 'ComputePass'es and compile it into a 'CommandBuffer' to be executed by the GPU
        // ├─── RenderPass 🧾🖌️: A struct of Drawing Commands and/or render state set() commands
        // └─── ComputePass 🧾🤓: A struct of computation commands
        let command_encoder_descriptor = wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder")
        };
        let mut command_encoder = self.device.create_command_encoder(&command_encoder_descriptor);


        // ---- Create Color Attachment 🎨
        // 
        let color_attachment = wgpu::RenderPassColorAttachment {
            view: &image_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.5,
                    g: 0.5,
                    b: 0.75,
                    a: 1.0
                }),
                store: wgpu::StoreOp::Store,
            },
        };

        // descriptor data for a renderpass
        let render_pass_descriptor = wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(color_attachment)],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None
        };

        // transforms the descriptor data into a RenderPass to the RenderEncoder 
        {
            let mut renderpass = command_encoder.begin_render_pass(&render_pass_descriptor);
            renderpass.set_pipeline(&self.render_pipeline);
            renderpass.draw(0..3, 0..1);
        }

        // Finish the commandEncoder and push the returned CommandBuffer to the Queue (to the GPU)
        self.queue.submit(std::iter::once(command_encoder.finish()));

        // Makes the drawable be presented on the owning surface
        drawable.present();

        Ok(())

    }

}


async fn run() {

    let mut glfw = glfw::init(fail_on_errors!())
        .unwrap()
    ;

    glfw.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));
    
    let (mut window, events) = 
        glfw.create_window(
            800, 600, "Awesome Voxel Game", 
            glfw::WindowMode::Windowed).unwrap()
    ;
    
    let mut state = State::new(&mut window).await;

    // make the window "poll" (send a message, like event) certain events 
    state.window.set_framebuffer_size_polling(true);
    state.window.set_size_polling(true);
    state.window.set_key_polling(true);
    state.window.set_mouse_button_polling(true);
    state.window.set_pos_polling(true);

    

    // tick loop
    while !state.window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {

                //Hit escape
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    state.window.set_should_close(true)
                }

                //Window was moved
                glfw::WindowEvent::Pos(..) => {
                    state.update_surface();
                    state.resize(state.size);
                }

                //Window was resized
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    state.update_surface();
                    state.resize((width, height));
                }
                _ => {}
            }
        }

        match state.render() {
            Ok(_) => {},
            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                state.update_surface();
                state.resize(state.size);
            },
            Err(e) => eprintln!("{:?}", e),
        }
        
    }
}


fn main() 
{
    unsafe { env::set_var("GLFW_PLATFORM", "wayland"); }

    let key = "GLFW_PLATFORM";
    match env::var(key) {
        Ok(val) => println!("{key}: {val:?}"),
        Err(e) => println!("couldn't interpret {key}: {e}"),
    }

    pollster::block_on(run())
}

