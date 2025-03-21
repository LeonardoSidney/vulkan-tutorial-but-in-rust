#include <stdio.h>
#include <vulkan/vulkan.h>

VkResult CustomVkCreateInstance(
    const VkInstanceCreateInfo *pCreateInfo,
    const VkAllocationCallbacks *pAllocator,
    VkInstance *pInstance
) {
    printf("CustomVkCreateInstance\n");
    printf("pCreateInfo->sType: %d\n", pCreateInfo->sType);
    printf("pCreateInfo->pNext: %p\n", pCreateInfo->pNext);
    printf("pCreateInfo->flags: %d\n", pCreateInfo->flags);
    printf("pCreateInfo->pApplicationInfo: %p\n", pCreateInfo->pApplicationInfo);
    printf("pCreateInfo->enabledLayerCount: %d\n", pCreateInfo->enabledLayerCount);
    printf("pCreateInfo->ppEnabledLayerNames: %p\n", pCreateInfo->ppEnabledLayerNames);
    printf("pCreateInfo->enabledExtensionCount: %d\n", pCreateInfo->enabledExtensionCount);
    printf("pCreateInfo->ppEnabledExtensionNames: %p\n", pCreateInfo->ppEnabledExtensionNames);
    printf("pCreateInfo->enabledLayerCount: %d\n", pCreateInfo->enabledLayerCount);
    printf("pCreateInfo->ppEnabledLayerNames: %p\n", pCreateInfo->ppEnabledLayerNames);
    printf("pCreateInfo->enabledExtensionCount: %d\n", pCreateInfo->enabledExtensionCount);
    printf("pCreateInfo->ppEnabledExtensionNames: %p\n", pCreateInfo->ppEnabledExtensionNames);
    printf("pCreateInfo->enabledLayerCount: %d\n", pCreateInfo->enabledLayerCount);
    printf("pCreateInfo->ppEnabledLayerNames: %p\n", pCreateInfo->ppEnabledLayerNames);
    printf("pCreateInfo->enabledExtensionCount: %d\n", pCreateInfo->enabledExtensionCount);
    printf("pCreateInfo->ppEnabledExtensionNames: %p\n", pCreateInfo->ppEnabledExtensionNames);
    printf("pAllocator: %p\n", pAllocator);
    printf("pInstance: %p\n", pInstance);

    for (uint32_t i = 0; i < pCreateInfo->enabledExtensionCount; i++) {
        printf("pCreateInfo->ppEnabledExtensionNames[%d]: %s\n", i, pCreateInfo->ppEnabledExtensionNames[i]);
    }

    for (uint32_t i = 0; i < pCreateInfo->enabledLayerCount; i++) {
        printf("pCreateInfo->ppEnabledLayerNames[%d]: %s\n", i, pCreateInfo->ppEnabledLayerNames[i]);
    }

    VkResult result = vkCreateInstance(pCreateInfo, pAllocator, pInstance);
    printf("result: %d\n", result);
    return result;
}