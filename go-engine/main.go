package main

import "C"

import (
	"context"
	"errors"
	"net"
	"os"
	"sync"
	"time"

	"github.com/anyproto/any-sync/app"
	"google.golang.org/grpc"

	"github.com/anyproto/anytype-heart/core"
	"github.com/anyproto/anytype-heart/core/event"
	"github.com/anyproto/anytype-heart/metrics"
	"github.com/anyproto/anytype-heart/pb"
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
		}
	}()

	grpcAddr := C.GoString(cGrpcAddr)

	serverMutex.Lock()
	defer serverMutex.Unlock()

	if globalServer != nil {
		log.Info("Engine is already running")
		return 0
	}

	// Fix 6: Initialize metrics before creating middleware
	metrics.Service.InitWithKeys(metrics.DefaultInHouseKey)

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
		listener.Close()
		return 1
	}

	// Fix 5: Ensure compilation does NOT use '-tags nogrpcserver'
	mw.SetEventSender(event.NewGrpcSender())

	// Fix 3 & 4: Rebuild the exact Interceptor Pipeline used by the real server
	var interceptors []grpc.UnaryServerInterceptor

	// Add Authorize & Panic Recovery interceptor
	interceptors = append(interceptors, func(
		ctx context.Context,
		req interface{},
		info *grpc.UnaryServerInfo,
		handler grpc.UnaryHandler,
	) (resp interface{}, err error) {
		// Panic Recovery wrapper
		defer func() {
			if r := recover(); r != nil {
				if rerr, ok := r.(error); ok && errors.Is(rerr, core.ErrNotLoggedIn) {
					log.Warnf("Unauthorized access attempt caught: %v", rerr)
				} else {
					log.Errorf("gRPC handler panic recovered: %v", r)
				}
			}
		}()

		switch info.FullMethod {
		case "/service.ClientCommands/AppShutdown",
			"/service.ClientCommands/InitialSetParameters",
			"/service.ClientCommands/AccountCreate",
			"/service.ClientCommands/AccountSelect":
			// Bypass authorization validation for these setup calls
			return handler(ctx, req)
		}
		// Authorize wraps the handler execution
		resp, err = mw.Authorize(ctx, req, info, handler)
		if err != nil {
			log.Errorf("authorize failure: %s", err)
		}
		return resp, err
	})

	grpcServer := grpc.NewServer(
		grpc.MaxRecvMsgSize(20*1024*1024),
		grpc.ChainUnaryInterceptor(interceptors...),
	)

	// Register endpoints
	service.RegisterClientCommandsServer(grpcServer, mw)

	globalServer = &Server{
		mw:         mw,
		grpcServer: grpcServer,
		listener:   listener,
	}

	go func() {
		log.Infof("Starting gRPC server on %s", listener.Addr())
		if err := grpcServer.Serve(listener); err != nil && !errors.Is(err, grpc.ErrServerStopped) {
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

		// Fix 2: Stop gRPC server FIRST to shed connections safely
		globalServer.grpcServer.Stop()
		globalServer.listener.Close()

		// Fix 1: Shut down middleware SECOND using the correct pb payload package
		globalServer.mw.AppShutdown(
			context.Background(),
			&pb.RpcAppShutdownRequest{},
		)

		globalServer = nil
		log.Info("Engine cleanly stopped and ports cleared.")
	}
}

func main() {}

/*
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

	service.RegisterClientCommandsServer(grpcServer, mw)

	globalServer = &Server{
		mw:         mw,
		grpcServer: grpcServer,
		listener:   listener,
	}

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
		log.Info("Shutting down engine middleware and gateway...")

		ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
		defer cancel()

		if globalServer.mw != nil {
			_ = globalServer.mw.AppShutdown(ctx, &service.RpcAppShutdownRequest{})
		}

		log.Info("Stopping gRPC listener immediately...")
		globalServer.grpcServer.Stop()
		globalServer.listener.Close()

		globalServer = nil
		log.Info("Engine completely stopped and ports released.")
	}
}

func main() {}
//*/
