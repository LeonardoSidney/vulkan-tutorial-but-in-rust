#include "HelloTriangle.hpp"

int main_hello_triange(uint32_t width, uint32_t height) {
    HelloTriangleApplication app(width, height);

    try {
        app.run();
    } catch (const std::exception &e) {
        std::cerr << e.what() << '\n';
        return EXIT_FAILURE;
    }

    return EXIT_SUCCESS;
}

extern "C" int hello_triange(uint32_t width, uint32_t height) {
    return main_hello_triange(width, height);
}