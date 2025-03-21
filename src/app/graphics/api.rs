use crate::glfw;

pub trait GraphicApi {
    fn init_window(&self) -> Window;
    fn init_api(&self);
    fn cleanup(&self);
    fn should_close(&self) -> bool;
    fn pool_events(&self);
}

pub enum Window {
    Vulkan(*mut glfw::GLFWwindow),
    OpenGL(*mut glfw::GLFWwindow),
}
