#![feature(proc_macro)]

#[macro_use]
extern crate stdweb;
use stdweb::js_export;
use stdweb::web::event::*;
use stdweb::web::IEventTarget;
use stdweb::web::XmlHttpRequest;

#[js_export]
fn test() {
    let xhr = XmlHttpRequest::new();
    xhr.add_event_listener(|e: ResourceLoadEvent| {
        js!{ console.log(@{format!("{:?}", e)}); };
    });
    xhr.open("GET", "/")
        .and_then(|_| xhr.send())
        .unwrap();
}

fn main() {
    println!("Hello, world!");
}
