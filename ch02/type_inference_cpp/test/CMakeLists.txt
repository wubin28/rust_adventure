# Parent project does not export its library target, so this CML implicitly
# depends on being added from it, i.e. the testing is done only from the build
# tree and is not feasible from an install location

project(type_inference_cppTests LANGUAGES CXX)

# ---- Tests ----

add_executable(type_inference_cpp_test source/type_inference_cpp_test.cpp)
target_link_libraries(type_inference_cpp_test PRIVATE type_inference_cpp_lib)
target_compile_features(type_inference_cpp_test PRIVATE cxx_std_20)

add_test(NAME type_inference_cpp_test COMMAND type_inference_cpp_test)

# ---- End-of-file commands ----

add_folders(Test)
