extern crate http_event_store as es;


#[test]
fn it_interacts_with_event_store() {
  let client = es::client::Client::new();

  client.get();

    println!("Hello")

}
