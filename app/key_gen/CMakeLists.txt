file(GLOB_RECURSE srcs *.cpp)

add_executable(key_gen ${srcs})

set_target_properties(key_gen
    PROPERTIES
    CXX_CLANG_TIDY "clang-tidy")

target_include_directories(key_gen PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/../../include/
)

target_link_libraries(key_gen
    license
)
