cmake_minimum_required(VERSION 3.14)

include(cmake/prelude.cmake)

project(
    type_inference_cpp
    VERSION 0.1.0
    DESCRIPTION "Short description"
    HOMEPAGE_URL "https://example.com/"
    LANGUAGES CXX
)

include(cmake/project-is-top-level.cmake)
include(cmake/variables.cmake)

# ---- Declare library ----

add_library(
    type_inference_cpp_lib OBJECT
    source/lib.cpp
)

target_include_directories(
    type_inference_cpp_lib ${warning_guard}
    PUBLIC
    "\$<BUILD_INTERFACE:${PROJECT_SOURCE_DIR}/source>"
)

target_compile_features(type_inference_cpp_lib PUBLIC cxx_std_20)

# ---- Declare executable ----

add_executable(type_inference_cpp_exe source/main.cpp)
add_executable(type_inference_cpp::exe ALIAS type_inference_cpp_exe)

set_property(TARGET type_inference_cpp_exe PROPERTY OUTPUT_NAME type_inference_cpp)

target_compile_features(type_inference_cpp_exe PRIVATE cxx_std_20)

target_link_libraries(type_inference_cpp_exe PRIVATE type_inference_cpp_lib)

# ---- Install rules ----

if(NOT CMAKE_SKIP_INSTALL_RULES)
  include(cmake/install-rules.cmake)
endif()

# ---- Developer mode ----

if(NOT type_inference_cpp_DEVELOPER_MODE)
  return()
elseif(NOT PROJECT_IS_TOP_LEVEL)
  message(
      AUTHOR_WARNING
      "Developer mode is intended for developers of type_inference_cpp"
  )
endif()

include(cmake/dev-mode.cmake)
