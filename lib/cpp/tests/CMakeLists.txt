# Add an executable for testing
set (TEST_APP test_${PROJECT})

add_executable(test_aabox2d test_aabox2d.cpp)
target_link_libraries(test_aabox2d PRIVATE ${${LIBRARY}})

add_executable(test_dvector3 test_dvector3.cpp)
target_link_libraries(test_dvector3 PRIVATE ${${LIBRARY}})

add_executable(test_line2d test_line2d.cpp)
target_link_libraries(test_line2d PRIVATE ${${LIBRARY}})

add_executable(test_plane test_plane.cpp)
target_link_libraries(test_plane PRIVATE ${${LIBRARY}})

add_executable(test_quaternion test_quaternion.cpp)
target_link_libraries(test_quaternion PRIVATE ${${LIBRARY}})

add_executable(test_ray test_ray.cpp)
target_link_libraries(test_ray PRIVATE ${${LIBRARY}})

add_executable(test_vector2 test_vector2.cpp)
target_link_libraries(test_vector2 PRIVATE ${${LIBRARY}})

add_executable(test_vector3 test_vector3.cpp)
target_link_libraries(test_vector3 PRIVATE ${${LIBRARY}})

add_executable(test_vector4 test_vector4.cpp)
target_link_libraries(test_vector4 PRIVATE ${${LIBRARY}})

add_executable(test_matrix4x4 test_matrix4x4.cpp)
target_link_libraries(test_matrix4x4 PRIVATE ${${LIBRARY}})


# Add the test using CTest
add_test(NAME AABox2DTests COMMAND test_aabox2d)
add_test(NAME DVector3Tests COMMAND test_dvector3)
add_test(NAME Line2DTests COMMAND test_line2d)
add_test(NAME PlaneTests COMMAND test_plane)
add_test(NAME QuaternionTests COMMAND test_quaternion)
add_test(NAME RayTests COMMAND test_ray)
add_test(NAME Vector2Tests COMMAND test_vector2)
add_test(NAME Vector3Tests COMMAND test_vector3)
add_test(NAME Vector4Tests COMMAND test_vector4)
add_test(NAME Matrix4x4Tests COMMAND test_matrix4x4)
