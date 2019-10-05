# STM32CubeIDE embedded C++ Development

STM32CubeIDE (Eclipse CDT) is perfectly capable of C++ development, and ``g++`` comes installed with it. However, the ST HAL is in C, and CubeMX generates sources that only support firmware development in C. Or, do they? In this article, I will show how to configure the IDE to handle C++ and how to use the HAL in a way that supports RAII and OOP.

* Create your new project, choosing CPP (instead of the default, C) in the wizard.
