pub mod api;
pub mod opengl;
pub mod vulkan;

use api::{GraphicApi, Window};
use opengl::OpenGLApi;
use vulkan::VulkanApi;

#[derive(PartialEq)]
pub enum GraphicsType {
    Vulkan,
    OpenGL,
}

pub struct Graphics {
    api: Box<dyn GraphicApi>,
}

impl Graphics {
    pub fn new(width: usize, height: usize, api_type: GraphicsType) -> Self {
        let api: Box<dyn GraphicApi> = match api_type {
            GraphicsType::Vulkan => Box::new(VulkanApi::new(width, height)),
            GraphicsType::OpenGL => Box::new(OpenGLApi::new(width, height)),
        };

        Self { api }
    }

    pub fn init_window(&self) {
        let window = self.api.init_window();
        match window {
            Window::Vulkan(window) => {
                if window.is_null() {
                    panic!("Vulkan window is null");
                }
            },
            Window::OpenGL(window) => {
                if window.is_null() {
                    panic!("OpenGL window is null");
                }
            }
        }
    }

    pub fn init_api(&self) {
        self.api.init_api()
    }

    pub fn cleanup(&self) {
        self.api.cleanup()
    }

    pub fn should_close(&self) -> bool {
        self.api.should_close()
    }

    pub fn pool_events(&self) {
        self.api.pool_events()
    }

    pub fn wait_events(&self) {
        self.api.wait_events()
    }

    pub fn draw_frame(&self) {
        self.api.draw_frame()
    }

    pub fn wait_device_idle(&self) {
        self.api.wait_device_idle()
    }
}
