/* fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/MurmurRPC.proto")?;
    Ok(())
}
*/

fn main() {
    let proto_root = "src/protos";
    protoc_grpcio::compile_grpc_protos(
        &["murmur/MurmurRPC.proto"],
        &[proto_root],
        &proto_root,
        None
    ).expect("Failed to compile gRPC definitions!");
}
