use std::cell::OnceCell;
use std::ffi::{c_char, c_void, CStr, CString};
use std::vec;

use crate::glfw::{
    glfw_create_window, glfw_create_window_surface, glfw_destroy_window,
    glfw_get_required_instance_extensions, glfw_init, glfw_poll_events, glfw_terminate,
    glfw_window_hint, glfw_window_should_close, GLFW_CLIENT_API, GLFW_FALSE, GLFW_NO_API,
    GLFW_RESIZABLE,
};
use crate::utils::debug_mode;
use crate::vulkan::{
    self, vk_bit_message_severity, vk_bit_message_type, vk_destroy_instance,
    vk_destroy_surface_khr, vk_enumerate_instance_extension_properties, vk_get_instance_proc_addr,
    PFN_vkCreateDebugUtilsMessengerEXT, PFN_vkDebugUtilsMessengerCallbackEXT,
    PFN_vkDestroyDebugUtilsMessengerEXT, VkAllocationCallbacks, VkApplicationInfo, VkBool32,
    VkDebugUtilsMessageSeverityFlagBitsEXT, VkDebugUtilsMessageTypeFlagBitsEXT,
    VkDebugUtilsMessageTypeFlagsEXT, VkDebugUtilsMessengerCallbackDataEXT,
    VkDebugUtilsMessengerCreateInfoEXT, VkDebugUtilsMessengerEXT, VkInstance,
    VkInstanceCreateFlags, VkInstanceCreateInfo, VkResult, VkStructureType, VkSurfaceKHR, VK_FALSE,
};
use crate::{glfw::GLFWwindow, utils};

use VkDebugUtilsMessageSeverityFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT;
use VkDebugUtilsMessageSeverityFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT;
use VkDebugUtilsMessageSeverityFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT;
use VkDebugUtilsMessageTypeFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT;
use VkDebugUtilsMessageTypeFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT;
use VkDebugUtilsMessageTypeFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT;

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

pub struct VulkanApi {
    width: usize,
    height: usize,
    window: OnceCell<*mut GLFWwindow>,
    validation_layers: Vec<CString>,
    instance: OnceCell<VkInstance>,
    debug_messenger: OnceCell<VkDebugUtilsMessengerEXT>,
    surface: OnceCell<VkSurfaceKHR>,
}

impl VulkanApi {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            window: OnceCell::new(),
            validation_layers: vec![CString::new("VK_LAYER_KHRONOS_validation")
                .expect("CString::new VK_LAYER_KHRONOS_validation failed!")],
            instance: OnceCell::new(),
            debug_messenger: OnceCell::new(),
            surface: OnceCell::new(),
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
            applicationVersion: vulkan::VK_MAKE_API_VERSION(0, 1, 0, 0),
            pEngineName: engine_name.as_ptr(),
            engineVersion: vulkan::VK_MAKE_API_VERSION(0, 1, 0, 0),
            apiVersion: vulkan::VK_API_VERSION_1_0,
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
        let result: VkResult =
            vulkan::vk_create_instance(&create_info, std::ptr::null(), &mut instance);
        if result != VkResult::VK_SUCCESS {
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
            extensions.push(vulkan::VK_EXT_DEBUG_UTILS_EXTENSION_NAME.as_ptr() as *const c_char);
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

        let mut available_extensions: Vec<vulkan::VkExtensionProperties> =
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
            println!("Available extensions:");
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
        vulkan::vk_enumerate_instance_layer_properties(
            &mut layer_count as *mut u32,
            std::ptr::null_mut(),
        );
        let mut available_layers: Vec<vulkan::VkLayerProperties> =
            Vec::with_capacity(layer_count as usize);

        unsafe {
            available_layers.set_len(layer_count as usize);
        }

        vulkan::vk_enumerate_instance_layer_properties(
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

        if result != VkResult::VK_SUCCESS {
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

    fn create_surface(&self) {
        let mut surface: VkSurfaceKHR = unsafe { std::mem::zeroed() };
        let result: VkResult = glfw_create_window_surface(
            *self.instance.get().expect("Instance is null"),
            *self.window.get().expect("Window is null"),
            std::ptr::null(),
            &mut surface,
        );

        self.surface.set(surface).expect("Failed to set surface");

        if result != VkResult::VK_SUCCESS {
            panic!("Failed to create window surface");
        }
    }
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

        glfw_window_hint(GLFW_CLIENT_API, GLFW_NO_API);
        glfw_window_hint(GLFW_RESIZABLE, GLFW_FALSE);

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
        self.create_surface();
    }

    fn cleanup(&self) {
        if debug_mode() {
            println!("Vulkan cleanup");
        }

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
