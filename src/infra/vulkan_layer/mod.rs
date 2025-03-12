#[link(name = "HelloTriangleApplication")]
extern "C" {
    fn hello_triange(width: usize, height: usize);
}

pub struct VulkanLayer {}

impl VulkanLayer {
    pub fn new() -> VulkanLayer {
        VulkanLayer {}
    }

    pub fn create_hello_triangle(&self, width: usize, height: usize) {
        unsafe { hello_triange(width, height) };
    }
}
