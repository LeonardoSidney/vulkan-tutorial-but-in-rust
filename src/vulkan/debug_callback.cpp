#include <iostream>
#include <vulkan/vulkan.h>
extern "C" {
VKAPI_ATTR VkBool32 VKAPI_CALL debugCallback(
    VkDebugUtilsMessageSeverityFlagBitsEXT messageSeverity,
    VkDebugUtilsMessageTypeFlagsEXT messageType,
    const VkDebugUtilsMessengerCallbackDataEXT *pCallbackData,
    void *pUserData
) {
    std::cerr << "validation layer: >> ";
    std::cerr << "messageSeverity: >> " << messageSeverity << "; ";
    std::cerr << "messageType: >> " << messageType << "; ";
    std::cerr << "message: >> " << pCallbackData->pMessage << "; ";
    std::cerr << "PUserData: >> " << pUserData << std::endl;

    return VK_FALSE;
};
}
