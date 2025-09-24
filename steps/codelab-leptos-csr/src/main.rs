mod components;
mod model;

use leptos::prelude::*;
use crate::components::app::App;

fn main() {
    mount_to_body(App);
}