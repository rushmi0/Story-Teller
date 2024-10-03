#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::components::{Banner, Filter};

#[component]
pub fn HomePage() -> Element {
    rsx! {
        Banner {}
        Filter {}
    }
}