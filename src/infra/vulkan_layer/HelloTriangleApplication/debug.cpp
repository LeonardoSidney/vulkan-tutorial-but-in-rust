#include "HelloTriangle.hpp"

int main() {
    HelloTriangleApplication app(800, 600);

    try {
        app.run();
    } catch (const std::exception &e) {
        std::cerr << e.what() << '\n';
        return EXIT_FAILURE;
    }

    return EXIT_SUCCESS;
}