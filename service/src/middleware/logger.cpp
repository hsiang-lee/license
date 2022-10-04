/*******************************************************************************
 *  FILENAME:      logger.cpp
 *
 *  AUTHORS:       Zheng Huijie    START DATE: Thursday November 19th 2020
 *
 *  LAST MODIFIED: Wednesday, December 9th 2020, 9:01:36 pm
 *
 *  CONTACT:       huijie.zheng@smartmore.com
*******************************************************************************/
#include "logger.h"
#include <chrono>

LoggingInterceptor::LoggingInterceptor(
    grpc::experimental::ServerRpcInfo *info) {
    info_ = info;
    /* grpc::experimental::ServerRpcInfo::Type type = info->type(); */
}

std::string FormatDuration(double ms) {
    if (ms < 1.0) {
        return std::to_string(ms * 1000.0) + "μs";
    }
    if (ms > 1000.0) {
        return std::to_string(ms / 1000.0) + "s";
    }
    return std::to_string(ms) + "ms";
}

void LoggingInterceptor::Intercept(
    grpc::experimental::InterceptorBatchMethods *methods) {
    using std::chrono::duration;
    using std::chrono::duration_cast;
    using std::chrono::high_resolution_clock;
    if (methods->QueryInterceptionHookPoint(
            grpc::experimental::InterceptionHookPoints::POST_RECV_MESSAGE)) {
        start_ = high_resolution_clock::now();
    }

    if (methods->QueryInterceptionHookPoint(
            grpc::experimental::InterceptionHookPoints::PRE_SEND_STATUS)) {
        status_ = methods->GetSendStatus();
    }

    if (methods->QueryInterceptionHookPoint(
            grpc::experimental::InterceptionHookPoints::POST_RECV_CLOSE)) {
        duration<double, std::ratio<1, 1000>> duration_ms =
            duration_cast<duration<double, std::ratio<1, 1000>>>(
                high_resolution_clock::now() - start_);

        printf("LoggingInterceptor::Intercept | code: %d | %s | %s | %s\n", status_.error_code(),
               FormatDuration(duration_ms.count()).c_str(),
               info_->server_context()->peer().c_str(), info_->method());
    }
    methods->Proceed();
}
