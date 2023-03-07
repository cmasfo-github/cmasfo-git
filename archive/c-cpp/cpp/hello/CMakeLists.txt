
cmake_minimum_required(VERSION 3.25)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

project(hello CXX)

add_compile_options(-fmodules-ts)

# modules must come first before being used
add_executable(${PROJECT_NAME}
  src/hello.cpp
  src/cli.cpp
)
