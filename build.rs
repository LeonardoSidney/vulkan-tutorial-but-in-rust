extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let glfw_include_dir: &str = "/usr/include/GLFW";
    let glfw_header_file: String = format!("{}/glfw3.h", glfw_include_dir);
    let bindings_glfw = bindgen::Builder::default()
        .header(glfw_header_file)
        .allowlist_var("GLFW_CLIENT_API")
        .allowlist_var("GLFW_NO_API")
        .allowlist_var("GLFW_RESIZABLE")
        .allowlist_var("GLFW_FALSE")
        .allowlist_type("GLFWwindow")
        .allowlist_type("GLFWmonitor")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate glfw bindings");
    bindings_glfw
        .write_to_file(out_path.join("bindings_glfw.rs"))
        .expect("Couldn't write glfw bindings!");
    println!("cargo:rustc-link-lib=glfw");

    let vulkan_include_dir: &str = "/usr/include/vulkan";
    let vulkan_header_file: String = format!("{}/vulkan.h", vulkan_include_dir);

    let bindings_vulkan = bindgen::Builder::default()
        .header(vulkan_header_file)
        .allowlist_var("VK_KHR_SWAPCHAIN_EXTENSION_NAME")
        .allowlist_type("VkLayerProperties")
        .allowlist_type("VkResult")
        .allowlist_type("VkApplicationInfo")
        .allowlist_type("VkInstanceCreateInfo")
        .allowlist_type("VkExtensionProperties")
        .allowlist_type("VkDebugUtilsMessengerCreateInfoEXT")
        .allowlist_type("VkInstance")
        .allowlist_type("VkAllocationCallbacks")
        .allowlist_type("VkDebugUtilsMessengerEXT")
        .allowlist_type("PFN_vkVoidFunction")
        .allowlist_item("VkStructureType")
        .allowlist_item("VK_EXT_DEBUG_UTILS_EXTENSION_NAME")
        .allowlist_item("VkDebugUtilsMessageSeverityFlagBitsEXT")
        .allowlist_item("VkDebugUtilsMessageTypeFlagBitsEXT")
        .allowlist_item("VkDebugUtilsMessageTypeFlagsEXT")
        .allowlist_item("PFN_vkCreateDebugUtilsMessengerEXT")
        .allowlist_item("PFN_vkDestroyDebugUtilsMessengerEXT")
        .allowlist_item("VkDebugUtilsMessengerCallbackDataEXT")
        .allowlist_item("VkBool32")
        .allowlist_item("VK_FALSE")
        .allowlist_item("VkSurfaceKHR")
        .allowlist_item("VkPhysicalDevice")
        .allowlist_item("PFN_vkDebugUtilsMessengerCallbackEXT")
        .allowlist_item("VkPhysicalDeviceProperties")
        .allowlist_item("VkPhysicalDeviceFeatures")
        .allowlist_item("VkQueueFamilyProperties")
        .allowlist_item("VkQueueFlagBits")
        .allowlist_item("VkSurfaceCapabilitiesKHR")
        .allowlist_item("VkSurfaceFormatKHR")
        .allowlist_item("VkPresentModeKHR")
        .allowlist_item("VkDeviceQueueCreateInfo")
        .allowlist_item("VkDeviceCreateInfo")
        .allowlist_item("VkDevice")
        .allowlist_item("VkQueue")
        .allowlist_item("VkFormat")
        .allowlist_item("VkColorSpaceKHR")
        .allowlist_item("VkExtent2D")
        .allowlist_item("VkSwapchainCreateInfoKHR")
        .allowlist_item("VkImageUsageFlagBits")
        .allowlist_item("VkSharingMode")
        .allowlist_item("VkCompositeAlphaFlagBitsKHR")
        .allowlist_item("VK_TRUE")
        .allowlist_item("VK_NULL_HANDLE")
        .allowlist_item("VkSwapchainKHR")
        .allowlist_item("VkImage")
        .allowlist_item("VkImageView")
        .allowlist_item("VkImageViewCreateInfo")
        .allowlist_item("VkImageViewType")
        .allowlist_item("VkComponentMapping")
        .allowlist_item("VkComponentSwizzle")
        .allowlist_item("VkImageSubresourceRange")
        .allowlist_item("VkImageAspectFlagBits")
        .allowlist_item("VkAttachmentDescription")
        .allowlist_item("VkSampleCountFlagBits")
        .allowlist_item("VkAttachmentLoadOp")
        .allowlist_item("VkAttachmentStoreOp")
        .allowlist_item("VkImageLayout")
        .allowlist_item("VkPipelineBindPoint")
        .allowlist_item("VkPipelineStageFlagBits")
        .allowlist_item("VkAccessFlagBits")
        .allowlist_item("VK_SUBPASS_EXTERNAL")
        .allowlist_item("VkAttachmentReference")
        .allowlist_item("VkSubpassDescription")
        .allowlist_item("VkSubpassDependency")
        .allowlist_item("VkRenderPassCreateInfo")
        .allowlist_item("VkRenderPass")
        .allowlist_item("VkShaderModule")
        .allowlist_item("VkShaderModuleCreateInfo")
        .allowlist_item("VkPipelineShaderStageCreateInfo")
        .allowlist_item("VkShaderStageFlagBits")
        .allowlist_item("VkPipelineVertexInputStateCreateInfo")
        .allowlist_item("VkPrimitiveTopology")
        .allowlist_item("VkPipelineInputAssemblyStateCreateInfo")
        .allowlist_item("VkViewport")
        .allowlist_item("VkRect2D")
        .allowlist_item("VkOffset2D")
        .allowlist_item("VkPipelineViewportStateCreateInfo")
        .allowlist_item("VkPipelineRasterizationStateCreateInfo")
        .allowlist_item("VkPolygonMode")
        .allowlist_item("VkCullModeFlagBits")
        .allowlist_item("VkFrontFace")
        .allowlist_item("VkPipelineMultisampleStateCreateInfo")
        .allowlist_item("VkPipelineColorBlendAttachmentState")
        .allowlist_item("VkColorComponentFlagBits")
        .allowlist_item("VkBlendFactor")
        .allowlist_item("VkBlendOp")
        .allowlist_item("VkPipelineColorBlendStateCreateInfo")
        .allowlist_item("VkLogicOp")
        .allowlist_item("VkDynamicState")
        .allowlist_item("VkPipelineDynamicStateCreateInfo")
        .allowlist_item("VkPipelineLayoutCreateInfo")
        .allowlist_item("VkPipelineLayout")
        .allowlist_item("VkGraphicsPipelineCreateInfo")
        .allowlist_item("VkPipeline")
        .allowlist_item("VkPipelineCache")
        .allowlist_item("VkFramebufferCreateInfo")
        .allowlist_item("VkFramebuffer")
        .allowlist_item("VkCommandPoolCreateInfo")
        .allowlist_item("VkCommandPool")
        .allowlist_item("VkCommandBufferAllocateInfo")
        .allowlist_item("VkCommandBuffer")
        .allowlist_item("VkCommandBufferLevel")
        .allowlist_item("VkSemaphoreCreateInfo")
        .allowlist_item("VkFenceCreateInfo")
        .allowlist_item("VkFenceCreateFlagBits")
        .allowlist_item("VkSemaphore")
        .allowlist_item("VkFence")
        .allowlist_item("VkCommandBufferResetFlags")
        .allowlist_item("VkCommandBufferBeginInfo")
        .allowlist_item("VkClearValue")
        .allowlist_item("VkRenderPassBeginInfo")
        .allowlist_item("VkSubpassContents")
        .allowlist_item("VkViewport")
        .allowlist_item("VkSubmitInfo")
        .allowlist_item("VkPipelineStageFlags")
        .allowlist_item("VkPresentInfoKHR")
        .allowlist_item("VkCommandPoolCreateFlagBits")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: true,
        })
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate vulkan bindings");
    bindings_vulkan
        .write_to_file(out_path.join("bindings_vulkan.rs"))
        .expect("Couldn't write vulkan bindings!");
    println!("cargo:rustc-link-lib=vulkan");

    let mut build_hello_shaders = Command::new("glslc");
    build_hello_shaders
        .arg("src/shaders/shader.frag")
        .arg("-o")
        .arg("src/shaders/shader.frag.spv");
    let output = build_hello_shaders
        .output()
        .expect("Failed to compile fragment shader");

    if !output.status.success() {
        panic!("Shader compilation failed: {:?}", output);
    }

    build_hello_shaders = Command::new("glslc");
    build_hello_shaders
        .arg("src/shaders/shader.vert")
        .arg("-o")
        .arg("src/shaders/shader.vert.spv");
    let output = build_hello_shaders
        .output()
        .expect("Failed to compile vertex shader");

    if !output.status.success() {
        panic!("Shader compilation failed: {:?}", output);
    }
}
