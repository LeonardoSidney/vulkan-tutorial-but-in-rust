# Makefile for C++ project

# Variables
CXX := g++
CXXFLAGS := -Wall -Wextra -std=c++17 -g
LDFLAGS := -lglfw -lvulkan -ldl -lpthread -lX11 -lXxf86vm -lXrandr -lXi
SRC_DIR := src
BUILD_DIR := build
DEBUG_DIR := debug
TARGET := $(BUILD_DIR)/main

clean:
	rm -rf $(BUILD_DIR)
	rm -rf $(DEBUG_DIR)

debug:
	rm -rf $(DEBUG_DIR)
	mkdir -p $(DEBUG_DIR)
	$(CXX) $(CXXFLAGS) $(FILENAME) -o $(DEBUG_DIR)/$(basename $(notdir $(FILENAME))).out $(LDFLAGS) -DDEBUG

.PHONY: all clean debug