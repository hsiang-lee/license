file(GLOB_RECURSE srcs *.cpp)

add_executable(license_check ${srcs})

set_target_properties(license_check
    PROPERTIES
    CXX_CLANG_TIDY "clang-tidy")

target_include_directories(license_check PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/../../include/
)

target_link_libraries(license_check
    license
)
