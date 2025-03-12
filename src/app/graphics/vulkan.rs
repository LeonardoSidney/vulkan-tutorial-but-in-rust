use std::cell::OnceCell;

use crate::{
    glfw::{self, GLFWwindow},
    utils,
};

use super::api::{GraphicApi, Window};

pub struct Vulkan {
    width: usize,
    height: usize,
    window: OnceCell<*mut GLFWwindow>,
}

impl Vulkan {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            window: OnceCell::new(),
        }
    }

    fn _enable_validation_layers(&self) -> bool {
        utils::debug_mode()
    }
}

impl GraphicApi for Vulkan {
    fn init_window(&self) -> Window {
        println!(
            "Vulkan window initialized with width: {} and height: {}",
            self.width, self.height
        );
        if self._enable_validation_layers() {
            println!("Validation layers enabled");
        }

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

        Window::Vulkan(window)
    }

    fn init_api(&self) {
        println!("Vulkan API initialized");
    }

    fn cleanup(&self) {
        println!("Vulkan cleanup");
    }
}
