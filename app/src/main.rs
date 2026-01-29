use leptos::prelude::*;

use crate::app::App;

mod app;
mod components;
mod screens;
mod state;
mod styles;

fn main() {
    mount_to_body(|| view! { <App /> });
}
