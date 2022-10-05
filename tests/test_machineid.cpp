#include <gtest/gtest.h>
#include "license.h"
#include <iostream>

TEST(MachineInfoTest, TestGetMachineID) {
    std::string machine_id = lic::GetMachineID();
#if defined(__linux__)
    REQUIRE(machine_id.size() == 19);
#endif
#if (defined(_WIN32) || defined(_WIN64))
    REQUIRE(machine_id.size() == 28);
#endif
}
