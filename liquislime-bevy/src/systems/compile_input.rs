use bevy::prelude::*;

use crate::{
    components::SlimeGrid,
    helpers::{CompileInput, Phase},
};

pub struct CompileInputPlugin;

impl Plugin for CompileInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, compile_input.in_set(CompileInput));
    }
}

// #[cfg(not(target_arch = "wasm32"))]
// fn compile_input() {}

// #[cfg(target_arch = "wasm32")]
fn compile_input() {
    if get_status() == Status::Waiting {
        set_status(Status::Compiling);
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Status {
    Idle,
    Waiting,   // Use has pressed "compile"
    Compiling, // Compilation is ongoing
}

fn get_status() -> Status {
    let status_text = js_sys::eval("document.getElementById('status').innerText")
        .unwrap()
        .as_string()
        .unwrap();
    if status_text == "idle" {
        Status::Idle
    } else if status_text == "waiting" {
        Status::Waiting
    } else if status_text == "compiling" {
        Status::Compiling
    } else {
        panic!("Cannot get status")
    }
}

fn set_status(status: Status) {
    let text = match status {
        Status::Idle => "idle",
        Status::Waiting => "waiting",
        Status::Compiling => "compiling",
    };

    js_sys::eval(&format!(
        "document.getElementById('status').innerText = '{text}'"
    ))
    .unwrap();
}
