async fn async_ref() {
    let local_val = String::from("Rust");
    tokio::spawn(async {
        println!("{}", local_val);
    });
    println!("{}", local_val);
}