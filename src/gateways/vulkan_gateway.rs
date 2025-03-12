use crate::infra::vulkan_layer::VulkanLayer;

pub struct VulkanGateway {
    vulkan_layer: VulkanLayer,
}

impl VulkanGateway {
    pub fn new() -> VulkanGateway {
        VulkanGateway {
            vulkan_layer: VulkanLayer::new(),
        }
    }

    pub fn create_hello_triangle(&self, width: usize, height: usize) {
        self.vulkan_layer.create_hello_triangle(width, height);
    }
}