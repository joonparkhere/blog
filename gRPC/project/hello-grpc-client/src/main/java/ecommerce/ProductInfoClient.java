package ecommerce;

import io.grpc.ManagedChannel;
import io.grpc.ManagedChannelBuilder;

import java.util.logging.Logger;

public class ProductInfoClient {

    private static final Logger logger = Logger.getLogger(ProductInfoClient.class.getName());

    public static void main(String[] args) {
        ManagedChannel channel = ManagedChannelBuilder.forAddress("localhost", 50051)
                .usePlaintext()
                .build();

        ProductInfoGrpc.ProductInfoBlockingStub stub = ProductInfoGrpc.newBlockingStub(channel);

        ProductInfoOuterClass.ProductID productID = stub.addProduct(
                ProductInfoOuterClass.Product
                        .newBuilder()
                        .setName("Samsung S22")
                        .setDescription("Samsung S20 is the latest smart phone")
                        .setPrice(700.0f)
                        .build()
        );
        logger.info("Product ID: " + productID.getValue() + " added successfully");

        ProductInfoOuterClass.Product product = stub.getProduct(productID);
        logger.info("Product: " + product.toString());
        channel.shutdown();
    }

}
