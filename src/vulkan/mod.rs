#[allow(dead_code)]
mod ffi;

pub use ffi::{
    PFN_vkCreateDebugUtilsMessengerEXT, PFN_vkDebugUtilsMessengerCallbackEXT,
    PFN_vkDestroyDebugUtilsMessengerEXT, VkAllocationCallbacks, VkApplicationInfo, VkBool32,
    VkDebugUtilsMessageSeverityFlagBitsEXT, VkDebugUtilsMessageSeverityFlagsEXT,
    VkDebugUtilsMessageTypeFlagBitsEXT, VkDebugUtilsMessageTypeFlagsEXT,
    VkDebugUtilsMessengerCallbackDataEXT, VkDebugUtilsMessengerCreateInfoEXT,
    VkDebugUtilsMessengerEXT, VkExtensionProperties, VkInstance, VkInstanceCreateFlags,
    VkInstanceCreateInfo, VkLayerProperties, VkResult, VkStructureType, VK_API_VERSION_1_0,
    VK_EXT_DEBUG_UTILS_EXTENSION_NAME, VK_FALSE, VK_MAKE_API_VERSION, VkSurfaceKHR
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
) -> std::ffi::c_void {
    unsafe { ffi::vkDestroyInstance(instance, p_allocator) }
}

// pub fn populate_debug_message_create_info(
//     create_info: &mut VkDebugUtilsMessengerCreateInfoEXT,
// ) -> std::ffi::c_void {
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
//     p_user_data: *mut std::ffi::c_void,
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
) {
    unsafe { ffi::vkDestroySurfaceKHR(instance, surface, p_allocator) }
}