# Parent project does not export its library target, so this CML implicitly
# depends on being added from it, i.e. the testing is done only from the build
# tree and is not feasible from an install location

project(hello_world_cppTests LANGUAGES CXX)

# ---- Tests ----

add_executable(hello_world_cpp_test source/hello_world_cpp_test.cpp)
target_link_libraries(hello_world_cpp_test PRIVATE hello_world_cpp_lib)
target_compile_features(hello_world_cpp_test PRIVATE cxx_std_17)

add_test(NAME hello_world_cpp_test COMMAND hello_world_cpp_test)

# ---- End-of-file commands ----

add_folders(Test)
