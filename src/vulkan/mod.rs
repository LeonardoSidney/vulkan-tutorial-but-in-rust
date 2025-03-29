#[allow(dead_code)]
mod ffi;

use std::ffi::c_void;

pub use ffi::{
    PFN_vkCreateDebugUtilsMessengerEXT, PFN_vkDebugUtilsMessengerCallbackEXT,
    PFN_vkDestroyDebugUtilsMessengerEXT, VkAccessFlagBits, VkAllocationCallbacks,
    VkApplicationInfo, VkAttachmentDescription, VkAttachmentLoadOp, VkAttachmentReference,
    VkAttachmentStoreOp, VkBlendFactor, VkBlendOp, VkBool32, VkColorComponentFlagBits,
    VkColorSpaceKHR, VkComponentMapping, VkComponentSwizzle, VkCompositeAlphaFlagBitsKHR,
    VkCullModeFlagBits, VkDebugUtilsMessageSeverityFlagBitsEXT,
    VkDebugUtilsMessageSeverityFlagsEXT, VkDebugUtilsMessageTypeFlagBitsEXT,
    VkDebugUtilsMessageTypeFlagsEXT, VkDebugUtilsMessengerCallbackDataEXT,
    VkDebugUtilsMessengerCreateInfoEXT, VkDebugUtilsMessengerEXT, VkDevice, VkDeviceCreateInfo,
    VkDeviceQueueCreateInfo, VkDynamicState, VkExtensionProperties, VkExtent2D, VkFormat,
    VkFrontFace, VkGraphicsPipelineCreateInfo, VkImage, VkImageAspectFlagBits, VkImageLayout,
    VkImageSubresourceRange, VkImageUsageFlagBits, VkImageView, VkImageViewCreateInfo,
    VkImageViewType, VkInstance, VkInstanceCreateFlags, VkInstanceCreateInfo, VkLayerProperties,
    VkLogicOp, VkOffset2D, VkPhysicalDevice, VkPhysicalDeviceFeatures, VkPhysicalDeviceProperties,
    VkPipeline, VkPipelineBindPoint, VkPipelineCache, VkPipelineColorBlendAttachmentState,
    VkPipelineColorBlendStateCreateInfo, VkPipelineDynamicStateCreateInfo,
    VkPipelineInputAssemblyStateCreateInfo, VkPipelineLayout, VkPipelineLayoutCreateInfo,
    VkPipelineMultisampleStateCreateInfo, VkPipelineRasterizationStateCreateInfo,
    VkPipelineShaderStageCreateInfo, VkPipelineStageFlagBits, VkPipelineVertexInputStateCreateInfo,
    VkPipelineViewportStateCreateInfo, VkPolygonMode, VkPresentModeKHR, VkPrimitiveTopology,
    VkQueue, VkQueueFamilyProperties, VkQueueFlagBits, VkRect2D, VkRenderPass,
    VkRenderPassCreateInfo, VkResult, VkSampleCountFlagBits, VkShaderModule,
    VkShaderModuleCreateInfo, VkShaderStageFlagBits, VkSharingMode, VkStructureType,
    VkSubpassDependency, VkSubpassDescription, VkSurfaceCapabilitiesKHR, VkSurfaceFormatKHR,
    VkSurfaceKHR, VkSwapchainCreateInfoKHR, VkSwapchainKHR, VkViewport, VK_API_VERSION_1_0,
    VK_EXT_DEBUG_UTILS_EXTENSION_NAME, VK_FALSE, VK_KHR_SWAPCHAIN_EXTENSION_NAME,
    VK_MAKE_API_VERSION, VK_SUBPASS_EXTERNAL, VK_TRUE,
};

pub fn vk_enumerate_instance_layer_properties(
    p_property_count: *mut u32,
    p_properties: *mut VkLayerProperties,
) -> VkResult {
    unsafe { ffi::vkEnumerateInstanceLayerProperties(p_property_count, p_properties) }
}

pub fn vk_enumerate_instance_extension_properties(
    p_layer_name: *const std::os::raw::c_char,
    p_property_count: *mut u32,
    p_properties: *mut VkExtensionProperties,
) -> VkResult {
    unsafe {
        ffi::vkEnumerateInstanceExtensionProperties(p_layer_name, p_property_count, p_properties)
    }
}

pub fn vk_create_instance(
    p_create_info: *const VkInstanceCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_instance: *mut VkInstance,
) -> VkResult {
    unsafe { ffi::vkCreateInstance(p_create_info, p_allocator, p_instance) }
}

pub fn vk_destroy_instance(
    instance: VkInstance,
    p_allocator: *const VkAllocationCallbacks,
) -> c_void {
    unsafe { ffi::vkDestroyInstance(instance, p_allocator) }
}

// pub fn populate_debug_message_create_info(
//     create_info: &mut VkDebugUtilsMessengerCreateInfoEXT,
// ) -> c_void {
//     unsafe { ffi::populateDebugMessengerCreateInfo(create_info) }
// }

pub fn vk_get_instance_proc_addr(
    instance: VkInstance,
    p_name: *const std::os::raw::c_char,
) -> ffi::PFN_vkVoidFunction {
    unsafe { ffi::vkGetInstanceProcAddr(instance, p_name) }
}

// pub fn debug_callback(
//     message_severity: VkDebugUtilsMessageSeverityFlagBitsEXT,
//     message_type: ffi::VkDebugUtilsMessageTypeFlagsEXT,
//     p_callback_data: *const ffi::VkDebugUtilsMessengerCallbackDataEXT,
//     p_user_data: *mut c_void,
// ) -> ffi::VkBool32 {
//     unsafe { ffi::debugCallback(message_severity, message_type, p_callback_data, p_user_data) }
// }

pub const fn vk_bit_message_severity(
    verbose: VkDebugUtilsMessageSeverityFlagBitsEXT,
    warning: VkDebugUtilsMessageSeverityFlagBitsEXT,
    error: VkDebugUtilsMessageSeverityFlagBitsEXT,
) -> VkDebugUtilsMessageSeverityFlagsEXT {
    let verbose = verbose as u32;
    let warning = warning as u32;
    let error = error as u32;
    verbose | warning | error
}

pub const fn vk_bit_message_type(
    general: VkDebugUtilsMessageTypeFlagBitsEXT,
    validation: VkDebugUtilsMessageTypeFlagBitsEXT,
    performance: VkDebugUtilsMessageTypeFlagBitsEXT,
) -> VkDebugUtilsMessageTypeFlagsEXT {
    let general = general as u32;
    let validation = validation as u32;
    let performance = performance as u32;
    general | validation | performance
}

pub fn vk_destroy_surface_khr(
    instance: VkInstance,
    surface: VkSurfaceKHR,
    p_allocator: *const VkAllocationCallbacks,
) -> c_void {
    unsafe { ffi::vkDestroySurfaceKHR(instance, surface, p_allocator) }
}

pub fn vk_enumerate_physical_devices(
    instance: VkInstance,
    p_physical_device_count: *mut u32,
    p_physical_devices: *mut VkPhysicalDevice,
) -> VkResult {
    unsafe {
        ffi::vkEnumeratePhysicalDevices(instance, p_physical_device_count, p_physical_devices)
    }
}

pub fn vk_get_physical_device_properties(
    physical_device: VkPhysicalDevice,
    p_properties: *mut VkPhysicalDeviceProperties,
) -> c_void {
    unsafe { ffi::vkGetPhysicalDeviceProperties(physical_device, p_properties) }
}

pub fn vk_get_physical_device_features(
    physical_device: VkPhysicalDevice,
    p_features: *mut VkPhysicalDeviceFeatures,
) -> c_void {
    unsafe { ffi::vkGetPhysicalDeviceFeatures(physical_device, p_features) }
}

pub fn vk_get_physical_device_queue_family_properties(
    physical_device: VkPhysicalDevice,
    p_queue_family_property_count: *mut u32,
    p_queue_family_properties: *mut VkQueueFamilyProperties,
) -> c_void {
    unsafe {
        ffi::vkGetPhysicalDeviceQueueFamilyProperties(
            physical_device,
            p_queue_family_property_count,
            p_queue_family_properties,
        )
    }
}

pub fn vk_get_physical_device_surface_support_khr(
    physical_device: VkPhysicalDevice,
    queue_family_index: u32,
    surface: VkSurfaceKHR,
    p_supported: *mut VkBool32,
) -> VkResult {
    unsafe {
        ffi::vkGetPhysicalDeviceSurfaceSupportKHR(
            physical_device,
            queue_family_index,
            surface,
            p_supported,
        )
    }
}

pub fn vk_enumerate_device_extension_properties(
    physical_device: VkPhysicalDevice,
    p_layer_name: *const std::os::raw::c_char,
    p_property_count: *mut u32,
    p_properties: *mut VkExtensionProperties,
) -> VkResult {
    unsafe {
        ffi::vkEnumerateDeviceExtensionProperties(
            physical_device,
            p_layer_name,
            p_property_count,
            p_properties,
        )
    }
}

pub fn vk_get_physical_device_surface_capabilities_khr(
    physical_device: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    p_surface_capabilities: *mut VkSurfaceCapabilitiesKHR,
) -> VkResult {
    unsafe {
        ffi::vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
            physical_device,
            surface,
            p_surface_capabilities,
        )
    }
}

pub fn vk_get_physical_device_surface_formats_khr(
    physical_device: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    p_surface_format_count: *mut u32,
    p_surface_formats: *mut VkSurfaceFormatKHR,
) -> VkResult {
    unsafe {
        ffi::vkGetPhysicalDeviceSurfaceFormatsKHR(
            physical_device,
            surface,
            p_surface_format_count,
            p_surface_formats,
        )
    }
}

pub fn vk_get_physical_device_surface_present_modes_khr(
    physical_device: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    p_present_mode_count: *mut u32,
    p_present_modes: *mut VkPresentModeKHR,
) -> VkResult {
    unsafe {
        ffi::vkGetPhysicalDeviceSurfacePresentModesKHR(
            physical_device,
            surface,
            p_present_mode_count,
            p_present_modes,
        )
    }
}

pub fn vk_create_device(
    physical_device: VkPhysicalDevice,
    p_create_info: *const VkDeviceCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_device: *const VkDevice,
) -> VkResult {
    unsafe { ffi::vkCreateDevice(physical_device, p_create_info, p_allocator, p_device) }
}

pub fn vk_get_device_queue(
    device: VkDevice,
    queue_family_index: u32,
    queue_index: u32,
    p_queue: *mut VkQueue,
) -> c_void {
    unsafe { ffi::vkGetDeviceQueue(device, queue_family_index, queue_index, p_queue) }
}

pub fn vk_destroy_device(device: VkDevice, p_allocator: *const VkAllocationCallbacks) -> c_void {
    unsafe { ffi::vkDestroyDevice(device, p_allocator) }
}

pub fn vk_create_swapchain_khr(
    device: VkDevice,
    p_create_info: *const VkSwapchainCreateInfoKHR,
    p_allocator: *const VkAllocationCallbacks,
    p_swapchain: *mut VkSwapchainKHR,
) -> VkResult {
    unsafe { ffi::vkCreateSwapchainKHR(device, p_create_info, p_allocator, p_swapchain) }
}

pub fn vk_get_swapchain_images_khr(
    device: VkDevice,
    p_swapchain: VkSwapchainKHR,
    p_swapchain_image_count: *mut u32,
    p_swapchain_images: *mut VkImage,
) -> VkResult {
    unsafe {
        ffi::vkGetSwapchainImagesKHR(
            device,
            p_swapchain,
            p_swapchain_image_count,
            p_swapchain_images,
        )
    }
}

pub fn vk_destroy_swapchain_khr(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    p_allocator: *const VkAllocationCallbacks,
) -> c_void {
    unsafe { ffi::vkDestroySwapchainKHR(device, swapchain, p_allocator) }
}

pub fn vk_create_image_view(
    device: VkDevice,
    p_create_info: *const VkImageViewCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_view: *mut VkImageView,
) -> VkResult {
    unsafe { ffi::vkCreateImageView(device, p_create_info, p_allocator, p_view) }
}

pub fn vk_destroy_image_view(
    device: VkDevice,
    image_view: VkImageView,
    p_allocator: *const VkAllocationCallbacks,
) -> c_void {
    unsafe { ffi::vkDestroyImageView(device, image_view, p_allocator) }
}

pub fn vk_create_render_pass(
    device: VkDevice,
    p_create_info: *const VkRenderPassCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_render_pass: *mut VkRenderPass,
) -> VkResult {
    unsafe { ffi::vkCreateRenderPass(device, p_create_info, p_allocator, p_render_pass) }
}

pub fn vk_destroy_render_pass(
    device: VkDevice,
    render_pass: VkRenderPass,
    p_allocator: *const VkAllocationCallbacks,
) -> c_void {
    unsafe { ffi::vkDestroyRenderPass(device, render_pass, p_allocator) }
}

pub fn vk_create_shader_module(
    device: VkDevice,
    p_create_info: *const VkShaderModuleCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_shader_module: *mut VkShaderModule,
) -> VkResult {
    unsafe { ffi::vkCreateShaderModule(device, p_create_info, p_allocator, p_shader_module) }
}

pub fn vk_create_pipeline_layout(
    device: VkDevice,
    p_create_info: *const VkPipelineLayoutCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_pipeline_layout: *mut VkPipelineLayout,
) -> VkResult {
    unsafe { ffi::vkCreatePipelineLayout(device, p_create_info, p_allocator, p_pipeline_layout) }
}

pub fn vk_create_graphics_pipelines(
    device: VkDevice,
    pipeline_cache: VkPipelineCache,
    create_info_count: u32,
    p_create_infos: *const VkGraphicsPipelineCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_pipelines: *mut VkPipeline,
) -> VkResult {
    unsafe {
        ffi::vkCreateGraphicsPipelines(
            device,
            pipeline_cache,
            create_info_count,
            p_create_infos,
            p_allocator,
            p_pipelines,
        )
    }
}

pub fn vk_destroy_shader_module(
    device: VkDevice,
    shader_module: VkShaderModule,
    p_allocator: *const VkAllocationCallbacks,
) -> c_void {
    unsafe { ffi::vkDestroyShaderModule(device, shader_module, p_allocator) }
}

pub fn vk_destroy_pipeline_layout(
    device: VkDevice,
    pipeline_layout: VkPipelineLayout,
    p_allocator: *const VkAllocationCallbacks,
) -> c_void {
    unsafe { ffi::vkDestroyPipelineLayout(device, pipeline_layout, p_allocator) }
}

pub fn vk_destroy_pipeline(
    device: VkDevice,
    pipeline: VkPipeline,
    p_allocator: *const VkAllocationCallbacks,
) -> c_void {
    unsafe { ffi::vkDestroyPipeline(device, pipeline, p_allocator) }
}