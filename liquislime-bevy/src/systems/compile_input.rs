use bevy::{prelude::*, tasks::AsyncComputeTaskPool};

use crate::{
    components::{ScriptComponent, ScriptsComponent, SlimeGrid},
    helpers::{CompileInput, Phase},
    units::UnitId,
};

pub struct CompileInputPlugin;

impl Plugin for CompileInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, compile_input.in_set(CompileInput));
    }
}

fn compile_input(query: Query<(&UnitId, &ScriptsComponent)>) {
    if cfg!(not(target_arch = "wasm32")) {
        return;
    }

    if get_status() == Status::Waiting {
        let unit = query
            .iter()
            .find_map(|(id, script)| {
                if *id == UnitId(1) {
                    Some(script.clone())
                } else {
                    None
                }
            })
            .expect("Find unit with id 1");

        // FIXME: fix this
        // start_compilation_task(unit.0[1], get_custom_unit_source());
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
    let status_text = unsafe { js_sys::eval("document.getElementById('status').innerText") }
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

    unsafe {
        js_sys::eval(&format!(
            "document.getElementById('status').innerText = '{text}'"
        ))
    }
    .unwrap();
}

fn get_custom_unit_source() -> String {
    unsafe { js_sys::eval("document.getElementById('custom_source').value") }
        .unwrap()
        .as_string()
        .unwrap()
}

fn start_compilation_task(unit: ScriptComponent, source: String) {
    info!("Starting compilation task");
    let thread_pool = AsyncComputeTaskPool::get();
    thread_pool.spawn_local(compile_task(unit, source));
}

async fn compile_task(unit: ScriptComponent, source: String) {
    info!("Started compilation task");

    let resp = reqwest::Client::new()
        .post("http://127.0.0.1:8000/compile")
        .body(source)
        .send()
        .await
        .expect("send post request");

    info!("RESP: {:?}", resp);

    let bytes = resp.bytes().await.expect("RESP should be bytes");

    if bytes.is_empty() {
        set_status(Status::Error);
    } else {
        unit.load_from_bytes(bytes.into(), todo!());
        set_status(Status::Success);
    }
}
