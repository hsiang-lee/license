/*******************************************************************************
 *  FILENAME:      test_machineid.cpp
 *
 *  AUTHORS:       Wang Xiaofei    START DATE: Saturday October 24th 2020
 *
 *  LAST MODIFIED: Thursday, October 29th 2020, 9:22:53 pm
 *
 *  CONTACT:       xiaofei.wang@smartmore.com
*******************************************************************************/

#include "catch2/catch.hpp"
#include "smlicense.h"
#include <iostream>

TEST_CASE("machineinfo") {
    std::string machine_id = smartmore::lic::GetMachineID();
#if defined(__linux__)
    REQUIRE(machine_id.size() == 19);
#endif
#if (defined(_WIN32) || defined(_WIN64))
    REQUIRE(machine_id.size() == 28);
#endif
}
