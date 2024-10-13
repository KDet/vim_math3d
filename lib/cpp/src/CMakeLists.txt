# Create a library from the source files
add_library(${LIBRARY} vim_math3d.cpp)

# Include the header file directory
target_include_directories(${LIBRARY} PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
