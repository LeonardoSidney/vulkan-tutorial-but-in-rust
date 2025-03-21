use std::ffi::CString;

use crate::vulkan::{VkAllocationCallbacks, VkInstance, VkResult, VkSurfaceKHR};

#[allow(dead_code)]
mod ffi;

pub const GLFW_CLIENT_API: isize = ffi::GLFW_CLIENT_API as isize;
pub const GLFW_NO_API: isize = ffi::GLFW_NO_API as isize;
pub const GLFW_RESIZABLE: isize = ffi::GLFW_RESIZABLE as isize;
pub const GLFW_FALSE: isize = ffi::GLFW_FALSE as isize;

pub type GLFWwindow = ffi::GLFWwindow;
pub type GLFWmonitor = ffi::GLFWmonitor;

pub fn glfw_init() {
    unsafe { ffi::glfwInit() }
}

pub fn glfw_window_hint(hint: isize, value: isize) {
    unsafe {
        ffi::glfwWindowHint(hint, value);
    }
}

pub fn glfw_create_window(
    width: i32,
    height: i32,
    title: &str,
    monitor: *mut GLFWmonitor,
    share: *mut GLFWwindow,
) -> *mut ffi::GLFWwindow {
    let c_title: CString = CString::new(title).expect("CString::new failed");
    unsafe {
        let c_title_ptr = c_title.as_ptr();
        ffi::glfwCreateWindow(width, height, c_title_ptr, monitor, share)
    }
}

pub fn glfw_get_required_instance_extensions(
    count: *mut u32,
) -> *const *const std::os::raw::c_char {
    unsafe { ffi::glfwGetRequiredInstanceExtensions(count) }
}

pub fn glfw_destroy_window(window: *mut ffi::GLFWwindow) {
    unsafe { ffi::glfwDestroyWindow(window) }
}

pub fn glfw_terminate() {
    unsafe { ffi::glfwTerminate() }
}

pub fn glfw_window_should_close(window: *mut ffi::GLFWwindow) -> i32 {
    unsafe { ffi::glfwWindowShouldClose(window) }
}

pub fn glfw_poll_events() {
    unsafe { ffi::glfwPollEvents() }
}

pub fn glfw_create_window_surface(
    instance: VkInstance,
    window: *mut GLFWwindow,
    allocator: *const VkAllocationCallbacks,
    surface: *mut VkSurfaceKHR,
) -> VkResult {
    unsafe { ffi::glfwCreateWindowSurface(instance, window, allocator, surface) }
}
