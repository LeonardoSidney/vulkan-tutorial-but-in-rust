use std::env;
use std::process::Command;

fn main() {
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

    println!("cargo:rereun-if-changed=src/infra/vulkan_layer/HelloTriangleApplication/HelloTriangle.cpp");
    println!("cargo:rerun-if-changed=src/infra/vulkan_layer/HelloTriangleApplication/HelloTriangle.hpp");
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
    let output = build_hello_shaders.output().expect("Failed to compile fragment shader");

    if !output.status.success() {
        panic!("Shader compilation failed: {:?}", output);
    }

    build_hello_shaders = Command::new("glslc");
    build_hello_shaders
        .arg("src/infra/vulkan_layer/HelloTriangleApplication/shaders/shader.vert")
        .arg("-o")
        .arg("src/infra/vulkan_layer/HelloTriangleApplication/shaders/shader.vert.spv");
    let output = build_hello_shaders.output().expect("Failed to compile vertex shader");

    if !output.status.success() {
        panic!("Shader compilation failed: {:?}", output);
    }

}
