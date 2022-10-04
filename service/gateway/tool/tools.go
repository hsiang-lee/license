// +build tools

package main

import (
	// For managing dependencies of the tools by go mod
	// _ "github.com/envoyproxy/protoc-gen-validate"
	_ "github.com/gogo/protobuf/protoc-gen-gofast"
	_ "github.com/grpc-ecosystem/grpc-gateway/protoc-gen-grpc-gateway"
)
