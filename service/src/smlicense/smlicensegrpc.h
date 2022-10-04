/*******************************************************************************
 *  FILENAME:      smlicense.h
 *
 *  AUTHORS:       Zheng Huijie    START DATE: Thursday December 3rd 2020
 *
 *  LAST MODIFIED: Thursday, December 3rd 2020, 9:30:26 pm
 *
 *  CONTACT:       huijie.zheng@smartmore.com
*******************************************************************************/
#ifndef SMLICENSE_SMLICENSEGRPC_H
#define SMLICENSE_SMLICENSEGRPC_H

#include "smlicense/smlicense.grpc.pb.h"
#include <grpcpp/impl/codegen/server_context.h>

class SmlicenseServiceImpl final : public smlicense::SmlicenseService::Service {
   public:
    grpc::Status GenerateKey(grpc::ServerContext *ctx,
                             const smlicense::KeyRequest *req,
                             smlicense::KeyResponse *rsp) override;

    grpc::Status GenerateLicense(grpc::ServerContext *ctx,
                                 const smlicense::LicenseRequest *req,
                                 smlicense::LicenseResponse *rsp) override;
};

#endif
