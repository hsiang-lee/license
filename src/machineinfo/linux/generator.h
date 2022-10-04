#ifndef SMARTMORE_MACHINEID_GENERATOR_H
#define SMARTMORE_MACHINEID_GENERATOR_H

#include <array>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

#include "global_data.h"

// windows
#if (defined(_WIN32) || defined(_WIN64))
// for mac address
#include <winsock2.h>
#include <iphlpapi.h>

#pragma comment(lib, "IPHLPAPI.lib")
#pragma comment(lib, "ws2_32.lib")
// for __cpuid()
#include <intrin.h>

// for register GUID
#include <objbase.h>
#include <stdexcept>

#endif
// linux
#if defined(__linux__)
// for mac
// for mac address
#include <arpa/inet.h>
#include <net/if.h>
#include <netinet/in.h>
#include <stdio.h>
#include <string.h>
#include <sys/ioctl.h>
#include <sys/socket.h>

// for cpu id
#include <unistd.h>
#endif

using Byte = unsigned char;
using Uint32 = unsigned int;
using Unit64 = unsigned long long;  //NOLINT

namespace smartmore {

class HardwareID {
    // base class of hardware information class

   public:
    // simple c'tor
    explicit HardwareID() : ID_{""} {};

    // default d'tor, copy and assign c'tors
    virtual ~HardwareID() = default;
    HardwareID(const HardwareID &input) = default;
    HardwareID &operator=(const HardwareID &input) = default;

    // get hardware id, a pure virtual for overload
    virtual int GetID(std::string &input) = 0;

    // check whether two hardware ids are the same or not
    bool operator==(const HardwareID &other) {
        return this->ID_ == other.ID_;
    }

   protected:
    // get different hardware id with parameter type
    std::string ID_;
};

// get cpu id with func __cpuidex since func CoInitializeEx
// cannot be called repeatedly with different parameters
class CpuIdGenerator : public HardwareID {
   public:
    int GetID(std::string &input);
};

class MacIdGenerator : public HardwareID {
   public:
    int GetID(std::string &input);
};

}  // namespace smartmore

#endif
