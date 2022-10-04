/*******************************************************************************
 *  FILENAME:      c_config.h
 *
 *  AUTHORS:       Zheng Huijie    START DATE: Wednesday December 23rd 2020
 *
 *  LAST MODIFIED: 
 *
 *  CONTACT:       huijie.zheng@smartmore.com
*******************************************************************************/

#ifndef UTIL_CCONFIG_H
#define UTIL_CCONFIG_H

#ifdef __cplusplus
extern "C" {
#endif

const char *C_GetGatewayPort();  //NOLINT
const char *C_GetGRpcPort();     //NOLINT

#ifdef __cplusplus
}
#endif

#endif
