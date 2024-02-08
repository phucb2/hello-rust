# How to implement gRPC in Rust

Tutorial here: [1]

[1] https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md

## Testing

``` bash
grpcurl -plaintext -import-path ./proto -proto helloworld.proto -d '{"name": "Tonic"}' '[::1]:50051' helloworld.Greeter/SayHello
```