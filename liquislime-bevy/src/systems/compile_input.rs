use bevy::{prelude::*, tasks::AsyncComputeTaskPool};

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

fn compile_input() {
    if cfg!(not(target_arch = "wasm32")) {
        return;
    }

    if get_status() == Status::Waiting {
        start_compilation_task(get_custom_unit_source());
        set_status(Status::Compiling);
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Status {
    Idle,
    Waiting,   // Use has pressed "compile"
    Compiling, // Compilation is ongoing
    Success,   // Like idle, but last compilation was successful
    Error,     // Like idle, but last compilation failed
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
    } else if status_text == "success" {
        Status::Success
    } else if status_text == "error" {
        Status::Error
    } else {
        panic!("Cannot get status")
    }
}

fn set_status(status: Status) {
    let text = match status {
        Status::Idle => "idle",
        Status::Waiting => "waiting",
        Status::Compiling => "compiling",
        Status::Success => "success",
        Status::Error => "error",
    };

    js_sys::eval(&format!(
        "document.getElementById('status').innerText = '{text}'"
    ))
    .unwrap();
}

fn get_custom_unit_source() -> String {
    js_sys::eval("document.getElementById('custom_source').value")
        .unwrap()
        .as_string()
        .unwrap()
}

fn start_compilation_task(source: String) {
    let thread_pool = AsyncComputeTaskPool::get();
    thread_pool.spawn_local(compile_task(source));
}

async fn compile_task(source: String) {
    let resp = reqwest::Client::new()
        .post("127.0.0.1:8000/compile")
        .body(source)
        .send()
        .await
        .expect("send post request");

    info!("RESP: {:?}", resp);

    set_status(Status::Success);
}
