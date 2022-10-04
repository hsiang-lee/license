/*******************************************************************************
 *  FILENAME:      test_license.cpp
 *
 *  AUTHORS:       Wang Xiaofei    START DATE: Saturday October 24th 2020
 *
 *  LAST MODIFIED: Friday, October 30th 2020, 12:50:55 pm
 *
 *  CONTACT:       xiaofei.wang@smartmore.com
*******************************************************************************/

#define CATCH_CONFIG_MAIN

#include "catch2/catch.hpp"
#include "smlicense.h"
#include <iostream>
#include <sstream>

namespace {
struct TestCase {
    std::vector<unsigned char> pk;
    std::string id;
    std::string app_name;
    smartmore::lic::Status expected;
};
}  // namespace

TEST_CASE("license") {
    std::vector<unsigned char> pk{
        36, 231, 226, 89, 105, 228, 105, 231, 245, 54, 211, 191, 175, 199, 23,
        167, 18, 241, 32, 230, 161, 175, 218, 242, 254, 74, 61, 10, 33, 159, 8, 195};

    std::vector<unsigned char> pk_malformed{
        36, 231, 226, 89, 105, 228, 105, 231, 245, 54, 211, 191, 175, 199, 23,
        167, 18, 241, 32, 230, 161, 175, 218, 242, 254, 74, 61, 110, 33, 159, 8, 190};

    std::vector<unsigned char> sk{209, 203, 151, 131, 240, 230, 140, 80, 10, 43, 227,
                                  205, 208, 163, 157, 229, 79, 214, 106, 156, 204, 132,
                                  169, 170, 19, 90, 216, 237, 51, 106, 167, 215, 36, 231,
                                  226, 89, 105, 228, 105, 231, 245, 54, 211, 191, 175,
                                  199, 23, 167, 18, 241, 32, 230, 161, 175, 218, 242,
                                  254, 74, 61, 10, 33, 159, 8, 195};

    std::vector<TestCase> cases = {
        // {pk, "AB", "AppName", smartmore::lic::success},
        // {pk, "ID", "appname", smartmore::lic::invalid_app_name},
        // {pk, "ab", "AppName", smartmore::lic::invalid_machine_id},
        {pk_malformed, "AB", "AppName", smartmore::lic::kInvalidLicense},
    };

    std::vector<char> license;
    smartmore::lic::License lic;
    lic.app_name = "AppName";
    lic.expiration_time = 12431241;
    lic.machine_id_list = {"ID", "AB"};

    std::ostringstream oss;
    smartmore::lic::GenerateLicense(lic, sk, oss);

    for (const auto &c : cases) {
        std::istringstream iss(oss.str());
        REQUIRE(smartmore::lic::CheckLicense(iss, c.pk, c.app_name, [&] { return c.id; }) == c.expected);
    }
}
