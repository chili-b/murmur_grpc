# murmur_grpc

### Examlpes

This example prints the contents of every message that is sent to a Mumble server. 
The `text_message` function gets called for each message that is sent.

```rust
use murmur_grpc::*;

fn text_message(_t: DataMutex<()>, _c: Client, event: &Event) -> bool {
    println!("{}", event.message.as_ref().unwrap().text.as_ref().unwrap());
    true
}

fn main() {
    let i = MurmurInterfaceBuilder::new((), "http://127.0.0.1:50051")
        .user_text_message(vec![text_message])
        .build();
    murmur_grpc::start(1, vec![i]);
    std::thread::park();
}
```
