file(GLOB_RECURSE srcs *.cpp)

add_executable(machine_id ${srcs})

set_target_properties(machine_id
    PROPERTIES
    CXX_CLANG_TIDY "clang-tidy")

target_include_directories(machine_id PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/../../include/
)

target_link_libraries(machine_id
    license
)
