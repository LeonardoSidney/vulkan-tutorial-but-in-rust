use std::cell::OnceCell;

use crate::glfw::{self, GLFWwindow};

use super::api::{GraphicApi, Window};

pub struct OpenGLApi {
    width: usize,
    height: usize,
    window: OnceCell<*mut GLFWwindow>,
}

impl OpenGLApi {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            window: OnceCell::new(),
        }
    }
}

impl GraphicApi for OpenGLApi {
    fn init_window(&self) -> Window {
        println!(
            "Vulkan window initialized with width: {} and height: {}",
            self.width, self.height
        );

        glfw::glfw_init();

        glfw::glfw_window_hint(glfw::GLFW_CLIENT_API, glfw::GLFW_NO_API);
        glfw::glfw_window_hint(glfw::GLFW_RESIZABLE, glfw::GLFW_FALSE);

        let window = glfw::glfw_create_window(
            self.width as i32,
            self.height as i32,
            "Oito-caneco",
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );

        self.window
            .set(window)
            .expect("Glfw window can not be initialized");

        Window::OpenGL(window)
    }

    fn init_api(&self) {
        println!("OpenGL API initialized");
    }

    fn cleanup(&self) {
        println!("OpenGL cleanup");
    }

    fn should_close(&self) -> bool {
        true
    }

    fn pool_events(&self) {
        println!("OpenGL pool events");
    }
}
