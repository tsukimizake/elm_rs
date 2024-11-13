#![allow(dead_code)]

use elm_rs::{Elm, ElmDecode, ElmEncode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Elm, ElmEncode, ElmDecode)]
enum Filetype {
    Jpeg,
    Png,
    #[elm_rs(lazy)]
    Recursive(Box<Filetype>),
}

fn main() {
    // the target would typically be a file
    let mut target = vec![];
    // elm_rs provides a macro for conveniently creating an Elm module with everything needed
    elm_rs::export!("Bindings", &mut target, {
        decoders: [Filetype],
        // generates types and functions for forming queries for types implementing ElmQuery
    })
    .unwrap();
    let output = String::from_utf8(target).unwrap();
    println!("{}", output);
}
