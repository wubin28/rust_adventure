# Parent project does not export its library target, so this CML implicitly
# depends on being added from it, i.e. the testing is done only from the build
# tree and is not feasible from an install location

project(dicey_temperatures_cppTests LANGUAGES CXX)

# ---- Tests ----

add_executable(dicey_temperatures_cpp_test source/dicey_temperatures_cpp_test.cpp)
target_link_libraries(dicey_temperatures_cpp_test PRIVATE dicey_temperatures_cpp_lib)
target_compile_features(dicey_temperatures_cpp_test PRIVATE cxx_std_20)

add_test(NAME dicey_temperatures_cpp_test COMMAND dicey_temperatures_cpp_test)

# ---- End-of-file commands ----

add_folders(Test)
