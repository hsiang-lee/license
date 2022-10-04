cmake_minimum_required(VERSION 3.20)

option(BUILD_GENERATOR "whether to build license generator" ON)
option(BUILD_TESTS "whether to build tests" ON)
option(BUILD_SERVICE "whether to build license service" OFF)

if (WIN32)
    set (CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/../)
    set (CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/../)
    set (CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/../)
else()
    add_compile_options(-std=c++11) 
endif()

set( CMAKE_CXX_FLAGS  "${CMAKE_CXX_FLAGS} -fPIC" )

project(license)

list(APPEND CMAKE_MODULE_PATH ${CMAKE_CURRENT_SOURCE_DIR}/cmake)

if (NOT TARGET nlohmann_json)
    add_subdirectory(thirdparty/json)
endif()

add_subdirectory(src)

if (BUILD_GENERATOR)
    add_subdirectory(app)
endif()

if (BUILD_TESTS)
    add_subdirectory(tests)
endif() 

if (BUILD_SERVICE)
    find_package(Threads REQUIRED)
    set(GRPC_AS_PACKAGE true)
    if(GRPC_AS_PACKAGE)
        set(protobuf_MODULE_COMPATIBLE TRUE)
        find_package(Protobuf CONFIG REQUIRED)
        message(STATUS "Using protobuf ${Protobuf_VERSION}")

        set(_PROTOBUF_LIBPROTOBUF protobuf::libprotobuf)
        set(_REFLECTION gRPC::grpc++_reflection)
        if(CMAKE_CROSSCOMPILING)
            find_program(_PROTOBUF_PROTOC protoc)
        else()
            set(_PROTOBUF_PROTOC $<TARGET_FILE:protobuf::protoc>)
        endif()

        find_package(gRPC CONFIG REQUIRED)
        message(STATUS "Using gRPC ${gRPC_VERSION}")

        set(_GRPC_GRPCPP gRPC::grpc++)
        if(CMAKE_CROSSCOMPILING)
            find_program(_GRPC_CPP_PLUGIN_EXECUTABLE grpc_cpp_plugin)
        else()
            set(_GRPC_CPP_PLUGIN_EXECUTABLE $<TARGET_FILE:gRPC::grpc_cpp_plugin>)
        endif()
    endif()
    add_subdirectory(${CMAKE_CURRENT_SOURCE_DIR}/thirdparty/cpp-cmakefiles)

    add_subdirectory(service)
endif()
