#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings_glfw.rs"));

extern "C" {
    pub fn glfwInit();
    pub fn glfwWindowHint(hint: isize, value: isize);
    pub fn glfwCreateWindow(
        width: i32,
        height: i32,
        title: *const std::os::raw::c_char,
        monitor: *mut std::os::raw::c_void,
        share: *mut std::os::raw::c_void,
    ) -> *mut GLFWwindow;
}