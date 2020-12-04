# murmur_grpc
<a href="https://docs.rs/murmur_grpc"><img src="https://docs.rs/murmur_grpc/badge.svg" alt="Docs.rs badge"></a>
<a href="https://crates.io/crates/murmur_grpc"><img src="https://img.shields.io/crates/v/murmur_grpc" alt="Crates.io badge"></a>

I made this library primarily for myself in order to simplify working with gRPC. All of the functionality is available, 
but no extensive testing has been done at this point so your mileage may vary. I really don't reccommend using this in
anything serious at this point in time.

### Example Usage

This example prints the contents of every message that is sent to a Mumble server. 
The `text_message` function gets called for each message that is sent. Internally,
the library is asynchronous, but to get around that a helper function named `future_from_bool`
is provided which is demonstrated in the example below.

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
    murmur_grpc::start_connection(i)
        .join()
        .expect("Waiting for connection to Mumble server to close.");
}
```
