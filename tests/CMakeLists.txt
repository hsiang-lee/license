file(GLOB_RECURSE srcs *.cpp)

add_executable(tests ${srcs})

target_include_directories(tests PUBLIC
    ${Sodium_INCLUDE_DIRS}
    ${Json_INCLUDE_DIRS}
    ${CMAKE_CURRENT_SOURCE_DIR}/../src
)

target_link_libraries(tests
    ${Sodium_LIBRARIES}
    license
    gtest
    gtest_main
)
