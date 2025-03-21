#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::vulkan::{VkAllocationCallbacks, VkInstance, VkResult, VkSurfaceKHR};

include!(concat!(env!("OUT_DIR"), "/bindings_glfw.rs"));

unsafe extern "C" {
    pub unsafe fn glfwInit();
    pub unsafe fn glfwWindowHint(hint: isize, value: isize);
    pub unsafe fn glfwCreateWindow(
        width: i32,
        height: i32,
        title: *const std::os::raw::c_char,
        monitor: *mut GLFWmonitor,
        share: *mut GLFWwindow,
    ) -> *mut GLFWwindow;
    pub unsafe fn glfwGetRequiredInstanceExtensions(
        count: *mut u32,
    ) -> *const *const std::os::raw::c_char;
    pub unsafe fn glfwDestroyWindow(window: *mut GLFWwindow);
    pub unsafe fn glfwTerminate();
    pub unsafe fn glfwWindowShouldClose(window: *mut GLFWwindow) -> i32;
    pub unsafe fn glfwPollEvents();
    pub unsafe fn glfwCreateWindowSurface(
        instance: VkInstance,
        window: *mut GLFWwindow,
        allocator: *const VkAllocationCallbacks,
        surface: *mut VkSurfaceKHR,
    ) -> VkResult;
}
