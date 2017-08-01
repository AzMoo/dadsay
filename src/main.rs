extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

use std::str;
use futures::{Future, Stream};
use hyper::{Client, Chunk, Method, Request};
use hyper::header::{Accept, qitem};
use hyper::mime;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;

fn main() {
    // Create our Core Event Loop
    let mut core = match Core::new() {
        Ok(core) => core,
        Err(_) => panic!("Couldn't start event loop!")
    };

    // Create a handle that can be used to access the event loop
    let handle = core.handle();

    // Create a connector hyper can use to connect https
    let connector = match HttpsConnector::new(4, &handle) {
        Ok(connector) =>  connector,
        Err(_) => panic!("Couldn't create https connector, is TLS b0rked?!")
    };

    // Attach a new client to the event loop 
    let client = Client::configure()
        .connector(connector)
        .build(&handle);
    let uri = "https://icanhazdadjoke.com/".parse().unwrap();

    // Create a new get request and send the accept header
    let mut req = Request::new(Method::Get, uri);
    req.headers_mut().set(Accept(vec![
        qitem(mime::TEXT_PLAIN)
    ]));

    // Set up our request and when we receive a response,
    // print it to the screen.
    let work = client.request(req).and_then(|res| {
        res.body().concat2().and_then(move |body: Chunk| {
            let joke = str::from_utf8(&body).unwrap();
            println!("{}", joke);
            Ok(())
        })
    });
    core.run(work).unwrap();
}
