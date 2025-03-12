use crate::controllers::vulkan_controller::VulkanController;

pub struct App {
    width: usize,
    height: usize,
    vulkan_controller: VulkanController,
}

impl App {
    pub fn new() -> App {
        let width: usize = 800;
        let height: usize = 600;
        let vulkan_controller: VulkanController = VulkanController::new();

        App { width, height, vulkan_controller }
    }

    pub fn execute(&self) {
        self.vulkan_controller.execute(self.width, self.height);
    }
}
