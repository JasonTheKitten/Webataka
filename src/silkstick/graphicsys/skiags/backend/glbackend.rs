use super::SkiaBackend;

use skia_safe::gpu;

pub struct GLBackend {

}

impl GLBackend {
    pub fn new() -> Self {
        GLBackend {}
    }
}

impl SkiaBackend for GLBackend {
    fn create_surface(&self, width: u32, height: u32) -> Box<skia_safe::Surface> {
        // If this call fails, make sure the x11 or wayland feature is enabled in the skia-safe crate
        let interface = gpu::gl::Interface::new_native()
            .expect("I definitely did not run into major issues this early into the program...");
        let mut gl_context = gpu::DirectContext::new_gl(interface, None)
            .expect("Unable to create GL context");
        let framebuffer_info = gpu::gl::FramebufferInfo {
            fboid: 0,
            format: gpu::gl::Format::RGBA8.into(),
            protected: skia_safe::gpu::Protected::No,
        };
        let render_target = skia_safe::gpu::backend_render_targets::make_gl(
            (width.try_into().unwrap(), height.try_into().unwrap()),
            0, 0,
            framebuffer_info);
        let color_space = skia_safe::ColorSpace::new_srgb();

        Box::new(skia_safe::gpu::surfaces::wrap_backend_render_target(
            &mut gl_context,
            &render_target,
            skia_safe::gpu::SurfaceOrigin::BottomLeft,
            skia_safe::ColorType::RGBA8888,
            color_space,
            None,
        ).expect("Failed to create surface"))
    }
}