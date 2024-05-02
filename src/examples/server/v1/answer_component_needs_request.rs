
fn main() {
    // listen method from ws-server, wss-server, tcp-server or tls-server

    EVENT_REGISTRY.add_listener(1, 0x0000, |e| {
        let event = e.downcast::<ComponentNeedsRequest>().unwrap();

        if e.requested_component_name == Some("answer".to_string()) {
            let mut args = Vec::<Arg>::new();
            args.push(Arg::Text("Answer".to_string()));
            args.push(Arg::VecText(vec!["/path/to/answer.html".to_string()]));

            return Box::new(args);
        }
    });
}