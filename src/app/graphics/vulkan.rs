use std::cell::OnceCell;
use std::collections::HashSet;
use std::ffi::{c_char, c_float, c_int, c_void, CStr, CString};
use std::mem::MaybeUninit;
use std::vec;

use crate::glfw::{
    glfw_create_window, glfw_create_window_surface, glfw_destroy_window, glfw_get_framebuffer_size,
    glfw_get_required_instance_extensions, glfw_init, glfw_poll_events, glfw_terminate,
    glfw_window_hint, glfw_window_should_close, GLFW_CLIENT_API, GLFW_FALSE, GLFW_NO_API,
    GLFW_RESIZABLE,
};
use crate::utils::debug_mode;
use crate::vulkan::{
    vk_bit_message_severity, vk_bit_message_type, vk_create_device, vk_create_image_view,
    vk_create_instance, vk_create_swapchain_khr, vk_destroy_device, vk_destroy_image_view,
    vk_destroy_instance, vk_destroy_surface_khr, vk_destroy_swapchain_khr,
    vk_enumerate_device_extension_properties, vk_enumerate_instance_extension_properties,
    vk_enumerate_instance_layer_properties, vk_enumerate_physical_devices, vk_get_device_queue,
    vk_get_instance_proc_addr, vk_get_physical_device_features, vk_get_physical_device_properties,
    vk_get_physical_device_queue_family_properties,
    vk_get_physical_device_surface_capabilities_khr, vk_get_physical_device_surface_formats_khr,
    vk_get_physical_device_surface_present_modes_khr, vk_get_physical_device_surface_support_khr,
    vk_get_swapchain_images_khr, PFN_vkCreateDebugUtilsMessengerEXT,
    PFN_vkDebugUtilsMessengerCallbackEXT, PFN_vkDestroyDebugUtilsMessengerEXT,
    VkAllocationCallbacks, VkApplicationInfo, VkBool32, VkColorSpaceKHR, VkComponentMapping,
    VkComponentSwizzle, VkCompositeAlphaFlagBitsKHR, VkDebugUtilsMessageSeverityFlagBitsEXT,
    VkDebugUtilsMessageTypeFlagBitsEXT, VkDebugUtilsMessageTypeFlagsEXT,
    VkDebugUtilsMessengerCallbackDataEXT, VkDebugUtilsMessengerCreateInfoEXT,
    VkDebugUtilsMessengerEXT, VkDevice, VkDeviceCreateInfo, VkDeviceQueueCreateInfo,
    VkExtensionProperties, VkExtent2D, VkFormat, VkImage, VkImageAspectFlagBits,
    VkImageSubresourceRange, VkImageUsageFlagBits, VkImageView, VkImageViewCreateInfo,
    VkImageViewType, VkInstance, VkInstanceCreateFlags, VkInstanceCreateInfo, VkLayerProperties,
    VkPhysicalDevice, VkPhysicalDeviceFeatures, VkPhysicalDeviceProperties, VkPresentModeKHR,
    VkQueue, VkQueueFamilyProperties, VkQueueFlagBits, VkResult, VkSharingMode, VkStructureType,
    VkSurfaceCapabilitiesKHR, VkSurfaceFormatKHR, VkSurfaceKHR, VkSwapchainCreateInfoKHR,
    VkSwapchainKHR, VK_API_VERSION_1_0, VK_EXT_DEBUG_UTILS_EXTENSION_NAME, VK_FALSE,
    VK_KHR_SWAPCHAIN_EXTENSION_NAME, VK_MAKE_API_VERSION, VK_TRUE,
};
use crate::{glfw::GLFWwindow, utils};

use VkColorSpaceKHR::VK_COLOR_SPACE_SRGB_NONLINEAR_KHR;
use VkComponentSwizzle::VK_COMPONENT_SWIZZLE_IDENTITY;
use VkCompositeAlphaFlagBitsKHR::VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR;
use VkDebugUtilsMessageSeverityFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT;
use VkDebugUtilsMessageSeverityFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT;
use VkDebugUtilsMessageSeverityFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT;
use VkDebugUtilsMessageTypeFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT;
use VkDebugUtilsMessageTypeFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT;
use VkDebugUtilsMessageTypeFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT;
use VkFormat::VK_FORMAT_B8G8R8A8_SRGB;
use VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT;
use VkImageUsageFlagBits::VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT;
use VkImageViewType::VK_IMAGE_VIEW_TYPE_2D;
use VkPresentModeKHR::VK_PRESENT_MODE_FIFO_KHR;
use VkPresentModeKHR::VK_PRESENT_MODE_MAILBOX_KHR;
use VkQueueFlagBits::VK_QUEUE_GRAPHICS_BIT;
use VkResult::VK_SUCCESS;
use VkSharingMode::VK_SHARING_MODE_CONCURRENT;
use VkSharingMode::VK_SHARING_MODE_EXCLUSIVE;
use VkStructureType::VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO;
use VkStructureType::VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO;
use VkStructureType::VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;
use VkStructureType::VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR;

pub extern "C" fn debug_callback(
    message_severity: VkDebugUtilsMessageSeverityFlagBitsEXT,
    message_type: VkDebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const VkDebugUtilsMessengerCallbackDataEXT,
    p_user_data: *mut std::ffi::c_void,
) -> VkBool32 {
    print!("Debug callback initiated: ");
    print!("\tMessage severity: {:?}", message_severity);
    print!("\t Message type: {:?}", message_type);
    unsafe {
        let message = CStr::from_ptr((*p_callback_data).pMessage)
            .to_str()
            .expect("Failed to get message");
        print!("\t Message: {:?}", message);
        println!("\t User data: {:?}", p_user_data);
    }

    VK_FALSE
}

use super::api::{GraphicApi, Window};

struct QueueFamilyIndices {
    graphics_family: Option<u32>,
    present_family: Option<u32>,
}

impl QueueFamilyIndices {
    fn new() -> Self {
        Self {
            graphics_family: None,
            present_family: None,
        }
    }

    fn is_complete(&self) -> bool {
        self.graphics_family.is_some() && self.present_family.is_some()
    }
}

struct SwapChainSupportDetails {
    pub capabilities: VkSurfaceCapabilitiesKHR,
    pub formats: Vec<VkSurfaceFormatKHR>,
    pub present_modes: Vec<VkPresentModeKHR>,
}

pub struct VulkanApi {
    width: usize,
    height: usize,
    window: OnceCell<*mut GLFWwindow>,
    validation_layers: Vec<CString>,
    instance: OnceCell<VkInstance>,
    debug_messenger: OnceCell<VkDebugUtilsMessengerEXT>,
    surface: OnceCell<VkSurfaceKHR>,
    physical_device: OnceCell<VkPhysicalDevice>,
    device_extensions: Vec<CString>,
    device: OnceCell<VkDevice>,
    graphics_queue: OnceCell<VkQueue>,
    present_queue: OnceCell<VkQueue>,
    swapchain: OnceCell<VkSwapchainKHR>,
    swapchain_images: OnceCell<Vec<VkImage>>,
    swapchain_image_format: OnceCell<VkFormat>,
    swapchain_extent: OnceCell<VkExtent2D>,
    swapchain_image_views: OnceCell<Vec<VkImageView>>,
}

impl VulkanApi {
    pub fn new(width: usize, height: usize) -> Self {
        let validation_layers: Vec<CString> = vec![CString::new("VK_LAYER_KHRONOS_validation")
            .expect("CString::new VK_LAYER_KHRONOS_validation failed!")];
        let device_extensions: Vec<CString> =
            vec![CStr::from_bytes_with_nul(VK_KHR_SWAPCHAIN_EXTENSION_NAME)
                .expect("CStr::from_bytes_with_nul VK_KHR_SWAPCHAIN_EXTENSION_NAME failed!")
                .to_owned()];
        Self {
            width,
            height,
            window: OnceCell::new(),
            validation_layers,
            instance: OnceCell::new(),
            debug_messenger: OnceCell::new(),
            surface: OnceCell::new(),
            physical_device: OnceCell::new(),
            device_extensions,
            device: OnceCell::new(),
            graphics_queue: OnceCell::new(),
            present_queue: OnceCell::new(),
            swapchain: OnceCell::new(),
            swapchain_images: OnceCell::new(),
            swapchain_image_format: OnceCell::new(),
            swapchain_extent: OnceCell::new(),
            swapchain_image_views: OnceCell::new(),
        }
    }

    fn _enable_validation_layers(&self) -> bool {
        utils::debug_mode()
    }

    fn _create_instance(&self) {
        if debug_mode() {
            println!("Creating Vulkan instance");
        }
        if self._enable_validation_layers() && !self._check_validation_layer_support() {
            panic!("Validation layers requested, but not available!");
        }

        let app_name = CString::new("Hello Triangle").expect("CString::new Hello Triangle failed!");
        let engine_name = CString::new("Oito-Caneco").expect("CString::new Oito-Caneco failed!");

        let app_info: VkApplicationInfo = VkApplicationInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_APPLICATION_INFO,
            pNext: std::ptr::null(),
            pApplicationName: app_name.as_ptr(),
            applicationVersion: VK_MAKE_API_VERSION(0, 1, 0, 0),
            pEngineName: engine_name.as_ptr(),
            engineVersion: VK_MAKE_API_VERSION(0, 1, 0, 0),
            apiVersion: VK_API_VERSION_1_0,
        };

        let extensions = self._get_required_extensions();
        let flags: VkInstanceCreateFlags = 0;
        let mut enabled_layer_count: u32 = 0;
        let mut pp_enabled_layer_names: *const *const i8 = std::ptr::null();
        let mut debug_create_info: VkDebugUtilsMessengerCreateInfoEXT =
            unsafe { std::mem::zeroed() };
        if self._enable_validation_layers() {
            enabled_layer_count = self.validation_layers.len() as u32;
            pp_enabled_layer_names = self.validation_layers.as_ptr() as *const *const i8;

            self._populate_debug_messenger_create_info(&mut debug_create_info);
        }
        let p_debug_create_info: *const VkDebugUtilsMessengerCreateInfoEXT = &debug_create_info;
        let create_info: VkInstanceCreateInfo = VkInstanceCreateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            pApplicationInfo: &app_info,
            enabledExtensionCount: extensions.len() as u32,
            ppEnabledExtensionNames: extensions.as_ptr() as *const *const i8,
            enabledLayerCount: enabled_layer_count,
            ppEnabledLayerNames: pp_enabled_layer_names,
            pNext: p_debug_create_info as *const c_void,
            flags,
        };

        let mut instance: VkInstance = std::ptr::null_mut();
        let result: VkResult = vk_create_instance(&create_info, std::ptr::null(), &mut instance);
        if result != VK_SUCCESS {
            panic!("Failed to create instance");
        }

        self.instance.set(instance).expect("Failed to set instance");
    }

    fn _populate_debug_messenger_create_info(
        &self,
        debug_create_info: &mut VkDebugUtilsMessengerCreateInfoEXT,
    ) {
        let message_severity: VkDebugUtilsMessageTypeFlagsEXT = vk_bit_message_severity(
            VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT,
            VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT,
            VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT,
        );

        let message_type: VkDebugUtilsMessageTypeFlagsEXT = vk_bit_message_type(
            VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT,
            VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT,
            VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT,
        );

        let debug_callback_fn = debug_callback;
        let debug_callback: PFN_vkDebugUtilsMessengerCallbackEXT = Some(debug_callback_fn);

        let create_info: VkDebugUtilsMessengerCreateInfoEXT = VkDebugUtilsMessengerCreateInfoEXT {
            sType: VkStructureType::VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
            pNext: std::ptr::null(),
            flags: 0,
            messageSeverity: message_severity,
            messageType: message_type,
            pfnUserCallback: debug_callback,
            pUserData: std::ptr::null_mut(),
        };

        *debug_create_info = create_info;
    }

    fn _get_required_extensions(&self) -> Vec<*const c_char> {
        let mut glfw_extension_count: u32 = 0;
        let glfw_extensions: *const *const c_char =
            glfw_get_required_instance_extensions(&mut glfw_extension_count);
        let mut extensions: Vec<*const c_char> = unsafe {
            assert!(!glfw_extensions.is_null());
            std::slice::from_raw_parts(glfw_extensions, glfw_extension_count as usize).to_vec()
        };

        if self._enable_validation_layers() {
            extensions.push(VK_EXT_DEBUG_UTILS_EXTENSION_NAME.as_ptr() as *const c_char);
        }

        if debug_mode() {
            println!("GLFW extensions:");
            for &extension_ptr in &extensions {
                let extension_name: &str = unsafe {
                    std::ffi::CStr::from_ptr(extension_ptr)
                        .to_str()
                        .expect("Failed to get extension name")
                };
                println!("\t {}", extension_name);
            }
        }

        let mut extension_count = 0;
        vk_enumerate_instance_extension_properties(
            std::ptr::null(),
            &mut extension_count,
            std::ptr::null_mut(),
        );

        let mut available_extensions: Vec<VkExtensionProperties> =
            Vec::with_capacity(extension_count as usize);

        unsafe {
            available_extensions.set_len(extension_count as usize);
        }

        vk_enumerate_instance_extension_properties(
            std::ptr::null(),
            &mut extension_count,
            available_extensions.as_mut_ptr(),
        );

        if debug_mode() {
            println!("Vulkan available instance extensions:");
            for extension in &available_extensions {
                let extension_name: &str = unsafe {
                    std::ffi::CStr::from_ptr(extension.extensionName.as_ptr())
                        .to_str()
                        .expect("Failed to get extension name")
                };
                println!("\t{}", extension_name);
            }
        }

        extensions
    }

    fn _check_validation_layer_support(&self) -> bool {
        let mut layer_count: u32 = 0;
        vk_enumerate_instance_layer_properties(&mut layer_count as *mut u32, std::ptr::null_mut());
        let mut available_layers: Vec<VkLayerProperties> = Vec::with_capacity(layer_count as usize);

        unsafe {
            available_layers.set_len(layer_count as usize);
        }

        vk_enumerate_instance_layer_properties(
            &mut layer_count as *mut u32,
            available_layers.as_mut_ptr(),
        );

        for layer_name in &self.validation_layers {
            let layer_name_str = layer_name
                .to_str()
                .expect("Failed to convert layer name to str");
            let mut layer_found = false;
            for layer_properties in &available_layers {
                let layer_properties_name = layer_properties.layerName;
                let layer_properties_name_str = unsafe {
                    CStr::from_ptr(layer_properties_name.as_ptr())
                        .to_str()
                        .expect("Failed to convert layer properties name to str")
                };

                if layer_name_str == layer_properties_name_str {
                    layer_found = true;
                    break;
                }
            }

            if !layer_found {
                return false;
            }
        }

        true
    }

    fn _setup_debug_messenger(&self) {
        if !self._enable_validation_layers() {
            return;
        }

        let mut debug_create_info: VkDebugUtilsMessengerCreateInfoEXT =
            unsafe { std::mem::zeroed() };
        self._populate_debug_messenger_create_info(&mut debug_create_info);

        let mut debug_messenger: VkDebugUtilsMessengerEXT = { unsafe { std::mem::zeroed() } };

        let p_debug_create_info: *const VkDebugUtilsMessengerCreateInfoEXT = &debug_create_info;
        let result: VkResult = self._create_debug_utils_messenger_ext(
            &self.instance.get().expect("Instance is null"),
            p_debug_create_info,
            std::ptr::null(),
            &mut debug_messenger,
        );

        self.debug_messenger
            .set(debug_messenger)
            .expect("Failed to set debug messenger");

        if result != VK_SUCCESS {
            panic!("Failed to set up debug messenger");
        }
    }

    fn _create_debug_utils_messenger_ext(
        &self,
        instance: &VkInstance,
        p_create_info: *const VkDebugUtilsMessengerCreateInfoEXT,
        p_allocator: *const VkAllocationCallbacks,
        p_debug_messenger: *mut VkDebugUtilsMessengerEXT,
    ) -> VkResult {
        let p_name = CString::new("vkCreateDebugUtilsMessengerEXT").expect("CString::new failed");
        let function: PFN_vkCreateDebugUtilsMessengerEXT =
            unsafe { std::mem::transmute(vk_get_instance_proc_addr(*instance, p_name.as_ptr())) };

        if !function.is_none() {
            return unsafe {
                function.unwrap()(*instance, p_create_info, p_allocator, p_debug_messenger)
            };
        }

        VkResult::VK_ERROR_EXTENSION_NOT_PRESENT
    }

    fn destroy_debug_utils_messenger_ext(
        &self,
        instance: &VkInstance,
        debug_messenger: &VkDebugUtilsMessengerEXT,
        p_allocator: *const VkAllocationCallbacks,
    ) {
        let p_name = CString::new("vkDestroyDebugUtilsMessengerEXT").expect("CString::new failed");
        let function: PFN_vkDestroyDebugUtilsMessengerEXT =
            unsafe { std::mem::transmute(vk_get_instance_proc_addr(*instance, p_name.as_ptr())) };

        if !function.is_none() {
            unsafe {
                function.unwrap()(*instance, *debug_messenger, p_allocator);
            }
        }
    }

    fn _create_surface(&self) {
        let mut surface: VkSurfaceKHR = unsafe { std::mem::zeroed() };
        let result: VkResult = glfw_create_window_surface(
            *self.instance.get().expect("Instance is null"),
            *self.window.get().expect("Window is null"),
            std::ptr::null(),
            &mut surface,
        );

        self.surface.set(surface).expect("Failed to set surface");

        if result != VK_SUCCESS {
            panic!("Failed to create window surface");
        }
    }

    fn _pick_physical_device(&self) {
        let mut device_count: u32 = 0;
        vk_enumerate_physical_devices(
            *self.instance.get().expect("Instance is null"),
            &mut device_count,
            std::ptr::null_mut(),
        );

        if device_count == 0 {
            panic!("Failed to find GPUs with Vulkan support");
        }

        let mut devices: Vec<VkPhysicalDevice> = Vec::with_capacity(device_count as usize);
        unsafe {
            devices.set_len(device_count as usize);
        }

        vk_enumerate_physical_devices(
            *self.instance.get().expect("Instance is null"),
            &mut device_count,
            devices.as_mut_ptr(),
        );

        for device in &devices {
            if self._is_device_suitable(device) {
                self.physical_device
                    .set(*device)
                    .expect("Failed to set physical device");
                break;
            }
        }

        if self.physical_device.get().is_none() {
            // VK_NULL_HANDLE is nullptr
            panic!("Failed to find a suitable GPU");
        }
    }

    fn _is_device_suitable(&self, device: &VkPhysicalDevice) -> bool {
        let mut device_properties: VkPhysicalDeviceProperties = unsafe { std::mem::zeroed() };
        vk_get_physical_device_properties(*device, &mut device_properties);

        let mut device_features: VkPhysicalDeviceFeatures = unsafe { std::mem::zeroed() };
        vk_get_physical_device_features(*device, &mut device_features);

        let indices: QueueFamilyIndices = self._find_queue_families(device);
        let extensions_supported: bool = self._check_device_extension_support(device);

        let mut swap_chain_adequate: bool = false;
        if extensions_supported {
            let swap_chain_support: SwapChainSupportDetails =
                self._query_swap_chain_support(device);
            swap_chain_adequate = !swap_chain_support.formats.is_empty()
                && !swap_chain_support.present_modes.is_empty();
        }

        indices.is_complete() && extensions_supported && swap_chain_adequate
    }

    fn _query_swap_chain_support(&self, device: &VkPhysicalDevice) -> SwapChainSupportDetails {
        let mut details: SwapChainSupportDetails = SwapChainSupportDetails {
            capabilities: unsafe {
                MaybeUninit::<VkSurfaceCapabilitiesKHR>::zeroed().assume_init()
            },
            formats: Vec::new(),
            present_modes: Vec::new(),
        };
        vk_get_physical_device_surface_capabilities_khr(
            *device,
            *self.surface.get().expect("Surface is null"),
            &mut details.capabilities,
        );

        let mut format_count: u32 = 0;
        vk_get_physical_device_surface_formats_khr(
            *device,
            *self.surface.get().expect("Surface is null"),
            &mut format_count,
            std::ptr::null_mut(),
        );
        if format_count != 0 {
            details.formats = Vec::with_capacity(format_count as usize);
            unsafe {
                details.formats.set_len(format_count as usize);
            }
            vk_get_physical_device_surface_formats_khr(
                *device,
                *self.surface.get().expect("Surface is null"),
                &mut format_count,
                details.formats.as_mut_ptr(),
            );
        }

        let mut present_mode_count: u32 = 0;
        vk_get_physical_device_surface_present_modes_khr(
            *device,
            *self.surface.get().expect("Surface is null"),
            &mut present_mode_count,
            std::ptr::null_mut(),
        );
        if present_mode_count != 0 {
            details.present_modes = Vec::with_capacity(present_mode_count as usize);
            unsafe {
                details.present_modes.set_len(present_mode_count as usize);
            }
            vk_get_physical_device_surface_present_modes_khr(
                *device,
                *self.surface.get().expect("Surface is null"),
                &mut present_mode_count,
                details.present_modes.as_mut_ptr(),
            );
        }

        details
    }

    fn _check_device_extension_support(&self, device: &VkPhysicalDevice) -> bool {
        let mut extension_count: u32 = 0;
        vk_enumerate_device_extension_properties(
            *device,
            std::ptr::null(),
            &mut extension_count,
            std::ptr::null_mut(),
        );

        let mut available_extensions: Vec<VkExtensionProperties> =
            Vec::with_capacity(extension_count as usize);
        unsafe {
            available_extensions.set_len(extension_count as usize);
        }

        vk_enumerate_device_extension_properties(
            *device,
            std::ptr::null(),
            &mut extension_count,
            available_extensions.as_mut_ptr(),
        );

        if debug_mode() {
            println!("Vulkan avaliable device extensions:");
            for extension in &available_extensions {
                let extension_name = unsafe {
                    CStr::from_ptr(extension.extensionName.as_ptr())
                        .to_str()
                        .expect("Failed to convert extension_name to str")
                };
                println!("\t{}", extension_name);
            }
        }

        let mut required_extensions: Vec<CString> = self.device_extensions.clone();

        for extension in &available_extensions {
            let extension_name: &str = unsafe {
                std::ffi::CStr::from_ptr(extension.extensionName.as_ptr())
                    .to_str()
                    .expect("Failed to get extension name")
            };

            required_extensions.retain(|ext: &CString| {
                let ext_str = ext.to_str().expect("Failed to convert extension to str");
                ext_str != extension_name
            });
        }

        required_extensions.is_empty()
    }

    fn _find_queue_families(&self, device: &VkPhysicalDevice) -> QueueFamilyIndices {
        let mut indices = QueueFamilyIndices::new();

        let mut queue_family_count: u32 = 0;
        vk_get_physical_device_queue_family_properties(
            *device,
            &mut queue_family_count,
            std::ptr::null_mut(),
        );

        let mut queue_families: Vec<VkQueueFamilyProperties> =
            Vec::with_capacity(queue_family_count as usize);
        unsafe {
            queue_families.set_len(queue_family_count as usize);
        }
        vk_get_physical_device_queue_family_properties(
            *device,
            &mut queue_family_count,
            queue_families.as_mut_ptr(),
        );

        let mut i = 0;
        for queue_family in &queue_families {
            if queue_family.queueFlags & (VK_QUEUE_GRAPHICS_BIT as u32) != 0 {
                indices.graphics_family = Some(i);
            }

            let mut present_support: VkBool32 = 0;
            vk_get_physical_device_surface_support_khr(
                *device,
                i,
                *self.surface.get().expect("Surface is null"),
                &mut present_support,
            );

            if present_support != 0 {
                indices.present_family = Some(i);
            }

            if indices.is_complete() {
                break;
            }

            i += 1;
        }

        indices
    }

    fn _create_logical_device(&self) {
        let indices: QueueFamilyIndices =
            self._find_queue_families(self.physical_device.get().expect("physical_device is null"));

        let mut queue_create_infos: Vec<VkDeviceQueueCreateInfo> = Vec::new();

        let unique_queue_families: HashSet<u32> = HashSet::from([
            indices.graphics_family.expect("graphics_family is null"),
            indices.present_family.expect("present_family is null"),
        ]);

        let queue_priority: c_float = 1.0;
        for queue_family in unique_queue_families {
            let queue_create_info = VkDeviceQueueCreateInfo {
                sType: VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
                queueFamilyIndex: queue_family,
                queueCount: 1,
                pQueuePriorities: &queue_priority,
                pNext: std::ptr::null(),
                flags: 0,
            };
            queue_create_infos.push(queue_create_info);
        }

        // let queue_create_infos: Vec<VkDeviceQueueCreateInfo> = unsafe {
        //     let mut new_vec: Vec<VkDeviceQueueCreateInfo> = Vec::with_capacity(queue_create_infos.len());
        //     new_vec.set_len(queue_create_infos.len());
        //     new_vec.copy_from_slice(&queue_create_infos);
        //     new_vec
        // };

        // let device_extensions: Vec<c_char> = Vec::new();

        println!("queue_create_infos: >> {:?}", queue_create_infos);

        let mut enabled_layer_count: u32 = 0;
        let mut pp_enabled_layer_names: *const *const i8 = std::ptr::null();
        if self._enable_validation_layers() {
            enabled_layer_count = self.validation_layers.len() as u32;
            pp_enabled_layer_names = self.validation_layers.as_ptr() as *const *const i8;
        }

        let device_features: VkPhysicalDeviceFeatures = unsafe { std::mem::zeroed() };
        let create_info: VkDeviceCreateInfo = VkDeviceCreateInfo {
            sType: VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
            queueCreateInfoCount: queue_create_infos.len() as u32,
            pQueueCreateInfos: queue_create_infos.as_ptr(),
            pEnabledFeatures: &device_features,
            enabledExtensionCount: self.device_extensions.len() as u32,
            ppEnabledExtensionNames: self.device_extensions.as_ptr() as *const *const i8,
            enabledLayerCount: enabled_layer_count,
            ppEnabledLayerNames: pp_enabled_layer_names,
            pNext: std::ptr::null(),
            flags: 0,
        };

        let device: VkDevice = unsafe { std::mem::zeroed() };
        let result: VkResult = vk_create_device(
            *self.physical_device.get().expect("a"),
            &create_info,
            std::ptr::null(),
            &device,
        );

        if result != VK_SUCCESS {
            panic!("Failed to create logical device!");
        }

        self.device
            .set(device)
            .expect("Device can not be inicialized!");

        let mut graphics_queue: VkQueue = unsafe { std::mem::zeroed() };
        let mut present_queue: VkQueue = unsafe { std::mem::zeroed() };
        vk_get_device_queue(
            *self.device.get().expect("self.device is null"),
            indices
                .graphics_family
                .expect("Failed to get indices.graphics_family"),
            0,
            &mut graphics_queue,
        );
        vk_get_device_queue(
            *self.device.get().expect("self.device is null"),
            indices
                .graphics_family
                .expect("Failed to get indices.graphics_family"),
            0,
            &mut present_queue,
        );

        self.graphics_queue
            .set(graphics_queue)
            .expect("Graphics queue can not be inicialized!");
        self.present_queue
            .set(present_queue)
            .expect("Present queue can not be inicialized!");
    }

    fn _create_swap_chain(&self) {
        let swap_chain_support: SwapChainSupportDetails = self._query_swap_chain_support(
            self.physical_device.get().expect("physical_device is null"),
        );

        let surface_format: VkSurfaceFormatKHR =
            self._choose_swap_surface_format(&swap_chain_support.formats);
        let present_mode: VkPresentModeKHR =
            self._choose_swap_present_mode(&swap_chain_support.present_modes);
        let extent: VkExtent2D = self._choose_swap_extent(&swap_chain_support.capabilities);

        let mut image_count: u32 = swap_chain_support.capabilities.minImageCount + 1;
        if swap_chain_support.capabilities.maxImageCount > 0
            && image_count > swap_chain_support.capabilities.maxImageCount
        {
            image_count = swap_chain_support.capabilities.maxImageCount;
        }

        let indices: QueueFamilyIndices =
            self._find_queue_families(self.physical_device.get().expect("physical_device is null"));
        let graphics_family: u32 = indices
            .graphics_family
            .expect("Failed to get indices.graphics_family");
        let present_family: u32 = indices
            .present_family
            .expect("Failed to get indices.present_family");
        let queue_families_indices: Vec<u32> = vec![graphics_family, present_family];

        let mut image_sharing_mode: VkSharingMode = VK_SHARING_MODE_EXCLUSIVE;
        let mut queue_family_index_count: u32 = 0;
        let mut p_queue_family_indices: *const u32 = std::ptr::null();
        if graphics_family != present_family {
            image_sharing_mode = VK_SHARING_MODE_CONCURRENT;
            queue_family_index_count = 2;
            p_queue_family_indices = queue_families_indices.as_ptr();
        }

        let create_info: VkSwapchainCreateInfoKHR = VkSwapchainCreateInfoKHR {
            sType: VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
            surface: *self.surface.get().expect("Failed to get surface"),
            minImageCount: image_count,
            imageFormat: surface_format.format,
            imageColorSpace: surface_format.colorSpace,
            imageExtent: extent,
            imageArrayLayers: 1,
            imageUsage: VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT as u32,
            imageSharingMode: image_sharing_mode,
            queueFamilyIndexCount: queue_family_index_count,
            pQueueFamilyIndices: p_queue_family_indices,
            preTransform: swap_chain_support.capabilities.currentTransform,
            compositeAlpha: VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
            presentMode: present_mode,
            clipped: VK_TRUE,
            oldSwapchain: std::ptr::null_mut(), // C++ nullptr
            pNext: std::ptr::null(),
            flags: 0,
        };

        let mut swapchain: VkSwapchainKHR = unsafe { std::mem::zeroed() };

        let result = vk_create_swapchain_khr(
            *self.device.get().expect("Failed to get device"),
            &create_info,
            std::ptr::null(),
            &mut swapchain,
        );

        if result != VK_SUCCESS {
            panic!("failed to create swap chain!");
        }

        self.swapchain
            .set(swapchain)
            .expect("Failed to set swapchain");

        vk_get_swapchain_images_khr(
            *self.device.get().expect("Failed to get device"),
            *self.swapchain.get().expect("Failed to get swapchain"),
            &mut image_count,
            std::ptr::null_mut(),
        );

        let mut swapchain_images: Vec<VkImage> = Vec::with_capacity(image_count as usize);
        unsafe {
            swapchain_images.set_len(image_count as usize);
        }
        vk_get_swapchain_images_khr(
            *self.device.get().expect("Failed to get device"),
            *self.swapchain.get().expect("Failed to get swapchain"),
            &mut image_count,
            swapchain_images.as_mut_ptr(),
        );

        self.swapchain_images
            .set(swapchain_images)
            .expect("Failed to set self.swapchain_images");
        self.swapchain_image_format
            .set(surface_format.format)
            .expect("Failed to set self.swapchain_image_format");
        self.swapchain_extent
            .set(extent)
            .expect("Failed to set self.swapchain_extent");
    }

    fn _choose_swap_extent(&self, capabilities: &VkSurfaceCapabilitiesKHR) -> VkExtent2D {
        if capabilities.currentExtent.width != std::u32::MAX {
            return capabilities.currentExtent;
        }

        let mut width: c_int = unsafe { std::mem::zeroed() };
        let mut height: c_int = unsafe { std::mem::zeroed() };
        glfw_get_framebuffer_size(
            *self.window.get().expect("Failed to get Window"),
            &mut width,
            &mut height,
        );

        let width: u32 = width.clamp(
            capabilities.minImageExtent.width as i32,
            capabilities.maxImageExtent.width as i32,
        ) as u32;

        let height: u32 = height.clamp(
            capabilities.minImageExtent.height as i32,
            capabilities.maxImageExtent.height as i32,
        ) as u32;

        VkExtent2D { width, height }
    }

    fn _choose_swap_present_mode(
        &self,
        avaliable_present_modes: &Vec<VkPresentModeKHR>,
    ) -> VkPresentModeKHR {
        for avaliable_present_mode in avaliable_present_modes {
            if *avaliable_present_mode == VK_PRESENT_MODE_MAILBOX_KHR {
                return *avaliable_present_mode;
            }
        }

        VK_PRESENT_MODE_FIFO_KHR
    }

    fn _choose_swap_surface_format(
        &self,
        avaliable_formats: &Vec<VkSurfaceFormatKHR>,
    ) -> VkSurfaceFormatKHR {
        for avaliable_format in avaliable_formats {
            if avaliable_format.format == VK_FORMAT_B8G8R8A8_SRGB
                && avaliable_format.colorSpace == VK_COLOR_SPACE_SRGB_NONLINEAR_KHR
            {
                return *avaliable_format;
            }
        }

        avaliable_formats[0]
    }

    fn _create_image_views(&self) {
        let swapchain_images: &Vec<VkImage> = self
            .swapchain_images
            .get()
            .expect("Swapchain images is null");
        let mut swapchain_image_views: Vec<VkImageView> =
            Vec::with_capacity(swapchain_images.len());
        unsafe {
            swapchain_image_views.set_len(swapchain_images.len());
        }

        let mut i = 0;
        for swapchain_image in swapchain_images {
            let create_info: VkImageViewCreateInfo = VkImageViewCreateInfo {
                sType: VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
                image: *swapchain_image,
                viewType: VK_IMAGE_VIEW_TYPE_2D,
                format: *self
                    .swapchain_image_format
                    .get()
                    .expect("Swapchain image format is null"),
                components: VkComponentMapping {
                    r: VK_COMPONENT_SWIZZLE_IDENTITY,
                    g: VK_COMPONENT_SWIZZLE_IDENTITY,
                    b: VK_COMPONENT_SWIZZLE_IDENTITY,
                    a: VK_COMPONENT_SWIZZLE_IDENTITY,
                },
                subresourceRange: VkImageSubresourceRange {
                    aspectMask: VK_IMAGE_ASPECT_COLOR_BIT as u32,
                    baseMipLevel: 0,
                    levelCount: 1,
                    baseArrayLayer: 0,
                    layerCount: 1,
                },
                pNext: std::ptr::null(),
                flags: 0,
            };

            let result = vk_create_image_view(
                *self.device.get().expect("Device is null"),
                &create_info,
                std::ptr::null(),
                &mut swapchain_image_views[i],
            );
            if result != VK_SUCCESS {
                panic!("Failed to create image views!");
            }

            i = i + 1;
        }

        if debug_mode() {
            println!("Vulkan swapchain image views created");
        }
        self.swapchain_image_views
            .set(swapchain_image_views)
            .expect("Failed to set swapchain image views");
    }
    fn _create_render_pass(&self) {}
    fn _create_graphics_pipeline(&self) {}
    fn _create_framebuffers(&self) {}
    fn _create_command_pool(&self) {}
    fn _create_command_buffers(&self) {}
    fn _create_sync_objects(&self) {}
}

impl GraphicApi for VulkanApi {
    fn init_window(&self) -> Window {
        if debug_mode() {
            println!(
                "Vulkan window initialized with width: {} and height: {}",
                self.width, self.height
            );
        }
        if self._enable_validation_layers() {
            println!("Validation layers enabled");
        }

        glfw_init();

        glfw_window_hint(GLFW_CLIENT_API as isize, GLFW_NO_API as isize);
        glfw_window_hint(GLFW_RESIZABLE as isize, GLFW_FALSE as isize);

        let window = glfw_create_window(
            self.width as i32,
            self.height as i32,
            "OITO-CANECO",
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );

        self.window
            .set(window)
            .expect("Glfw window can not be initialized");

        Window::Vulkan(window)
    }

    fn init_api(&self) {
        if debug_mode() {
            println!("Vulkan API initialized");
        }
        self._create_instance();
        self._setup_debug_messenger();
        self._create_surface();
        self._pick_physical_device();
        self._create_logical_device();
        self._create_swap_chain();
        self._create_image_views();
        self._create_render_pass();
        self._create_graphics_pipeline();
        self._create_framebuffers();
        self._create_command_pool();
        self._create_command_buffers();
        self._create_sync_objects();
    }

    fn cleanup(&self) {
        if debug_mode() {
            println!("Vulkan cleanup");
        }

        let swapchain_image_views: &Vec<VkImageView> = self
            .swapchain_image_views
            .get()
            .expect("Swapchain image views is null");
        for swapchain_image_view in swapchain_image_views {
            vk_destroy_image_view(
                *self.device.get().expect("Device is null"),
                *swapchain_image_view,
                std::ptr::null(),
            );
        }

        vk_destroy_swapchain_khr(
            *self.device.get().expect("Device is null"),
            *self.swapchain.get().expect("Swapchain is null"),
            std::ptr::null(),
        );

        vk_destroy_device(
            *self.device.get().expect("Device is null"),
            std::ptr::null(),
        );

        vk_destroy_surface_khr(
            *self.instance.get().expect("Instance is null"),
            *self.surface.get().expect("Surface is null"),
            std::ptr::null(),
        );

        if self._enable_validation_layers() {
            self.destroy_debug_utils_messenger_ext(
                self.instance.get().expect("Instance is null"),
                self.debug_messenger.get().expect("Debug messenger is null"),
                std::ptr::null(),
            );
        }

        vk_destroy_instance(
            *self.instance.get().expect("Instance is null"),
            std::ptr::null(),
        );
        glfw_destroy_window(*self.window.get().expect("Window is null"));
        glfw_terminate();
    }

    fn should_close(&self) -> bool {
        let should_close: i32 =
            glfw_window_should_close(*self.window.get().expect("Window is null"));

        should_close != 0
    }

    fn pool_events(&self) {
        glfw_poll_events();
    }
}
