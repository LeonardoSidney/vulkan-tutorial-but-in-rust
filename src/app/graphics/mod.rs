pub mod api;
pub mod opengl;
pub mod vulkan;

use api::{GraphicApi, Window};
use opengl::OpenGL;
use vulkan::Vulkan;

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
            GraphicsType::Vulkan => Box::new(Vulkan::new(width, height)),
            GraphicsType::OpenGL => Box::new(OpenGL::new(width, height)),
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
}
