/*package main

import "C"

import (
	"context"
	"net"
	"os"
	"sync"
	"time"

	"github.com/anyproto/any-sync/app"
	"google.golang.org/grpc"

	"github.com/anyproto/anytype-heart/core"
	"github.com/anyproto/anytype-heart/core/event"
	"github.com/anyproto/anytype-heart/pb/service"
	"github.com/anyproto/anytype-heart/pkg/lib/logging"
)

var log = logging.Logger("anytype-heart")

var (
	globalServer *Server
	serverMutex  sync.Mutex
)

type Server struct {
	mw         *core.Middleware
	grpcServer *grpc.Server
	listener   net.Listener
}

//export StartAnytypeEngine
func StartAnytypeEngine(cGrpcAddr *C.char) C.int {
	grpcAddr := C.GoString(cGrpcAddr)

	serverMutex.Lock()
	defer serverMutex.Unlock()

	if globalServer != nil {
		log.Info("Engine is already running")
		return 0
	}

	app.StartWarningAfter = time.Second * 5
	os.Setenv("ANYTYPE_LOG_LEVEL", "ERROR")

	listener, err := net.Listen("tcp", grpcAddr)
	if err != nil {
		log.Errorf("Failed to listen on %s: %v", grpcAddr, err)
		return 1
	}

	mw := core.New()
	mw.SetEventSender(event.NewGrpcSender())

	grpcServer := grpc.NewServer(
		grpc.MaxRecvMsgSize(20 * 1024 * 1024),
	)

	// Register the Anytype middleware endpoints to the gRPC server
	service.RegisterClientCommandsServer(grpcServer, mw)

	globalServer = &Server{
		mw:         mw,
		grpcServer: grpcServer,
		listener:   listener,
	}

	// Start serving in the background so we don't block Rust
	go func() {
		log.Infof("Starting gRPC server on %s", listener.Addr())
		if err := grpcServer.Serve(listener); err != nil {
			log.Errorf("gRPC server error: %v", err)
		}
	}()

	return 0
}

//export StopAnytypeEngine
func StopAnytypeEngine() {
	serverMutex.Lock()
	defer serverMutex.Unlock()

	if globalServer != nil {
		log.Info("Shutting down engine...")
		globalServer.grpcServer.GracefulStop()

		ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
		defer cancel()
		// _ = globalServer.mw.AppShutdown(ctx, &pb.RpcAppShutdownRequest{})

		globalServer = nil
		log.Info("Engine stopped")
	}
}

func main() {}
*/
//*
package main

import "C"

import (
	"context"
	"net"
	"os"
	"sync"
	"time"

	"github.com/anyproto/any-sync/app"
	"google.golang.org/grpc"

	"github.com/anyproto/anytype-heart/core"
	"github.com/anyproto/anytype-heart/core/event"
	"github.com/anyproto/anytype-heart/pb/service"
	"github.com/anyproto/anytype-heart/pkg/lib/logging"
)

var log = logging.Logger("anytype-heart")

var (
	globalServer *Server
	serverMutex  sync.Mutex
)

type Server struct {
	mw         *core.Middleware
	grpcServer *grpc.Server
	listener   net.Listener
}

//export StartAnytypeEngine
func StartAnytypeEngine(cGrpcAddr *C.char) C.int {
	defer func() {
		if r := recover(); r != nil {
			log.Errorf("Recovered from panic in StartAnytypeEngine: %v", r)
			// You can also print the stack trace here if needed
		}
	}()

	grpcAddr := C.GoString(cGrpcAddr)

	serverMutex.Lock()
	defer serverMutex.Unlock()

	if globalServer != nil {
		log.Info("Engine is already running")
		return 0
	}

	app.StartWarningAfter = time.Second * 5
	os.Setenv("ANYTYPE_LOG_LEVEL", "ERROR")

	listener, err := net.Listen("tcp", grpcAddr)
	if err != nil {
		log.Errorf("Failed to listen on %s: %v", grpcAddr, err)
		return 1
	}

	mw := core.New()
	if mw == nil {
		log.Error("core.New() returned nil!")
		return 1
	}
	mw.SetEventSender(event.NewGrpcSender())

	grpcServer := grpc.NewServer(
		grpc.MaxRecvMsgSize(20 * 1024 * 1024),
	)

	// Register the Anytype middleware endpoints to the gRPC server
	service.RegisterClientCommandsServer(grpcServer, mw)

	globalServer = &Server{
		mw:         mw,
		grpcServer: grpcServer,
		listener:   listener,
	}

	// Start serving in the background so we don't block Rust
	go func() {
		log.Infof("Starting gRPC server on %s", listener.Addr())
		if err := grpcServer.Serve(listener); err != nil {
			log.Errorf("gRPC server error: %v", err)
		}
	}()

	return 0
}

//export StopAnytypeEngine
func StopAnytypeEngine() {
	serverMutex.Lock()
	defer serverMutex.Unlock()

	if globalServer != nil {
		log.Info("Shutting down engine...")
		globalServer.grpcServer.GracefulStop()

		_, cancel := context.WithTimeout(context.Background(), 5*time.Second)
		defer cancel()
		// _ = globalServer.mw.AppShutdown(ctx, &pb.RpcAppShutdownRequest{})

		globalServer = nil
		log.Info("Engine stopped")
	}
}

func main() {}

//*/
