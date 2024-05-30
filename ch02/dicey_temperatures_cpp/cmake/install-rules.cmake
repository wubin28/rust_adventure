install(
    TARGETS dicey_temperatures_cpp_exe
    RUNTIME COMPONENT dicey_temperatures_cpp_Runtime
)

if(PROJECT_IS_TOP_LEVEL)
  include(CPack)
endif()
