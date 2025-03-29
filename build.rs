extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let glfw_include_dir: &str = "/usr/include/GLFW";
    let glfw_header_file: String = format!("{}/glfw3.h", glfw_include_dir);
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

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

    let vulkan_include_dir: &str = "/usr/include/vulkan";
    let vulkan_header_file: String = format!("{}/vulkan.h", vulkan_include_dir);
    let vulkan_core_header_file: String = format!("{}/vulkan_core.h", vulkan_include_dir);

    let bindings_vulkan = bindgen::Builder::default()
        .header(vulkan_header_file)
        .header(vulkan_core_header_file)
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
        .default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: true })
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate vulkan bindings");

    bindings_vulkan
        .write_to_file(out_path.join("bindings_vulkan.rs"))
        .expect("Couldn't write vulkan bindings!");

    let profile: String = env::var("PROFILE").unwrap();

    let mut build_hello_triangle = cc::Build::new();
    build_hello_triangle
        .file("src/infra/vulkan_layer/HelloTriangleApplication/HelloTriangle.cpp")
        .cpp(true)
        .flag("-std=c++17")
        .flag("-O3");
    if profile == "debug" {
        build_hello_triangle.flag("-DDEBUG");
        build_hello_triangle.flag("-g");
        build_hello_triangle.flag("-shared");
        build_hello_triangle.flag("-fPIC");
    }
    build_hello_triangle.compile("HelloTriangleApplication");

    println!(
        "cargo:rereun-if-changed=src/infra/vulkan_layer/HelloTriangleApplication/HelloTriangle.cpp"
    );
    println!(
        "cargo:rerun-if-changed=src/infra/vulkan_layer/HelloTriangleApplication/HelloTriangle.hpp"
    );
    println!("cargo:rustc-link-lib=glfw");
    println!("cargo:rustc-link-lib=vulkan");
    println!("cargo:rustc-link-lib=dl");
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=X11");
    println!("cargo:rustc-link-lib=Xxf86vm");
    println!("cargo:rustc-link-lib=Xrandr");
    println!("cargo:rustc-link-lib=Xi");

    let mut build_hello_shaders = Command::new("glslc");
    build_hello_shaders
        .arg("src/infra/vulkan_layer/HelloTriangleApplication/shaders/shader.frag")
        .arg("-o")
        .arg("src/infra/vulkan_layer/HelloTriangleApplication/shaders/shader.frag.spv");
    let output = build_hello_shaders
        .output()
        .expect("Failed to compile fragment shader");

    if !output.status.success() {
        panic!("Shader compilation failed: {:?}", output);
    }

    build_hello_shaders = Command::new("glslc");
    build_hello_shaders
        .arg("src/infra/vulkan_layer/HelloTriangleApplication/shaders/shader.vert")
        .arg("-o")
        .arg("src/infra/vulkan_layer/HelloTriangleApplication/shaders/shader.vert.spv");
    let output = build_hello_shaders
        .output()
        .expect("Failed to compile vertex shader");

    if !output.status.success() {
        panic!("Shader compilation failed: {:?}", output);
    }

    let mut build_custom_vk_create_instance = cc::Build::new();
    build_custom_vk_create_instance
        .file("src/vulkan/vk_create_instance.c")
        .flag("-shared")
        .flag("-fPIC")
        .flag("-O3")
        .flag("-g")
        .compile("custom_vk_create_instance");  

    println!("cargo:rerun-if-changed=src/vulkan/vk_create_instance.c");
    println!("cargo:rustc-link-lib=vulkan");

    let mut build_debug_callback = cc::Build::new();
    build_debug_callback
        .cpp(true)
        .file("src/vulkan/debug_callback.cpp")
        .flag("-shared")
        .flag("-fPIC")
        .flag("-O3")
        .flag("-g")
        .compile("debug_callback");

    println!("cargo:rerun-if-changed=src/vulkan/debug_callback.cpp");
    println!("cargo:rustc-link-lib=vulkan");

    let mut build_populate_debug_messenger_create_info = cc::Build::new();
    build_populate_debug_messenger_create_info
        .cpp(true)
        .file("src/vulkan/populate_debug_message_create_info.cpp")
        .flag("-shared")
        .flag("-fPIC")
        .flag("-O3")
        .flag("-g")
        .compile("populate_debug_messenger_create_info");

    println!("cargo:rerun-if-changed=src/vulkan/populate_debug_message_create_info.cpp");
    println!("cargo:rustc-link-lib=vulkan");


}
