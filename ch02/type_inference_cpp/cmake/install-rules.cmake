install(
    TARGETS type_inference_cpp_exe
    RUNTIME COMPONENT type_inference_cpp_Runtime
)

if(PROJECT_IS_TOP_LEVEL)
  include(CPack)
endif()
