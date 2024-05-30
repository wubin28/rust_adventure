cmake_minimum_required(VERSION 3.14)

include(cmake/prelude.cmake)

project(
    dicey_temperatures_cpp
    VERSION 0.1.0
    DESCRIPTION "Short description"
    HOMEPAGE_URL "https://example.com/"
    LANGUAGES CXX
)

include(cmake/project-is-top-level.cmake)
include(cmake/variables.cmake)

# ---- Declare library ----

add_library(
    dicey_temperatures_cpp_lib OBJECT
    source/lib.cpp
)

target_include_directories(
    dicey_temperatures_cpp_lib ${warning_guard}
    PUBLIC
    "\$<BUILD_INTERFACE:${PROJECT_SOURCE_DIR}/source>"
)

target_compile_features(dicey_temperatures_cpp_lib PUBLIC cxx_std_20)

# ---- Declare executable ----

add_executable(dicey_temperatures_cpp_exe source/main.cpp)
add_executable(dicey_temperatures_cpp::exe ALIAS dicey_temperatures_cpp_exe)

set_property(TARGET dicey_temperatures_cpp_exe PROPERTY OUTPUT_NAME dicey_temperatures_cpp)

target_compile_features(dicey_temperatures_cpp_exe PRIVATE cxx_std_20)

target_link_libraries(dicey_temperatures_cpp_exe PRIVATE dicey_temperatures_cpp_lib)

# ---- Install rules ----

if(NOT CMAKE_SKIP_INSTALL_RULES)
  include(cmake/install-rules.cmake)
endif()

# ---- Developer mode ----

if(NOT dicey_temperatures_cpp_DEVELOPER_MODE)
  return()
elseif(NOT PROJECT_IS_TOP_LEVEL)
  message(
      AUTHOR_WARNING
      "Developer mode is intended for developers of dicey_temperatures_cpp"
  )
endif()

include(cmake/dev-mode.cmake)
