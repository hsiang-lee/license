/*******************************************************************************
 *  FILENAME:      smlicense.cpp
 *
 *  AUTHORS:       Zheng Huijie    START DATE: Thursday December 3rd 2020
 *
 *  LAST MODIFIED: Friday, December 18th 2020, 12:51:48 pm
 *
 *  CONTACT:       huijie.zheng@smartmore.com
*******************************************************************************/
#include <grpcpp/impl/codegen/status.h>
#include <string>
#include <vector>
#include <sstream>

#include "smlicensegrpc.h"
#include "smlicense.h"

namespace {
enum class StatusCode {
    kOk = 0,
    kParamErr = 1,
    kServiceErr = 3,
};

const std::string kStatusOk = "成功";
const std::string kStatusParamErr = "参数错误";
const std::string kStatusServiceErr = "内部错误";
}  // namespace

grpc::Status SmlicenseServiceImpl::GenerateKey(grpc::ServerContext *ctx,
                                               const smlicense::KeyRequest *req,
                                               smlicense::KeyResponse *rsp) {
    try {
        rsp->set_requestid(req->requestid());

        std::vector<unsigned char> pk, sk;
        smartmore::lic::GenerateKeyPair(pk, sk);

        for (auto const &key : pk) {
            rsp->add_pubkey(static_cast<uint32_t>(key));
        }
        for (auto const &key : sk) {
            rsp->add_seckey(static_cast<uint32_t>(key));
        }
    } catch (const std::exception &e) {
        printf("SmlicenseServiceImpl::GenerateKey exception:%s\n", e.what());
    }

    return grpc::Status::OK;
}

grpc::Status SmlicenseServiceImpl::GenerateLicense(grpc::ServerContext *ctx,
                                                   const smlicense::LicenseRequest *req,
                                                   smlicense::LicenseResponse *rsp) {
    try {
        rsp->set_requestid(req->requestid());
        auto status = rsp->mutable_status();
        auto info = req->info();
        if (info.appname().empty() || info.machineids().empty() || req->seckey().empty()) {
            status->set_code(static_cast<uint32_t>(StatusCode::kParamErr));
            status->set_message(kStatusParamErr);
            return grpc::Status::OK;
        }

        smartmore::lic::License license;
        license.app_name = info.appname();
        license.issuer = info.issuer();
        license.expiration_time = info.expiredtimestamp();
        license.machine_id_list.insert(license.machine_id_list.begin(),
                                       info.machineids().begin(),
                                       info.machineids().end());
        license.payloads.insert(info.payloads().begin(), info.payloads().end());

        std::vector<unsigned char> sk(req->seckey().begin(), req->seckey().end());
        std::ostringstream licensebase64;

        smartmore::lic::GenerateLicense(license, sk, licensebase64);
        if (licensebase64.str().empty()) {
            status->set_code(static_cast<uint32_t>(StatusCode::kServiceErr));
            status->set_message(kStatusServiceErr);
        } else {
            status->set_code(static_cast<uint32_t>(StatusCode::kOk));
            status->set_message(kStatusOk);
        }

        rsp->set_license(licensebase64.str());
    } catch (const std::exception &e) {
        printf("SmlicenseServiceImpl::GenerateLicense exception:%s\n", e.what());
    }
    return grpc::Status::OK;
}
