/*******************************************************************************
 *  FILENAME:      c_config.cpp
 *
 *  AUTHORS:       Zheng Huijie    START DATE: Wednesday December 23rd 2020
 *
 *  LAST MODIFIED: 
 *
 *  CONTACT:       huijie.zheng@smartmore.com
*******************************************************************************/

#include "config.h"
#include "c_config.h"

const char *C_GetGatewayPort() {
    return GetGatewayPort().c_str();  //NOLINT
}
const char *C_GetGRpcPort() {
    return GetGRpcPort().c_str();  //NOLINT
}
