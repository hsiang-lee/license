/*******************************************************************************
 *  FILENAME:      logger.h
 *
 *  AUTHORS:       Zheng Huijie    START DATE: Thursday November 19th 2020
 *
 *  LAST MODIFIED: Tuesday, December 1st 2020, 8:17:06 pm
 *
 *  CONTACT:       huijie.zheng@smartmore.com
*******************************************************************************/
#ifndef MIDDLEWARE_LOGGER_H
#define MIDDLEWARE_LOGGER_H

#include <grpcpp/grpcpp.h>

#include <chrono>

class LoggingInterceptor : public grpc::experimental::Interceptor {
   public:
    explicit LoggingInterceptor(grpc::experimental::ServerRpcInfo *info);
    void Intercept(
        grpc::experimental::InterceptorBatchMethods *methods) override;

   private:
    grpc::experimental::ServerRpcInfo *info_;
    std::chrono::high_resolution_clock::time_point start_;
    grpc::Status status_;
};

class LoggingInterceptorFactory
    : public grpc::experimental::ServerInterceptorFactoryInterface {
   public:
    virtual grpc::experimental::Interceptor *CreateServerInterceptor(
        grpc::experimental::ServerRpcInfo *info) override {
        return new LoggingInterceptor(info);
    }
};

#endif
