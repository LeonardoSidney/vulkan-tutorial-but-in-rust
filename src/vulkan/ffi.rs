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
    );
}

//#define VK_MAKE_API_VERSION(variant, major, minor, patch) ((((uint32_t)(variant)) << 29U) | (((uint32_t)(major)) << 22U) | (((uint32_t)(minor)) << 12U) | ((uint32_t)(patch)))
pub const fn VK_MAKE_API_VERSION(variant: u32, major: u32, minor: u32, patch: u32) -> u32 {
    (variant << 29) | (major << 22) | (minor << 12) | patch
}

//#define VK_API_VERSION_1_0 VK_MAKE_API_VERSION(0, 1, 0, 0)
pub const VK_API_VERSION_1_0: u32 = VK_MAKE_API_VERSION(0, 1, 0, 0);
