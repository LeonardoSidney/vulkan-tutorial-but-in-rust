use std::cell::OnceCell;
use std::collections::HashSet;
use std::ffi::{c_char, c_float, c_int, c_void, CStr, CString};
use std::fs::File;
use std::io::Read;
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
    vk_acquire_next_image_khr, vk_allocate_command_buffers, vk_begin_command_buffer,
    vk_bit_message_severity, vk_bit_message_type, vk_cmd_begin_render_pass, vk_cmd_bind_pipeline,
    vk_cmd_draw, vk_cmd_end_render_pass, vk_cmd_set_scissor, vk_cmd_set_viewport,
    vk_create_command_pool, vk_create_device, vk_create_fence, vk_create_framebuffer,
    vk_create_graphics_pipelines, vk_create_image_view, vk_create_instance,
    vk_create_pipeline_layout, vk_create_render_pass, vk_create_semaphore, vk_create_shader_module,
    vk_create_swapchain_khr, vk_destroy_command_pool, vk_destroy_device, vk_destroy_fence,
    vk_destroy_framebuffer, vk_destroy_image_view, vk_destroy_instance, vk_destroy_pipeline,
    vk_destroy_pipeline_layout, vk_destroy_render_pass, vk_destroy_semaphore,
    vk_destroy_shader_module, vk_destroy_surface_khr, vk_destroy_swapchain_khr,
    vk_device_wait_idle, vk_end_command_buffer, vk_enumerate_device_extension_properties,
    vk_enumerate_instance_extension_properties, vk_enumerate_instance_layer_properties,
    vk_enumerate_physical_devices, vk_get_device_queue, vk_get_instance_proc_addr,
    vk_get_physical_device_features, vk_get_physical_device_properties,
    vk_get_physical_device_queue_family_properties,
    vk_get_physical_device_surface_capabilities_khr, vk_get_physical_device_surface_formats_khr,
    vk_get_physical_device_surface_present_modes_khr, vk_get_physical_device_surface_support_khr,
    vk_get_swapchain_images_khr, vk_queue_present_khr, vk_queue_submit, vk_reset_command_buffer,
    vk_reset_fences, vk_wait_for_fences, PFN_vkCreateDebugUtilsMessengerEXT,
    PFN_vkDebugUtilsMessengerCallbackEXT, PFN_vkDestroyDebugUtilsMessengerEXT, VkAccessFlagBits,
    VkAllocationCallbacks, VkApplicationInfo, VkAttachmentDescription, VkAttachmentLoadOp,
    VkAttachmentReference, VkAttachmentStoreOp, VkBlendFactor, VkBlendOp, VkBool32,
    VkClearColorValue, VkClearValue, VkColorComponentFlagBits, VkColorSpaceKHR, VkCommandBuffer,
    VkCommandBufferAllocateInfo, VkCommandBufferBeginInfo, VkCommandBufferLevel, VkCommandPool,
    VkCommandPoolCreateFlagBits, VkCommandPoolCreateInfo, VkComponentMapping, VkComponentSwizzle,
    VkCompositeAlphaFlagBitsKHR, VkCullModeFlagBits, VkDebugUtilsMessageSeverityFlagBitsEXT,
    VkDebugUtilsMessageTypeFlagBitsEXT, VkDebugUtilsMessageTypeFlagsEXT,
    VkDebugUtilsMessengerCallbackDataEXT, VkDebugUtilsMessengerCreateInfoEXT,
    VkDebugUtilsMessengerEXT, VkDevice, VkDeviceCreateInfo, VkDeviceQueueCreateInfo,
    VkDynamicState, VkExtensionProperties, VkExtent2D, VkFence, VkFenceCreateFlagBits,
    VkFenceCreateInfo, VkFormat, VkFramebuffer, VkFramebufferCreateInfo, VkFrontFace,
    VkGraphicsPipelineCreateInfo, VkImage, VkImageAspectFlagBits, VkImageLayout,
    VkImageSubresourceRange, VkImageUsageFlagBits, VkImageView, VkImageViewCreateInfo,
    VkImageViewType, VkInstance, VkInstanceCreateFlags, VkInstanceCreateInfo, VkLayerProperties,
    VkLogicOp, VkOffset2D, VkPhysicalDevice, VkPhysicalDeviceFeatures, VkPhysicalDeviceProperties,
    VkPipeline, VkPipelineBindPoint, VkPipelineColorBlendAttachmentState,
    VkPipelineColorBlendStateCreateInfo, VkPipelineDynamicStateCreateInfo,
    VkPipelineInputAssemblyStateCreateInfo, VkPipelineLayout, VkPipelineLayoutCreateInfo,
    VkPipelineMultisampleStateCreateInfo, VkPipelineRasterizationStateCreateInfo,
    VkPipelineShaderStageCreateInfo, VkPipelineStageFlagBits, VkPipelineStageFlags,
    VkPipelineVertexInputStateCreateInfo, VkPipelineViewportStateCreateInfo, VkPolygonMode,
    VkPresentInfoKHR, VkPresentModeKHR, VkPrimitiveTopology, VkQueue, VkQueueFamilyProperties,
    VkQueueFlagBits, VkRect2D, VkRenderPass, VkRenderPassBeginInfo, VkRenderPassCreateInfo,
    VkResult, VkSampleCountFlagBits, VkSemaphore, VkSemaphoreCreateInfo, VkShaderModule,
    VkShaderModuleCreateInfo, VkShaderStageFlagBits, VkSharingMode, VkStructureType, VkSubmitInfo,
    VkSubpassContents, VkSubpassDependency, VkSubpassDescription, VkSurfaceCapabilitiesKHR,
    VkSurfaceFormatKHR, VkSurfaceKHR, VkSwapchainCreateInfoKHR, VkSwapchainKHR, VkViewport,
    VK_API_VERSION_1_0, VK_EXT_DEBUG_UTILS_EXTENSION_NAME, VK_FALSE,
    VK_KHR_SWAPCHAIN_EXTENSION_NAME, VK_MAKE_API_VERSION, VK_SUBPASS_EXTERNAL, VK_TRUE,
};
use crate::{glfw::GLFWwindow, utils};

use VkAccessFlagBits::VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT;
use VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_CLEAR;
use VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_DONT_CARE;
use VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_DONT_CARE;
use VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_STORE;
use VkBlendFactor::VK_BLEND_FACTOR_ONE;
use VkBlendFactor::VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA;
use VkBlendFactor::VK_BLEND_FACTOR_SRC_ALPHA;
use VkBlendFactor::VK_BLEND_FACTOR_ZERO;
use VkBlendOp::VK_BLEND_OP_ADD;
use VkColorComponentFlagBits::VK_COLOR_COMPONENT_A_BIT;
use VkColorComponentFlagBits::VK_COLOR_COMPONENT_B_BIT;
use VkColorComponentFlagBits::VK_COLOR_COMPONENT_G_BIT;
use VkColorComponentFlagBits::VK_COLOR_COMPONENT_R_BIT;
use VkColorSpaceKHR::VK_COLOR_SPACE_SRGB_NONLINEAR_KHR;
use VkCommandBufferLevel::VK_COMMAND_BUFFER_LEVEL_PRIMARY;
use VkCommandPoolCreateFlagBits::VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT;
use VkComponentSwizzle::VK_COMPONENT_SWIZZLE_IDENTITY;
use VkCompositeAlphaFlagBitsKHR::VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR;
use VkCullModeFlagBits::VK_CULL_MODE_BACK_BIT;
use VkDebugUtilsMessageSeverityFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT;
use VkDebugUtilsMessageSeverityFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT;
use VkDebugUtilsMessageSeverityFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT;
use VkDebugUtilsMessageTypeFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT;
use VkDebugUtilsMessageTypeFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT;
use VkDebugUtilsMessageTypeFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT;
use VkDynamicState::VK_DYNAMIC_STATE_SCISSOR;
use VkDynamicState::VK_DYNAMIC_STATE_VIEWPORT;
use VkFenceCreateFlagBits::VK_FENCE_CREATE_SIGNALED_BIT;
use VkFormat::VK_FORMAT_B8G8R8A8_SRGB;
use VkFrontFace::VK_FRONT_FACE_CLOCKWISE;
use VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT;
use VkImageLayout::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL;
use VkImageLayout::VK_IMAGE_LAYOUT_PRESENT_SRC_KHR;
use VkImageLayout::VK_IMAGE_LAYOUT_UNDEFINED;
use VkImageUsageFlagBits::VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT;
use VkImageViewType::VK_IMAGE_VIEW_TYPE_2D;
use VkLogicOp::VK_LOGIC_OP_COPY;
use VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS;
use VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT;
use VkPolygonMode::VK_POLYGON_MODE_FILL;
use VkPresentModeKHR::VK_PRESENT_MODE_FIFO_KHR;
use VkPresentModeKHR::VK_PRESENT_MODE_MAILBOX_KHR;
use VkPrimitiveTopology::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST;
use VkQueueFlagBits::VK_QUEUE_GRAPHICS_BIT;
use VkResult::VK_SUCCESS;
use VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT;
use VkShaderStageFlagBits::VK_SHADER_STAGE_FRAGMENT_BIT;
use VkShaderStageFlagBits::VK_SHADER_STAGE_VERTEX_BIT;
use VkSharingMode::VK_SHARING_MODE_CONCURRENT;
use VkSharingMode::VK_SHARING_MODE_EXCLUSIVE;
use VkStructureType::{
    VK_STRUCTURE_TYPE_APPLICATION_INFO, VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
    VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO, VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
    VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT, VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
    VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO, VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
    VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO, VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,
    VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO, VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
    VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
    VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO,
    VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
    VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
    VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
    VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
    VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
    VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
    VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO, VK_STRUCTURE_TYPE_PRESENT_INFO_KHR,
    VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO, VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO,
    VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO, VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
    VK_STRUCTURE_TYPE_SUBMIT_INFO, VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
};
use VkSubpassContents::VK_SUBPASS_CONTENTS_INLINE;

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

    fn get_graphics_family(&self) -> u32 {
        self.graphics_family.expect("Graphics family is null")
    }

    fn get_present_family(&self) -> u32 {
        self.present_family.expect("Present family is null")
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
    render_pass: OnceCell<VkRenderPass>,
    pipeline_layout: OnceCell<VkPipelineLayout>,
    graphics_pipeline: OnceCell<VkPipeline>,
    swapchain_framebuffers: OnceCell<Vec<VkFramebuffer>>,
    command_pool: OnceCell<VkCommandPool>,
    command_buffer: OnceCell<VkCommandBuffer>,
    image_available_semaphore: OnceCell<VkSemaphore>,
    render_finished_semaphore: OnceCell<VkSemaphore>,
    in_flight_fence: OnceCell<VkFence>,
}

impl VulkanApi {
    fn _get_window(&self) -> *mut GLFWwindow {
        *self.window.get().expect("Window is null")
    }

    fn _get_instance(&self) -> VkInstance {
        *self.instance.get().expect("Instance is null")
    }

    fn _get_debug_messenger(&self) -> VkDebugUtilsMessengerEXT {
        *self.debug_messenger.get().expect("Debug messenger is null")
    }

    fn _get_surface(&self) -> VkSurfaceKHR {
        *self.surface.get().expect("Surface is null")
    }

    fn _get_physical_device(&self) -> VkPhysicalDevice {
        *self.physical_device.get().expect("Physical device is null")
    }

    fn _get_device(&self) -> VkDevice {
        *self.device.get().expect("Device is null")
    }

    fn _get_graphics_queue(&self) -> VkQueue {
        *self.graphics_queue.get().expect("Graphics queue is null")
    }

    fn _get_present_queue(&self) -> VkQueue {
        *self.present_queue.get().expect("Present queue is null")
    }

    fn _get_swapchain(&self) -> VkSwapchainKHR {
        *self.swapchain.get().expect("Swapchain is null")
    }

    fn _get_swapchain_images(&self) -> &Vec<VkImage> {
        self.swapchain_images
            .get()
            .expect("Swapchain images is null")
    }

    fn _get_swapchain_image_format(&self) -> VkFormat {
        *self
            .swapchain_image_format
            .get()
            .expect("Swapchain image format is null")
    }

    fn _get_swapchain_extent(&self) -> VkExtent2D {
        *self
            .swapchain_extent
            .get()
            .expect("Swapchain extent is null")
    }

    fn _get_swapchain_image_views(&self) -> &Vec<VkImageView> {
        self.swapchain_image_views
            .get()
            .expect("Swapchain image views is null")
    }

    fn _get_render_pass(&self) -> VkRenderPass {
        *self.render_pass.get().expect("Render pass is null")
    }

    fn _get_pipeline_layout(&self) -> VkPipelineLayout {
        *self.pipeline_layout.get().expect("Pipeline layout is null")
    }

    fn _get_graphics_pipeline(&self) -> VkPipeline {
        *self
            .graphics_pipeline
            .get()
            .expect("Graphics pipeline is null")
    }

    fn _get_swapchain_framebuffers(&self) -> &Vec<VkFramebuffer> {
        self.swapchain_framebuffers
            .get()
            .expect("Swapchain framebuffers is null")
    }

    fn _get_command_pool(&self) -> VkCommandPool {
        *self.command_pool.get().expect("Command pool is null")
    }

    fn _get_command_buffer(&self) -> VkCommandBuffer {
        *self.command_buffer.get().expect("Command buffer is null")
    }

    fn _get_image_available_semaphore(&self) -> VkSemaphore {
        *self
            .image_available_semaphore
            .get()
            .expect("Image available semaphore is null")
    }

    fn _get_render_finished_semaphore(&self) -> VkSemaphore {
        *self
            .render_finished_semaphore
            .get()
            .expect("Render finished semaphore is null")
    }

    fn _get_in_flight_fence(&self) -> VkFence {
        *self.in_flight_fence.get().expect("In flight fence is null")
    }
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
            render_pass: OnceCell::new(),
            pipeline_layout: OnceCell::new(),
            graphics_pipeline: OnceCell::new(),
            swapchain_framebuffers: OnceCell::new(),
            command_pool: OnceCell::new(),
            command_buffer: OnceCell::new(),
            image_available_semaphore: OnceCell::new(),
            render_finished_semaphore: OnceCell::new(),
            in_flight_fence: OnceCell::new(),
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
            sType: VK_STRUCTURE_TYPE_APPLICATION_INFO,
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
            sType: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
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
            sType: VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
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
            &self._get_instance(),
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
            self._get_instance(),
            self._get_window(),
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
            self._get_instance(),
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
            self._get_instance(),
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
            self._get_surface(),
            &mut details.capabilities,
        );

        let mut format_count: u32 = 0;
        vk_get_physical_device_surface_formats_khr(
            *device,
            self._get_surface(),
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
                self._get_surface(),
                &mut format_count,
                details.formats.as_mut_ptr(),
            );
        }

        let mut present_mode_count: u32 = 0;
        vk_get_physical_device_surface_present_modes_khr(
            *device,
            self._get_surface(),
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
                self._get_surface(),
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
                self._get_surface(),
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
        let indices: QueueFamilyIndices = self._find_queue_families(&self._get_physical_device());

        let mut queue_create_infos: Vec<VkDeviceQueueCreateInfo> = Vec::new();
        let unique_queue_families: HashSet<u32> =
            HashSet::from([indices.get_graphics_family(), indices.get_present_family()]);
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
            self._get_physical_device(),
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
            self._get_device(),
            indices.get_graphics_family(),
            0,
            &mut graphics_queue,
        );
        vk_get_device_queue(
            self._get_device(),
            indices.get_present_family(),
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
        let swap_chain_support: SwapChainSupportDetails =
            self._query_swap_chain_support(&self._get_physical_device());

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

        let indices: QueueFamilyIndices = self._find_queue_families(&self._get_physical_device());
        let queue_families_indices: Vec<u32> =
            vec![indices.get_graphics_family(), indices.get_present_family()];

        let mut image_sharing_mode: VkSharingMode = VK_SHARING_MODE_EXCLUSIVE;
        let mut queue_family_index_count: u32 = 0;
        let mut p_queue_family_indices: *const u32 = std::ptr::null();
        if indices.get_graphics_family() != indices.get_present_family() {
            image_sharing_mode = VK_SHARING_MODE_CONCURRENT;
            queue_family_index_count = 2;
            p_queue_family_indices = queue_families_indices.as_ptr();
        }

        let create_info: VkSwapchainCreateInfoKHR = VkSwapchainCreateInfoKHR {
            sType: VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
            surface: self._get_surface(),
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
            self._get_device(),
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
            self._get_device(),
            self._get_swapchain(),
            &mut image_count,
            std::ptr::null_mut(),
        );

        let mut swapchain_images: Vec<VkImage> = Vec::with_capacity(image_count as usize);
        unsafe {
            swapchain_images.set_len(image_count as usize);
        }
        vk_get_swapchain_images_khr(
            self._get_device(),
            self._get_swapchain(),
            &mut image_count,
            swapchain_images.as_mut_ptr(),
        );

        self.swapchain_images
            .set(swapchain_images)
            .expect("Failed to set swapchain images");
        self.swapchain_image_format
            .set(surface_format.format)
            .expect("Failed to set swapchain image format");
        self.swapchain_extent
            .set(extent)
            .expect("Failed to set swapchain extent");
    }

    fn _choose_swap_extent(&self, capabilities: &VkSurfaceCapabilitiesKHR) -> VkExtent2D {
        if capabilities.currentExtent.width != std::u32::MAX {
            return capabilities.currentExtent;
        }

        let mut width: c_int = unsafe { std::mem::zeroed() };
        let mut height: c_int = unsafe { std::mem::zeroed() };
        glfw_get_framebuffer_size(self._get_window(), &mut width, &mut height);

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
        let mut swapchain_image_views: Vec<VkImageView> =
            Vec::with_capacity(self._get_swapchain_images().len());
        unsafe {
            swapchain_image_views.set_len(self._get_swapchain_images().len());
        }

        let mut i = 0;
        for swapchain_image in self._get_swapchain_images() {
            let create_info: VkImageViewCreateInfo = VkImageViewCreateInfo {
                sType: VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
                image: *swapchain_image,
                viewType: VK_IMAGE_VIEW_TYPE_2D,
                format: self._get_swapchain_image_format(),
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
                self._get_device(),
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
    fn _create_render_pass(&self) {
        let color_attachment: VkAttachmentDescription = VkAttachmentDescription {
            format: self._get_swapchain_image_format(),
            samples: VK_SAMPLE_COUNT_1_BIT,
            loadOp: VK_ATTACHMENT_LOAD_OP_CLEAR,
            storeOp: VK_ATTACHMENT_STORE_OP_STORE,
            stencilLoadOp: VK_ATTACHMENT_LOAD_OP_DONT_CARE,
            stencilStoreOp: VK_ATTACHMENT_STORE_OP_DONT_CARE,
            initialLayout: VK_IMAGE_LAYOUT_UNDEFINED,
            finalLayout: VK_IMAGE_LAYOUT_PRESENT_SRC_KHR,
            flags: 0,
        };

        let color_attachment_ref: VkAttachmentReference = VkAttachmentReference {
            attachment: 0,
            layout: VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
        };

        let subpass: VkSubpassDescription = VkSubpassDescription {
            pipelineBindPoint: VK_PIPELINE_BIND_POINT_GRAPHICS,
            colorAttachmentCount: 1,
            pColorAttachments: &color_attachment_ref,
            inputAttachmentCount: 0,
            pInputAttachments: std::ptr::null(),
            pResolveAttachments: std::ptr::null(),
            pDepthStencilAttachment: std::ptr::null(),
            preserveAttachmentCount: 0,
            pPreserveAttachments: std::ptr::null(),
            flags: 0,
        };

        let dependency: VkSubpassDependency = VkSubpassDependency {
            srcSubpass: VK_SUBPASS_EXTERNAL as u32,
            dstSubpass: 0,
            srcStageMask: VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as u32,
            srcAccessMask: 0,
            dstStageMask: VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as u32,
            dstAccessMask: VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT as u32,
            dependencyFlags: 0,
        };

        let render_pass_info: VkRenderPassCreateInfo = VkRenderPassCreateInfo {
            sType: VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO,
            attachmentCount: 1,
            pAttachments: &color_attachment,
            subpassCount: 1,
            pSubpasses: &subpass,
            dependencyCount: 1,
            pDependencies: &dependency,
            pNext: std::ptr::null(),
            flags: 0,
        };

        let mut render_pass: VkRenderPass = unsafe { std::mem::zeroed() };
        let result: VkResult = vk_create_render_pass(
            self._get_device(),
            &render_pass_info,
            std::ptr::null(),
            &mut render_pass,
        );
        if result != VK_SUCCESS {
            panic!("Failed to create render pass!");
        }

        self.render_pass
            .set(render_pass)
            .expect("Render pass can not be initialized!");
    }
    fn _create_graphics_pipeline(&self) {
        if debug_mode() {
            println!("Creating graphics pipeline");
        }

        let vert_shader_code: Vec<c_char> = self._read_file("src/shaders/shader.vert.spv");
        let frag_shader_code: Vec<c_char> = self._read_file("src/shaders/shader.frag.spv");

        let vert_shader_module: VkShaderModule = self._create_shader_module(&vert_shader_code);
        let frag_shader_module: VkShaderModule = self._create_shader_module(&frag_shader_code);

        let queue_name = CString::new("main").expect("CString::new failed");
        let vert_shader_stage_info: VkPipelineShaderStageCreateInfo =
            VkPipelineShaderStageCreateInfo {
                sType: VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
                stage: VK_SHADER_STAGE_VERTEX_BIT,
                module: vert_shader_module,
                pName: queue_name.as_ptr(),
                pSpecializationInfo: std::ptr::null(),
                pNext: std::ptr::null(),
                flags: 0,
            };

        let frag_shader_stage_info: VkPipelineShaderStageCreateInfo =
            VkPipelineShaderStageCreateInfo {
                sType: VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
                stage: VK_SHADER_STAGE_FRAGMENT_BIT,
                module: frag_shader_module,
                pName: queue_name.as_ptr(),
                pSpecializationInfo: std::ptr::null(),
                pNext: std::ptr::null(),
                flags: 0,
            };

        let shader_stages: Vec<VkPipelineShaderStageCreateInfo> =
            vec![vert_shader_stage_info, frag_shader_stage_info];

        let vertex_input_info: VkPipelineVertexInputStateCreateInfo =
            VkPipelineVertexInputStateCreateInfo {
                sType: VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
                vertexBindingDescriptionCount: 0,
                pVertexBindingDescriptions: std::ptr::null(),
                vertexAttributeDescriptionCount: 0,
                pVertexAttributeDescriptions: std::ptr::null(),
                pNext: std::ptr::null(),
                flags: 0,
            };

        let input_assembly: VkPipelineInputAssemblyStateCreateInfo =
            VkPipelineInputAssemblyStateCreateInfo {
                sType: VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
                topology: VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
                primitiveRestartEnable: VK_FALSE,
                pNext: std::ptr::null(),
                flags: 0,
            };

        let viewport: VkViewport = VkViewport {
            x: 0.0,
            y: 0.0,
            width: self._get_swapchain_extent().width as c_float,
            height: self._get_swapchain_extent().height as c_float,
            minDepth: 0.0,
            maxDepth: 1.0,
        };

        let scissor: VkRect2D = VkRect2D {
            offset: VkOffset2D { x: 0, y: 0 },
            extent: self._get_swapchain_extent(),
        };

        let viewport_state: VkPipelineViewportStateCreateInfo = VkPipelineViewportStateCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO,
            viewportCount: 1,
            pViewports: &viewport,
            scissorCount: 1,
            pScissors: &scissor,
            pNext: std::ptr::null(),
            flags: 0,
        };

        let rasterizer: VkPipelineRasterizationStateCreateInfo =
            VkPipelineRasterizationStateCreateInfo {
                sType: VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
                depthClampEnable: VK_FALSE,
                rasterizerDiscardEnable: VK_FALSE,
                polygonMode: VK_POLYGON_MODE_FILL,
                lineWidth: 1.0,
                cullMode: VK_CULL_MODE_BACK_BIT as u32,
                frontFace: VK_FRONT_FACE_CLOCKWISE,
                depthBiasEnable: VK_FALSE,
                depthBiasConstantFactor: 0.0,
                depthBiasClamp: 0.0,
                depthBiasSlopeFactor: 0.0,
                pNext: std::ptr::null(),
                flags: 0,
            };

        let multisampling: VkPipelineMultisampleStateCreateInfo =
            VkPipelineMultisampleStateCreateInfo {
                sType: VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
                sampleShadingEnable: VK_FALSE,
                rasterizationSamples: VK_SAMPLE_COUNT_1_BIT,
                minSampleShading: 1.0,
                pSampleMask: std::ptr::null(),
                alphaToCoverageEnable: VK_FALSE,
                alphaToOneEnable: VK_FALSE,
                pNext: std::ptr::null(),
                flags: 0,
            };

        let color_white_mask_bit_or: u32 = VK_COLOR_COMPONENT_R_BIT as u32
            | VK_COLOR_COMPONENT_G_BIT as u32
            | VK_COLOR_COMPONENT_B_BIT as u32
            | VK_COLOR_COMPONENT_A_BIT as u32;

        let color_blend_attachment: VkPipelineColorBlendAttachmentState =
            VkPipelineColorBlendAttachmentState {
                colorWriteMask: color_white_mask_bit_or,
                blendEnable: VK_FALSE,
                srcColorBlendFactor: VK_BLEND_FACTOR_SRC_ALPHA,
                dstColorBlendFactor: VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA,
                colorBlendOp: VK_BLEND_OP_ADD,
                srcAlphaBlendFactor: VK_BLEND_FACTOR_ONE,
                dstAlphaBlendFactor: VK_BLEND_FACTOR_ZERO,
                alphaBlendOp: VK_BLEND_OP_ADD,
            };

        let color_blending: VkPipelineColorBlendStateCreateInfo =
            VkPipelineColorBlendStateCreateInfo {
                sType: VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
                logicOpEnable: VK_FALSE,
                logicOp: VK_LOGIC_OP_COPY,
                attachmentCount: 1,
                pAttachments: &color_blend_attachment,
                blendConstants: [0.0, 0.0, 0.0, 0.0],
                pNext: std::ptr::null(),
                flags: 0,
            };

        let dynamic_states: Vec<VkDynamicState> =
            vec![VK_DYNAMIC_STATE_VIEWPORT, VK_DYNAMIC_STATE_SCISSOR];

        let dynamic_state: VkPipelineDynamicStateCreateInfo = VkPipelineDynamicStateCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO,
            dynamicStateCount: dynamic_states.len() as u32,
            pDynamicStates: dynamic_states.as_ptr(),
            pNext: std::ptr::null(),
            flags: 0,
        };

        let pipeline_layout_info: VkPipelineLayoutCreateInfo = VkPipelineLayoutCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
            setLayoutCount: 0,
            pSetLayouts: std::ptr::null(),
            pushConstantRangeCount: 0,
            pPushConstantRanges: std::ptr::null(),
            pNext: std::ptr::null(),
            flags: 0,
        };

        let mut pipeline_layout: VkPipelineLayout = unsafe { std::mem::zeroed() };
        let result = vk_create_pipeline_layout(
            self._get_device(),
            &pipeline_layout_info,
            std::ptr::null(),
            &mut pipeline_layout,
        );
        if result != VK_SUCCESS {
            panic!("Failed to create pipeline layout!");
        }
        self.pipeline_layout
            .set(pipeline_layout)
            .expect("Pipeline layout can not be initialized!");

        let graphics_pipeline_info: VkGraphicsPipelineCreateInfo = VkGraphicsPipelineCreateInfo {
            sType: VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,
            stageCount: shader_stages.len() as u32,
            pStages: shader_stages.as_ptr(),
            pVertexInputState: &vertex_input_info,
            pInputAssemblyState: &input_assembly,
            pViewportState: &viewport_state,
            pRasterizationState: &rasterizer,
            pMultisampleState: &multisampling,
            pDepthStencilState: std::ptr::null(),
            pColorBlendState: &color_blending,
            pDynamicState: &dynamic_state,
            layout: self._get_pipeline_layout(),
            renderPass: self._get_render_pass(),
            subpass: 0,
            basePipelineHandle: std::ptr::null_mut(),
            basePipelineIndex: -1,
            pTessellationState: std::ptr::null(),
            pNext: std::ptr::null(),
            flags: 0,
        };

        let mut graphics_pipeline: VkPipeline = unsafe { std::mem::zeroed() };
        let result: VkResult = vk_create_graphics_pipelines(
            self._get_device(),
            std::ptr::null_mut(),
            1,
            &graphics_pipeline_info,
            std::ptr::null(),
            &mut graphics_pipeline,
        );
        if result != VK_SUCCESS {
            panic!("Failed to create graphics pipeline!");
        }
        self.graphics_pipeline
            .set(graphics_pipeline)
            .expect("Graphics pipeline can not be initialized!");

        vk_destroy_shader_module(self._get_device(), vert_shader_module, std::ptr::null());
        vk_destroy_shader_module(self._get_device(), frag_shader_module, std::ptr::null());
    }

    fn _create_shader_module(&self, code: &Vec<c_char>) -> VkShaderModule {
        let create_info: VkShaderModuleCreateInfo = VkShaderModuleCreateInfo {
            sType: VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
            codeSize: code.len() as usize,
            pCode: code.as_ptr() as *const u32,
            pNext: std::ptr::null(),
            flags: 0,
        };

        let mut shader_module: VkShaderModule = unsafe { std::mem::zeroed() };
        let result: VkResult = vk_create_shader_module(
            self._get_device(),
            &create_info,
            std::ptr::null(),
            &mut shader_module,
        );

        if result != VK_SUCCESS {
            panic!("Failed to create shader module!");
        }

        shader_module
    }

    fn _read_file(&self, filename: &str) -> Vec<c_char> {
        let mut file: File =
            File::open(filename).expect(&format!("Failed to open file: {}", filename));
        let mut content: Vec<u8> = Vec::new();
        file.read_to_end(&mut content)
            .expect(&format!("Failed to read file: {}", filename));

        let mut result: Vec<c_char> = Vec::with_capacity(content.len());
        for byte in content {
            result.push(byte as c_char);
        }

        if debug_mode() {
            println!("File {} read successfully", filename);
        }

        result
    }
    fn _create_framebuffers(&self) {
        let mut swapchain_framebuffers: Vec<VkFramebuffer> =
            Vec::with_capacity(self._get_swapchain_image_views().len());
        for swapchain_image_view in self._get_swapchain_image_views() {
            let attachments: Vec<VkImageView> = vec![*swapchain_image_view];
            let framebuffer_info: VkFramebufferCreateInfo = VkFramebufferCreateInfo {
                sType: VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
                renderPass: self._get_render_pass(),
                attachmentCount: 1,
                pAttachments: attachments.as_ptr(),
                width: self._get_swapchain_extent().width,
                height: self._get_swapchain_extent().height,
                layers: 1,
                pNext: std::ptr::null(),
                flags: 0,
            };

            let mut framebuffer: VkFramebuffer = unsafe { std::mem::zeroed() };
            let result: VkResult = vk_create_framebuffer(
                self._get_device(),
                &framebuffer_info,
                std::ptr::null(),
                &mut framebuffer,
            );
            if result != VK_SUCCESS {
                panic!("Failed to create framebuffer!");
            }
            swapchain_framebuffers.push(framebuffer);
        }

        if debug_mode() {
            println!("Vulkan swapchain framebuffers created");
        }
        self.swapchain_framebuffers
            .set(swapchain_framebuffers)
            .expect("Failed to set swapchain framebuffers");
    }
    fn _create_command_pool(&self) {
        let queue_family_indices: QueueFamilyIndices =
            self._find_queue_families(&self._get_physical_device());

        let pool_info: VkCommandPoolCreateInfo = VkCommandPoolCreateInfo {
            sType: VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
            flags: VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT as u32,
            queueFamilyIndex: queue_family_indices.get_graphics_family(),
            pNext: std::ptr::null(),
        };
        let mut command_pool: VkCommandPool = unsafe { std::mem::zeroed() };
        let result: VkResult = vk_create_command_pool(
            self._get_device(),
            &pool_info,
            std::ptr::null(),
            &mut command_pool,
        );
        if result != VK_SUCCESS {
            panic!("Failed to create command pool!");
        }
        self.command_pool
            .set(command_pool)
            .expect("Command pool can not be initialized!");
        if debug_mode() {
            println!("Vulkan command pool created");
        }
    }
    fn _create_command_buffers(&self) {
        let alloc_info: VkCommandBufferAllocateInfo = VkCommandBufferAllocateInfo {
            sType: VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
            commandPool: self._get_command_pool(),
            level: VK_COMMAND_BUFFER_LEVEL_PRIMARY,
            commandBufferCount: 1,
            pNext: std::ptr::null(),
        };

        let mut command_buffer: VkCommandBuffer = unsafe { std::mem::zeroed() };
        let result: VkResult =
            vk_allocate_command_buffers(self._get_device(), &alloc_info, &mut command_buffer);
        if result != VK_SUCCESS {
            panic!("Failed to allocate command buffers!");
        }
        if debug_mode() {
            println!("Vulkan command buffer created");
        }
        self.command_buffer
            .set(command_buffer)
            .expect("Command buffer can not be initialized!");
    }
    fn _create_sync_objects(&self) {
        let semaphore_info: VkSemaphoreCreateInfo = VkSemaphoreCreateInfo {
            sType: VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO,
            pNext: std::ptr::null(),
            flags: 0,
        };

        let fence_info: VkFenceCreateInfo = VkFenceCreateInfo {
            sType: VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
            flags: VK_FENCE_CREATE_SIGNALED_BIT as u32,
            pNext: std::ptr::null(),
        };

        let mut image_available_semaphore: VkSemaphore = unsafe { std::mem::zeroed() };
        let mut render_finished_semaphore: VkSemaphore = unsafe { std::mem::zeroed() };
        let mut in_flight_fence: VkFence = unsafe { std::mem::zeroed() };

        let result_semaphore_image_available: VkResult = vk_create_semaphore(
            self._get_device(),
            &semaphore_info,
            std::ptr::null(),
            &mut image_available_semaphore,
        );
        let result_semaphore_render_finished: VkResult = vk_create_semaphore(
            self._get_device(),
            &semaphore_info,
            std::ptr::null(),
            &mut render_finished_semaphore,
        );
        let result_fence: VkResult = vk_create_fence(
            self._get_device(),
            &fence_info,
            std::ptr::null(),
            &mut in_flight_fence,
        );

        if result_semaphore_image_available != VK_SUCCESS
            || result_semaphore_render_finished != VK_SUCCESS
            || result_fence != VK_SUCCESS
        {
            panic!("Failed to create synchronization objects!");
        }
        self.image_available_semaphore
            .set(image_available_semaphore)
            .expect("Image available semaphore can not be initialized!");
        self.render_finished_semaphore
            .set(render_finished_semaphore)
            .expect("Render finished semaphore can not be initialized!");
        self.in_flight_fence
            .set(in_flight_fence)
            .expect("In flight fence can not be initialized!");
        if debug_mode() {
            println!("Vulkan synchronization objects created");
        }
    }

    fn _record_command_buffer(&self, command_buffer: VkCommandBuffer, image_index: u32) {
        let begin_info: VkCommandBufferBeginInfo = VkCommandBufferBeginInfo {
            sType: VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
            flags: 0,
            pInheritanceInfo: std::ptr::null(),
            pNext: std::ptr::null(),
        };

        let result: VkResult = vk_begin_command_buffer(command_buffer, &begin_info);
        if result != VK_SUCCESS {
            panic!("Failed to begin recording command buffer!");
        }

        let clear_color: VkClearValue = VkClearValue {
            color: VkClearColorValue {
                float32: [0.0, 0.0, 0.0, 1.0],
            },
        };
        let render_area = VkRect2D {
            offset: VkOffset2D { x: 0, y: 0 },
            extent: self._get_swapchain_extent(),
        };
        let render_pass_info: VkRenderPassBeginInfo = VkRenderPassBeginInfo {
            sType: VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
            renderPass: self._get_render_pass(),
            framebuffer: self._get_swapchain_framebuffers()[image_index as usize],
            renderArea: render_area,
            clearValueCount: 1,
            pClearValues: &clear_color,
            pNext: std::ptr::null(),
        };

        vk_cmd_begin_render_pass(
            command_buffer,
            &render_pass_info,
            VK_SUBPASS_CONTENTS_INLINE,
        );
        vk_cmd_bind_pipeline(
            command_buffer,
            VK_PIPELINE_BIND_POINT_GRAPHICS,
            self._get_graphics_pipeline(),
        );

        let viewport: VkViewport = VkViewport {
            x: 0.0,
            y: 0.0,
            width: self._get_swapchain_extent().width as c_float,
            height: self._get_swapchain_extent().height as c_float,
            minDepth: 0.0,
            maxDepth: 1.0,
        };
        vk_cmd_set_viewport(command_buffer, 0, 1, &viewport);

        let scissor: VkRect2D = VkRect2D {
            offset: VkOffset2D { x: 0, y: 0 },
            extent: self._get_swapchain_extent(),
        };
        vk_cmd_set_scissor(command_buffer, 0, 1, &scissor);
        vk_cmd_draw(command_buffer, 3, 1, 0, 0);
        vk_cmd_end_render_pass(command_buffer);

        let result: VkResult = vk_end_command_buffer(command_buffer);
        if result != VK_SUCCESS {
            panic!("Failed to record command buffer!");
        }
    }
}

impl GraphicApi for VulkanApi {
    fn init_window(&self) -> Window {
        if debug_mode() {
            println!("debug_mode is enabled");
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

        vk_destroy_semaphore(
            self._get_device(),
            self._get_render_finished_semaphore(),
            std::ptr::null(),
        );
        vk_destroy_semaphore(
            self._get_device(),
            self._get_image_available_semaphore(),
            std::ptr::null(),
        );
        vk_destroy_fence(
            self._get_device(),
            self._get_in_flight_fence(),
            std::ptr::null(),
        );

        vk_destroy_command_pool(
            self._get_device(),
            self._get_command_pool(),
            std::ptr::null(),
        );

        for swapchain_framebuffer in self._get_swapchain_framebuffers() {
            vk_destroy_framebuffer(self._get_device(), *swapchain_framebuffer, std::ptr::null());
        }

        vk_destroy_pipeline(
            self._get_device(),
            self._get_graphics_pipeline(),
            std::ptr::null(),
        );

        vk_destroy_pipeline_layout(
            self._get_device(),
            self._get_pipeline_layout(),
            std::ptr::null(),
        );

        vk_destroy_render_pass(
            self._get_device(),
            self._get_render_pass(),
            std::ptr::null(),
        );

        for swapchain_image_view in self._get_swapchain_image_views() {
            vk_destroy_image_view(self._get_device(), *swapchain_image_view, std::ptr::null());
        }

        vk_destroy_swapchain_khr(self._get_device(), self._get_swapchain(), std::ptr::null());

        vk_destroy_device(self._get_device(), std::ptr::null());

        vk_destroy_surface_khr(self._get_instance(), self._get_surface(), std::ptr::null());

        if self._enable_validation_layers() {
            self.destroy_debug_utils_messenger_ext(
                &self._get_instance(),
                &self._get_debug_messenger(),
                std::ptr::null(),
            );
        }

        vk_destroy_instance(self._get_instance(), std::ptr::null());
        glfw_destroy_window(self._get_window());
        glfw_terminate();
    }

    fn should_close(&self) -> bool {
        let should_close: i32 = glfw_window_should_close(self._get_window());

        should_close != 0
    }

    fn pool_events(&self) {
        glfw_poll_events();
    }

    fn wait_events(&self) {
        vk_wait_for_fences(
            self._get_device(),
            1,
            &self._get_in_flight_fence(),
            VK_TRUE,
            std::u64::MAX,
        );
        vk_reset_fences(self._get_device(), 1, &self._get_in_flight_fence());
    }

    fn draw_frame(&self) {
        let mut image_index: u32 = 0;

        vk_acquire_next_image_khr(
            self._get_device(),
            self._get_swapchain(),
            std::u64::MAX,
            self._get_image_available_semaphore(),
            std::ptr::null_mut(),
            &mut image_index,
        );

        vk_reset_command_buffer(self._get_command_buffer(), 0);
        self._record_command_buffer(self._get_command_buffer(), image_index);

        let wait_semaphores: Vec<VkSemaphore> = vec![self._get_image_available_semaphore()];
        let wait_stages: Vec<VkPipelineStageFlags> =
            vec![VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as u32];
        let signal_semaphores: Vec<VkSemaphore> = vec![self._get_render_finished_semaphore()];
        let submit_info = VkSubmitInfo {
            sType: VK_STRUCTURE_TYPE_SUBMIT_INFO,
            waitSemaphoreCount: 1,
            pWaitSemaphores: wait_semaphores.as_ptr(),
            pWaitDstStageMask: wait_stages.as_ptr(),
            commandBufferCount: 1,
            pCommandBuffers: &self._get_command_buffer(),
            signalSemaphoreCount: 1,
            pSignalSemaphores: signal_semaphores.as_ptr(),
            pNext: std::ptr::null(),
        };

        let result: VkResult = vk_queue_submit(
            self._get_graphics_queue(),
            1,
            &submit_info,
            self._get_in_flight_fence(),
        );
        if result != VK_SUCCESS {
            panic!("Failed to submit draw command buffer!");
        }

        let swapchains = vec![self._get_swapchain()];
        let preset_info = VkPresentInfoKHR {
            sType: VK_STRUCTURE_TYPE_PRESENT_INFO_KHR,
            waitSemaphoreCount: 1,
            pWaitSemaphores: signal_semaphores.as_ptr(),
            swapchainCount: 1,
            pSwapchains: swapchains.as_ptr(),
            pImageIndices: &image_index,
            pResults: std::ptr::null_mut(),
            pNext: std::ptr::null(),
        };

        vk_queue_present_khr(self._get_present_queue(), &preset_info);
    }

    fn wait_device_idle(&self) {
        vk_device_wait_idle(self._get_device());
    }
}
