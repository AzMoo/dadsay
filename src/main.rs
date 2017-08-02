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

const TERMWIDTH: i32 = 80;
const PADDING: i32 = 5;

fn get_max_line_length(lines: &Vec<&str>) -> usize {
    let mut largest = lines[0].len();
    for &line in lines.iter() {
        if line.len() > largest {
            largest = line.len();
        }
    }

    largest
}


fn split_joke_into_lines(joke: &str, line_width: i32) -> Vec<String> {
    let mut newlines = Vec::<String>::new();

    for line in joke.lines() {
        if line.len() > line_width as usize {
            let mut current_line = Vec::<String>::new();
            
            for word in line.split(" ") {
                let joinedline = current_line.join(" ");
                let this_line_length = joinedline.len() + word.len();

                if this_line_length > line_width as usize {
                    newlines.push(joinedline);
                    current_line = Vec::<String>::new();
                }

                current_line.push(word.to_owned());
            }

            newlines.push(current_line.join(" "));
        } else {
            newlines.push(line.to_owned());
        }
    }

    newlines
}


fn print_joke_and_dadface(joke: Vec<String>, dadface: Vec<&str>, jokewidth: i32) {

    let diff = dadface.len() - joke.len() - 1;
    let top_padding = diff / 2;
    let text_bottom = top_padding + joke.len();

    for idx in 0..dadface.len() {
        let mut jokestring = "";
        
        if idx > text_bottom - joke.len() && idx <= text_bottom {
            let joke_idx = idx as i32 - text_bottom as i32 + joke.len() as i32 - 1;
            jokestring = &joke[joke_idx as usize];
        }

        println!(
            "{:lpad$}{:width$}{}", 
            "", jokestring, dadface[idx], 
            lpad = PADDING as usize, 
            width = (jokewidth + PADDING) as usize
        );
    }
}


fn main() {
    // Create our Core Event Loop
    let mut core = match Core::new() {
        Ok(core) => core,
        Err(_) => panic!("Couldn't start event loop!")
    };

    let dadface = vec![
        r#"      ***********"#,
        r#"    ***** ***********"#,
        r#"    ** ****** *** ********"#,
        r#"****  ******  ** *******"#,
        r#"***     ******* ** ******"#,
        r#"***       **        *  **"#,
        r#"    *|\------  \------\ ** *"#,
        r#"    |       |=|       :===**"#,
        r#"    |  O  |   | O   |  }|*"#,
        r#"    |---- |   ----  |  |*"#,
        r#"    |    |___       |\/"#,
        r#"    |              |"#,
        r#"     \  \ ----/    |"#,
        r#"      \  \___/     |"#,
        r#"        -__ -- -/"#
    ];

    let max: usize = get_max_line_length(&dadface);
    // We want a gap of at least PADDING spaces on each
    // side of the joke which means we have this much 
    // space to fit the joke
    let jokewidth: i32 = TERMWIDTH - PADDING * 2 - (max as i32);

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
            let jokelines = split_joke_into_lines(joke, jokewidth);
            print_joke_and_dadface(jokelines, dadface, jokewidth);
            Ok(())
        })
    });

    // Actually run the request
    core.run(work).unwrap();
}
