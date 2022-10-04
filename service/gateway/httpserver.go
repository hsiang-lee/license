/*******************************************************************************
 *  FILENAME:      httpserver.go
 *
 *  AUTHORS:       Zheng Huijie    START DATE: Tuesday November 24th 2020
 *
 *  LAST MODIFIED: Monday, December 7th 2020, 2:51:00 pm
 *
 *  CONTACT:       huijie.zheng@smartmore.com
*******************************************************************************/

package main

import (
	context "context"
	"flag"
	"fmt"
	"net/http"

	"github.com/grpc-ecosystem/grpc-gateway/runtime"
	"github.com/smartmore/smlicense/service/gateway/smlicense"

	grpc "google.golang.org/grpc"
)



/*
#cgo CFLAGS: -I${SRCDIR}/../src
#cgo CXXFLAGS: -std=c++11 -I${SRCDIR}/../src
#cgo LDFLAGS: -L${SRCDIR}/lib -lutil -lyaml -lstdc++ 
#include "util/c_config.h"
*/
import "C"

var (
	endpoint = "localhost:" + C.GoString(C.C_GetGRpcPort())
	port     = ":" + C.GoString(C.C_GetGatewayPort())
)

func run() error {
	ctx := context.Background()
	ctx, cancel := context.WithCancel(ctx)
	defer cancel()

	mux := runtime.NewServeMux()
	// mux := runtime.NewServeMux(runtime.WithMarshalerOption(runtime.MIMEWildcard,
	// 	&runtime.JSONPb{OrigName: true, EmitDefaults: true}))
	size := 1024 * 1024 * 1024
	opts := []grpc.DialOption{
		grpc.WithInsecure(),
		grpc.WithDefaultCallOptions(grpc.MaxCallRecvMsgSize(size)),
	}

	err := smlicense.RegisterSmlicenseServiceHandlerFromEndpoint(ctx, mux, endpoint, opts)
	if err != nil {
		return err
	}
	fmt.Printf("gateway connect to %s, listen on port %s\n", endpoint, port)
	return http.ListenAndServe(port, mux)
}

func main() {
	flag.Parse()

	if err := run(); err != nil {
		fmt.Printf("gateway run error:%s\n", err.Error())
	}
}
