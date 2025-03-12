use crate::gateways::vulkan_gateway::VulkanGateway;

pub struct VulkanController {
    _gateway: VulkanGateway,
}

impl VulkanController {
    pub fn new() -> VulkanController {
        VulkanController {
            _gateway: VulkanGateway::new(),
        }
    }

    pub fn execute(&self, width: usize, height: usize) {
        println!("VulkanController executed");
        self._gateway.create_hello_triangle(width, height);
    }
}