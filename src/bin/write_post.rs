extern crate sagbackend;
extern crate diesel;

use self::sagbackend::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();
    println!("Titel?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)]; //BEGONE NEWLINE
    println!("\nOk! Lets write {} (Press {} when finished)", title, EOF);
    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();

    let post = create_post(&connection, title, &body);
    println!("\nSaved draft {} with ID {}", title, post.id);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";