package main

import (
	"context"
	"github.com/google/uuid"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
	pb "hello-grpc-server/ecommerce"
	"log"
	"net"
)

type server struct {
	productMap map[string]*pb.Product
	pb.UnimplementedProductInfoServer
}

func (s *server) AddProduct(ctx context.Context, in *pb.Product) (p *pb.ProductID, err error) {
	id, err := uuid.NewUUID()
	if err != nil {
		err = status.Errorf(codes.Internal, "Error while generating Product ID", err)
		return
	}

	in.Id = id.String()

	if s.productMap == nil {
		s.productMap = make(map[string]*pb.Product)
	}
	s.productMap[in.Id] = in

	p = &pb.ProductID{
		Value: in.Id,
	}
	return
}

func (s *server) GetProduct(ctx context.Context, in *pb.ProductID) (p *pb.Product, err error) {
	product, ok := s.productMap[in.Value]
	if ok && product != nil {
		log.Printf("Product %v: %v - Retrieved", product.Id, product.Name)
		p = product
		return
	}

	err = status.Errorf(codes.NotFound, "Product does not exist", in.Value)
	return
}

func main() {
	listen, err := net.Listen("tcp", ":50051")
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}

	s := grpc.NewServer()
	pb.RegisterProductInfoServer(s, &server{})

	err = s.Serve(listen)
	if err != nil {
		log.Fatalf("failed to serve: %v", err)
	}
}
