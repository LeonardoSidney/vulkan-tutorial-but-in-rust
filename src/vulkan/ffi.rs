#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::c_void;

include!(concat!(env!("OUT_DIR"), "/bindings_vulkan.rs"));

unsafe extern "C" {
    pub unsafe fn vkEnumerateInstanceLayerProperties(
        pPropertyCount: *mut u32,
        pProperties: *mut VkLayerProperties,
    ) -> VkResult;
    pub unsafe fn vkEnumerateInstanceExtensionProperties(
        pLayerName: *const std::os::raw::c_char,
        pPropertyCount: *mut u32,
        pProperties: *mut VkExtensionProperties,
    ) -> VkResult;
    pub unsafe fn vkCreateInstance(
        pCreateInfo: *const VkInstanceCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pInstance: *mut VkInstance,
    ) -> VkResult;
    pub unsafe fn vkDestroyInstance(
        instance: VkInstance,
        pAllocator: *const VkAllocationCallbacks,
    ) -> c_void;
    pub unsafe fn populateDebugMessengerCreateInfo(
        create_info: &mut VkDebugUtilsMessengerCreateInfoEXT,
    ) -> c_void;
    pub unsafe fn debugCallback(
        message_severity: VkDebugUtilsMessageSeverityFlagBitsEXT,
        message_type: VkDebugUtilsMessageTypeFlagsEXT,
        p_callback_data: *const VkDebugUtilsMessengerCallbackDataEXT,
        p_user_data: *mut c_void,
    ) -> VkBool32;
    pub unsafe fn vkGetInstanceProcAddr(
        instance: VkInstance,
        pName: *const std::os::raw::c_char,
    ) -> PFN_vkVoidFunction;
    pub unsafe fn vkDestroySurfaceKHR(
        instance: VkInstance,
        surface: VkSurfaceKHR,
        pAllocator: *const VkAllocationCallbacks,
    ) -> c_void;
    pub unsafe fn vkEnumeratePhysicalDevices(
        instance: VkInstance,
        pPhysicalDeviceCount: *mut u32,
        pPhysicalDevices: *mut VkPhysicalDevice,
    ) -> VkResult;
    pub unsafe fn vkGetPhysicalDeviceProperties(
        physicalDevice: VkPhysicalDevice,
        pProperties: *mut VkPhysicalDeviceProperties,
    ) -> c_void;
    pub unsafe fn vkGetPhysicalDeviceFeatures(
        physicalDevice: VkPhysicalDevice,
        pFeatures: *mut VkPhysicalDeviceFeatures,
    ) -> c_void;
    pub unsafe fn vkGetPhysicalDeviceQueueFamilyProperties(
        physicalDevice: VkPhysicalDevice,
        pQueueFamilyPropertyCount: *mut u32,
        pQueueFamilyProperties: *mut VkQueueFamilyProperties,
    ) -> c_void;
    pub unsafe fn vkGetPhysicalDeviceSurfaceSupportKHR(
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32,
        surface: VkSurfaceKHR,
        pSupported: *mut VkBool32,
    ) -> VkResult;
    pub unsafe fn vkEnumerateDeviceExtensionProperties(
        physicalDevice: VkPhysicalDevice,
        pLayerName: *const std::os::raw::c_char,
        pPropertyCount: *mut u32,
        pProperties: *mut VkExtensionProperties,
    ) -> VkResult;
    pub unsafe fn vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
        physicalDevice: VkPhysicalDevice,
        surface: VkSurfaceKHR,
        pSurfaceCapabilities: *mut VkSurfaceCapabilitiesKHR,
    ) -> VkResult;
    pub unsafe fn vkGetPhysicalDeviceSurfaceFormatsKHR(
        physicalDevice: VkPhysicalDevice,
        surface: VkSurfaceKHR,
        pSurfaceFormatCount: *mut u32,
        pSurfaceFormats: *mut VkSurfaceFormatKHR,
    ) -> VkResult;
    pub unsafe fn vkGetPhysicalDeviceSurfacePresentModesKHR(
        physicalDevice: VkPhysicalDevice,
        surface: VkSurfaceKHR,
        pPresentModeCount: *mut u32,
        pPresentModes: *mut VkPresentModeKHR,
    ) -> VkResult;
    pub unsafe fn vkCreateDevice(
        physicalDevice: VkPhysicalDevice,
        pCreateInfo: *const VkDeviceCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pDevice: *const VkDevice,
    ) -> VkResult;
    pub unsafe fn vkGetDeviceQueue(
        device: VkDevice,
        queueFamilyIndex: u32,
        queueIndex: u32,
        pQueue: *mut VkQueue,
    ) -> c_void;
    pub unsafe fn vkDestroyDevice(
        device: VkDevice,
        pAllocator: *const VkAllocationCallbacks,
    ) -> c_void;
    pub unsafe fn vkCreateSwapchainKHR(
        device: VkDevice,
        pCreateInfo: *const VkSwapchainCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSwapchain: *mut VkSwapchainKHR,
    ) -> VkResult;
    pub unsafe fn vkGetSwapchainImagesKHR(
        device: VkDevice,
        pSwapchain: VkSwapchainKHR,
        pSwapchainImageCount: *mut u32,
        pSwapchainImages: *mut VkImage,
    ) -> VkResult;
    pub unsafe fn vkDestroySwapchainKHR(
        device: VkDevice,
        swapchain: VkSwapchainKHR,
        pAllocator: *const VkAllocationCallbacks,
    ) -> c_void;
    pub unsafe fn vkCreateImageView(
        device: VkDevice,
        pCreateInfo: *const VkImageViewCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pView: *mut VkImageView,
    ) -> VkResult;
    pub unsafe fn vkDestroyImageView(
        device: VkDevice,
        imageView: VkImageView,
        pAllocator: *const VkAllocationCallbacks,
    ) -> c_void;
    pub unsafe fn vkCreateRenderPass(
        device: VkDevice,
        pCreateInfo: *const VkRenderPassCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pRenderPass: *mut VkRenderPass,
    ) -> VkResult;
    pub unsafe fn vkDestroyRenderPass(
        device: VkDevice,
        renderPass: VkRenderPass,
        pAllocator: *const VkAllocationCallbacks,
    ) -> c_void;
    pub unsafe fn vkCreateShaderModule(
        device: VkDevice,
        pCreateInfo: *const VkShaderModuleCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pShaderModule: *mut VkShaderModule,
    ) -> VkResult;
    pub unsafe fn vkCreatePipelineLayout(
        device: VkDevice,
        pCreateInfo: *const VkPipelineLayoutCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pPipelineLayout: *mut VkPipelineLayout,
    ) -> VkResult;
    pub unsafe fn vkCreateGraphicsPipelines(
        device: VkDevice,
        pipelineCache: VkPipelineCache,
        createInfoCount: u32,
        pCreateInfos: *const VkGraphicsPipelineCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pPipelines: *mut VkPipeline,
    ) -> VkResult;
    pub unsafe fn vkDestroyShaderModule(
        device: VkDevice,
        shaderModule: VkShaderModule,
        pAllocator: *const VkAllocationCallbacks,
    ) -> c_void;
    pub unsafe fn vkDestroyPipelineLayout(
        device: VkDevice,
        pipelineLayout: VkPipelineLayout,
        pAllocator: *const VkAllocationCallbacks,
    ) -> c_void;
    pub unsafe fn vkDestroyPipeline(
        device: VkDevice,
        pipeline: VkPipeline,
        pAllocator: *const VkAllocationCallbacks,
    ) -> c_void;
    pub unsafe fn vkCreateFramebuffer(
        device: VkDevice,
        pCreateInfo: *const VkFramebufferCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pFramebuffer: *mut VkFramebuffer,
    ) -> VkResult;
    pub unsafe fn vkDestroyFramebuffer(
        device: VkDevice,
        framebuffer: VkFramebuffer,
        pAllocator: *const VkAllocationCallbacks,
    ) -> c_void;
    pub unsafe fn vkCreateCommandPool(
        device: VkDevice,
        pCreateInfo: *const VkCommandPoolCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pCommandPool: *mut VkCommandPool,
    ) -> VkResult;
    pub unsafe fn vkDestroyCommandPool(
        device: VkDevice,
        commandPool: VkCommandPool,
        pAllocator: *const VkAllocationCallbacks,
    ) -> c_void;
    pub unsafe fn vkAllocateCommandBuffers(
        device: VkDevice,
        pAllocateInfo: *const VkCommandBufferAllocateInfo,
        pCommandBuffers: *mut VkCommandBuffer,
    ) -> VkResult;
    pub unsafe fn vkCreateSemaphore(
        device: VkDevice,
        pCreateInfo: *const VkSemaphoreCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pSemaphore: *mut VkSemaphore,
    ) -> VkResult;
    pub unsafe fn vkDestroySemaphore(
        device: VkDevice,
        semaphore: VkSemaphore,
        pAllocator: *const VkAllocationCallbacks,
    ) -> c_void;
    pub unsafe fn vkCreateFence(
        device: VkDevice,
        pCreateInfo: *const VkFenceCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pFence: *mut VkFence,
    ) -> VkResult;
    pub unsafe fn vkDestroyFence(
        device: VkDevice,
        fence: VkFence,
        pAllocator: *const VkAllocationCallbacks,
    ) -> c_void;
    pub unsafe fn vkDeviceWaitIdle(device: VkDevice) -> VkResult;
    pub unsafe fn vkWaitForFences(
        device: VkDevice,
        fenceCount: u32,
        pFences: *const VkFence,
        waitAll: VkBool32,
        timeout: u64,
    ) -> VkResult;
    pub unsafe fn vkResetFences(
        device: VkDevice,
        fenceCount: u32,
        pFences: *const VkFence,
    ) -> VkResult;
    pub unsafe fn vkAcquireNextImageKHR(
        device: VkDevice,
        swapchain: VkSwapchainKHR,
        timeout: u64,
        semaphore: VkSemaphore,
        fence: VkFence,
        pImageIndex: *mut u32,
    ) -> VkResult;
    pub unsafe fn vkResetCommandBuffer(
        commandBuffer: VkCommandBuffer,
        flags: VkCommandBufferResetFlags,
    ) -> VkResult;
    pub unsafe fn vkBeginCommandBuffer(
        commandBuffer: VkCommandBuffer,
        pBeginInfo: *const VkCommandBufferBeginInfo,
    ) -> VkResult;
    pub unsafe fn vkCmdBeginRenderPass(
        commandBuffer: VkCommandBuffer,
        pRenderPassBegin: *const VkRenderPassBeginInfo,
        contents: VkSubpassContents,
    ) -> c_void;
    pub unsafe fn vkCmdBindPipeline(
        commandBuffer: VkCommandBuffer,
        pipelineBindPoint: VkPipelineBindPoint,
        pipeline: VkPipeline,
    ) -> c_void;
    pub unsafe fn vkCmdSetViewport(
        commandBuffer: VkCommandBuffer,
        firstViewport: u32,
        viewportCount: u32,
        pViewports: *const VkViewport,
    ) -> c_void;
    pub unsafe fn vkCmdSetScissor(
        commandBuffer: VkCommandBuffer,
        firstScissor: u32,
        scissorCount: u32,
        pScissors: *const VkRect2D,
    ) -> c_void;
    pub unsafe fn vkCmdDraw(
        commandBuffer: VkCommandBuffer,
        vertexCount: u32,
        instanceCount: u32,
        firstVertex: u32,
        firstInstance: u32,
    ) -> c_void;
    pub unsafe fn vkCmdEndRenderPass(commandBuffer: VkCommandBuffer) -> c_void;
    pub unsafe fn vkEndCommandBuffer(commandBuffer: VkCommandBuffer) -> VkResult;
    pub unsafe fn vkQueueSubmit(
        queue: VkQueue,
        submitCount: u32,
        pSubmits: *const VkSubmitInfo,
        fence: VkFence,
    ) -> VkResult;
    pub unsafe fn vkQueuePresentKHR(
        queue: VkQueue,
        pPresentInfo: *const VkPresentInfoKHR,
    ) -> VkResult;
}

//#define VK_MAKE_API_VERSION(variant, major, minor, patch) ((((uint32_t)(variant)) << 29U) | (((uint32_t)(major)) << 22U) | (((uint32_t)(minor)) << 12U) | ((uint32_t)(patch)))
pub const fn VK_MAKE_API_VERSION(variant: u32, major: u32, minor: u32, patch: u32) -> u32 {
    (variant << 29) | (major << 22) | (minor << 12) | patch
}

//#define VK_API_VERSION_1_0 VK_MAKE_API_VERSION(0, 1, 0, 0)
pub const VK_API_VERSION_1_0: u32 = VK_MAKE_API_VERSION(0, 1, 0, 0);
