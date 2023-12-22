use pb::hello;

mod pb;

fn main() {
    let request = hello::HelloRequest {
        name: "world".to_string(),
    };
    println!("request: {:?}", request);
}
