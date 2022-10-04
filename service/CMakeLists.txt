set(SERVICES "smlicense")
set(SERVICES_NAME "smlicense-service")

# Build Proto files
set(proto_path ${CMAKE_CURRENT_SOURCE_DIR}/proto/)
set(proto_output_path ${CMAKE_CURRENT_BINARY_DIR}/proto/)
set (protofilelist)
foreach(svc ${SERVICES})
    set (outputfolder "${proto_output_path}/${svc}")
    file(MAKE_DIRECTORY ${outputfolder})
    add_custom_command(
        OUTPUT "${outputfolder}/${svc}.pb.cc"
        "${outputfolder}/${svc}.pb.h"
        "${outputfolder}/${svc}.grpc.pb.cc"
        "${outputfolder}/${svc}.grpc.pb.h"
        COMMAND ${_PROTOBUF_PROTOC}
        ARGS --grpc_out "${outputfolder}/"
        --cpp_out "${outputfolder}/"
        -I "${proto_path}/${svc}/"
        -I "${CMAKE_CURRENT_SOURCE_DIR}/../thirdparty/"
        --plugin=protoc-gen-grpc="${_GRPC_CPP_PLUGIN_EXECUTABLE}"
        "${proto_path}/${svc}/${svc}.proto"
        DEPENDS "${proto_path}/${svc}/${svc}.proto"
    )
    list(APPEND protofilelist
        "${outputfolder}/${svc}.pb.cc"
        "${outputfolder}/${svc}.pb.h"
        "${outputfolder}/${svc}.grpc.pb.cc"
        "${outputfolder}/${svc}.grpc.pb.h"
    )
endforeach(svc)

add_library(protos)

target_sources(protos PRIVATE
    ${protofilelist})

target_include_directories(protos PUBLIC
    ${proto_output_path})

target_link_libraries(protos PUBLIC
    googleapis-c++::api_annotations_protos
    ${_PROTOBUF_LIBPROTOBUF}
    ${_REFLECTION}
    ${_GRPC_GRPCPP})

add_library(yaml)

target_sources(yaml PRIVATE
    "${CMAKE_CURRENT_LIST_DIR}/src/util/yaml/Yaml.cpp")

target_include_directories(yaml 
    PUBLIC
    ${CMAKE_CURRENT_LIST_DIR}/src/util/yaml)

add_library(util)

target_sources(util PRIVATE
    "./src/util/config.cpp"
    "./src/util/c_config.cpp")

target_include_directories(util
    PUBLIC
    ${CMAKE_CURRENT_LIST_DIR}/src/util)

target_link_libraries(util PUBLIC yaml)

set(CPPSRCS
    "./src/smlicense_server.cpp"
    "./src/smlicense/smlicensegrpc.cpp"
    "./src/middleware/logger.cpp")

add_executable(${SERVICES_NAME} ${CPPSRCS})

set_target_properties(${SERVICES_NAME} PROPERTIES
    CXX_CLANG_TIDY "clang-tidy")

target_include_directories(${SERVICES_NAME} 
    PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/../include
    PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/smlicense
    ${CMAKE_CURRENT_SOURCE_DIR}/src/middleware
    ${CMAKE_CURRENT_SOURCE_DIR}/src/util
)

target_link_libraries(${SERVICES_NAME}
    protos
    util
    Sodium::Sodium
    smlicense
)

# target to build grpcgateway
get_filename_component(gateway_path "./gateway/" ABSOLUTE)
set (grpcgateway_filelist)
foreach(svc ${SERVICES})
    add_custom_command(
        OUTPUT
        "${gateway_path}/${svc}/${svc}.pb.gw.go"
        "${gateway_path}/${svc}/${svc}.pb.go"
        COMMAND mkdir -p "${gateway_path}/${svc}/"
        COMMAND ${_PROTOBUF_PROTOC}
        ARGS -I.
        --gofast_out=plugins=grpc:"${gateway_path}/${svc}/"
        --grpc-gateway_out=logtostderr=true:"${gateway_path}/${svc}/"
        -I "${proto_path}/${svc}/"
        -I "${CMAKE_CURRENT_SOURCE_DIR}/../thirdparty/"
        --plugin=protoc-gen-grpc="${_GRPC_CPP_PLUGIN_EXECUTABLE}"
        "${proto_path}/${svc}/${svc}.proto"
        DEPENDS "${proto_path}/${svc}/${svc}.proto"
    )
    list(APPEND grpcgateway_filelist 
    "${gateway_path}/${svc}/${svc}.pb.gw.go"
    "${gateway_path}/${svc}/${svc}.pb.go")
endforeach(svc)

set_target_properties(util
    PROPERTIES
    ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/gateway/lib"
    CXX_CLANG_TIDY "clang-tidy"
)

set_target_properties(yaml
    PROPERTIES
    ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/gateway/lib")

add_custom_command(
    OUTPUT gateway
    COMMAND go
    ARGS build
    -o gateway
    "${gateway_path}/httpserver.go"
    DEPENDS "${gateway_path}/httpserver.go" util ${grpcgateway_filelist}
)

add_custom_target(grpcgateway ALL
    DEPENDS ${grpcgateway_filelist} gateway
)
