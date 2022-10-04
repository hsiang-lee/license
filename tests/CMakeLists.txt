file(GLOB_RECURSE srcs *.cpp)

add_executable(tests ${srcs})

set_target_properties(tests
    PROPERTIES
    CXX_CLANG_TIDY "clang-tidy")

target_include_directories(tests PUBLIC
    ${Catch2_INCLUDE_DIRS}
    ${Sodium_INCLUDE_DIRS}
    ${Json_INCLUDE_DIRS}
    ${CMAKE_CURRENT_SOURCE_DIR}/../src
)

target_link_libraries(tests
    ${Sodium_LIBRARIES}
    smlicense
)
