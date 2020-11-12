# murmur_grpc
<a href="https://docs.rs/murmur_grpc"><img src="https://docs.rs/murmur_grpc/badge.svg" alt="Docs.rs badge"></a>
<a href="https://crates.io/crates/murmur_grpc"><img src="https://img.shields.io/crates/v/murmur_grpc" alt="Crates.io badge"></a>

This library was primarily created for my own purposes in order to take the pain out of interacting with Murmur's gRPC inteface. 
All of the functionality is available, but no extensive testing has been done at this point so your mileage may vary. The documentation
is very sparse at the moment because I am very stupid, but I intend to improve it as the library matures.

### Examples

This example prints the contents of every message that is sent to a Mumble server. 
The `text_message` function gets called for each message that is sent.

```rust
use murmur_grpc::*;

fn text_message(_t: DataMutex<()>, _c: Client, event: &Event) -> FutureBool {
    println!("{}", event.message.as_ref().unwrap().text.as_ref().unwrap());
    // All of client's methods for communicating with Murmur are asynchronous so this function
    // must return a future even though we aren't doing anything asynchronous in this example.
    future_from_bool(true)
}

fn main() {
    let i = MurmurInterfaceBuilder::new((), "http://127.0.0.1:50051")
        .user_text_message(vec![text_message])
        .build();
    murmur_grpc::start(1, vec![i]);
    std::thread::park();
}
```
