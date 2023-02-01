#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        println!("GOT {}", String::from("HANDLE"));
        "return value"
    });

    // Do some other work
    println!("s");

    let out = handle.await.unwrap();
    println!("GOT {}", out);
}