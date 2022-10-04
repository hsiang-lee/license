/*******************************************************************************
 *  FILENAME:      smlicense_server.cpp
 *
 *  AUTHORS:       Zheng Huijie    START DATE: Thursday December 3rd 2020
 *
 *  LAST MODIFIED: Friday, December 4th 2020, 12:51:36 pm
 *
 *  CONTACT:       huijie.zheng@smartmore.com
*******************************************************************************/
#include <grpcpp/ext/proto_server_reflection_plugin.h>
#include <grpcpp/grpcpp.h>
#include <grpcpp/health_check_service_interface.h>

#include <iostream>
#include <memory>
#include <string>

#include "smlicense/smlicensegrpc.h"
#include "middleware/logger.h"

#include "util/config.h"

using grpc::Server;
using grpc::ServerBuilder;
using grpc::ServerContext;
using grpc::Status;

const static int kMaxMessageSize = 1024 * 1024 * 1024;

void RunServer() {
    std::string port = GetGRpcPort();
    std::string server_address("0.0.0.0:" + port);

    SmlicenseServiceImpl smlicense_service;

    grpc::EnableDefaultHealthCheckService(true);
    grpc::reflection::InitProtoReflectionServerBuilderPlugin();
    ServerBuilder builder;
    builder.SetMaxReceiveMessageSize(kMaxMessageSize);
    // Listen on the given address without any authentication mechanism.
    builder.AddListeningPort(server_address, grpc::InsecureServerCredentials());

    // Register "service" as the instance through which we'll communicate with
    // clients. In this case it corresponds to an *synchronous* service.
    builder.RegisterService(&smlicense_service);

    std::vector<
        std::unique_ptr<grpc::experimental::ServerInterceptorFactoryInterface>>
        creators;
    creators.push_back(
        std::unique_ptr<grpc::experimental::ServerInterceptorFactoryInterface>(
            new LoggingInterceptorFactory()));

    builder.experimental().SetInterceptorCreators(std::move(creators));

    // Finally assemble the server.
    std::unique_ptr<Server> server(builder.BuildAndStart());
    printf("Server listening on: %s\n", server_address.c_str());
    // Wait for the server to shutdown. Note that some other thread must be
    // responsible for shutting down the server for this call to ever return.

    server->Wait();
}

int main(int argc, char **argv) {
    RunServer();

    return 0;
}
