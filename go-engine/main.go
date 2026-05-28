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

	mw.SetEventSender(event.NewGrpcSender())

	var interceptors []grpc.UnaryServerInterceptor

	interceptors = append(interceptors, func(
		ctx context.Context,
		req interface{},
		info *grpc.UnaryServerInfo,
		handler grpc.UnaryHandler,
	) (resp interface{}, err error) {
		defer func() {
			if r := recover(); r != nil {
				if rerr, ok := r.(error); ok && errors.Is(rerr, core.ErrNotLoggedIn) {
					log.Warnf("Unauthorized access attempt caught: %v", rerr)
				} else {
					log.Errorf("gRPC handler panic recovered: %v", r)
				}
			}
		}()

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

		globalServer.grpcServer.Stop()
		globalServer.listener.Close()

		globalServer.mw.AppShutdown(
			context.Background(),
			&pb.RpcAppShutdownRequest{},
		)

		globalServer = nil
		log.Info("Engine cleanly stopped and ports cleared.")
	}
}

func main() {}
