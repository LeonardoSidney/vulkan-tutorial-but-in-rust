use std::cell::OnceCell;

use crate::glfw::{
    glfw_create_window, glfw_init, glfw_window_hint, GLFWwindow, GLFW_CLIENT_API, GLFW_FALSE,
    GLFW_NO_API, GLFW_RESIZABLE,
};

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

        glfw_init();

        glfw_window_hint(GLFW_CLIENT_API as isize, GLFW_NO_API as isize);
        glfw_window_hint(GLFW_RESIZABLE as isize, GLFW_FALSE as isize);

        let window = glfw_create_window(
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
