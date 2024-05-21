install(
    TARGETS hello_world_cpp_exe
    RUNTIME COMPONENT hello_world_cpp_Runtime
)

if(PROJECT_IS_TOP_LEVEL)
  include(CPack)
endif()
