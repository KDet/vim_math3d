cmake_minimum_required(VERSION 3.10)

set (PROJECT "vim_math3d")
set (LIBRARY ${PROJECT}_lib)

project (${PROJECT} VERSION 1.6.1)

# Set C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Add the library
add_subdirectory(lib)

# Add tests
enable_testing()
add_subdirectory(tests)
