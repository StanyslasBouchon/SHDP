
fn main() {
    // listen method from ws-server, wss-server, tcp-server or tls-server

    EVENT_REGISTRY.add_listener(1, 0x0005, |e| {
        let event = e.downcast::<InteractionRequest>().unwrap();

        if e.function_name == "get_message" && e.parent_name == "users" {
            let mut args = Vec::<Arg>::new();
            args.push(Arg::OptionValue(Some(json!({
                "id": 5,
                "message": "Hello, World!"
            }))));

            return Box::new(args);
        }
    });
}