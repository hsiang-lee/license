/*******************************************************************************
 *  FILENAME:      config.cpp
 *
 *  AUTHORS:       Zheng Huijie    START DATE: Wednesday December 23rd 2020
 *
 *  LAST MODIFIED: 
 *
 *  CONTACT:       huijie.zheng@smartmore.com
*******************************************************************************/

#include "config.h"
#include "Yaml.hpp"

namespace {
const static std::string kYamlPath = "/yaml/smlicense.yaml";

const static std::string kYamlPort = "port";
const static std::string kYamlNodePort = "nodePort";

const static std::string kYamlGatewayPortName = "gateway";
const static std::string kYamlGRpcPortName = "grpc";

const static std::string kGatewayDefaultPort = "8080";
const static std::string kGRpcDefaultPort = "3002";
}  // namespace

std::string GetGatewayPort() {
    std::string port;
    try {
        Yaml::Node root;
        Yaml::Parse(root, kYamlPath.c_str());

        Yaml::Node &items = root["spec"]["ports"];
        for (auto it = items.Begin(); it != items.End(); it++) {
            Yaml::Node &item = (*it).second;
            if (item["name"].As<std::string>() == kYamlGatewayPortName) {
                port = item["port"].As<std::string>();
            }
        }
    } catch (const std::exception &e) {
        printf("GatewayPort parse exception:%s, using default port:%s\n",
               e.what(), kGatewayDefaultPort.c_str());
    }
    return port.empty() ? kGatewayDefaultPort : port;
}

std::string GetGRpcPort() {
    std::string port;
    try {
        Yaml::Node root;
        Yaml::Parse(root, kYamlPath.c_str());

        Yaml::Node &items = root["spec"]["ports"];
        for (auto it = items.Begin(); it != items.End(); it++) {
            Yaml::Node &item = (*it).second;
            if (item["name"].As<std::string>() == kYamlGRpcPortName) {
                port = item["port"].As<std::string>();
            }
        }
    } catch (const std::exception &e) {
        printf("GRpcPort parse exception:%s, using default port:%s\n",
               e.what(), kYamlGRpcPortName.c_str());
    }
    return port.empty() ? kGRpcDefaultPort : port;
}
