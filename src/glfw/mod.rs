mod ffi;

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
    monitor: *mut std::os::raw::c_void,
    share: *mut std::os::raw::c_void,
) -> *mut ffi::GLFWwindow {
    let title = std::ffi::CString::new(title).unwrap();
    unsafe { ffi::glfwCreateWindow(width, height, title.as_ptr(), monitor, share) }
}

pub const GLFW_CLIENT_API: isize = ffi::GLFW_CLIENT_API as isize;
pub const GLFW_NO_API: isize = ffi::GLFW_NO_API as isize;
pub const GLFW_RESIZABLE: isize = ffi::GLFW_RESIZABLE as isize;
pub const GLFW_FALSE: isize = ffi::GLFW_FALSE as isize;

pub type GLFWwindow = ffi::GLFWwindow;