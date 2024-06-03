#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use midir::{Ignore, MidiInput, MidiInputConnection};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use tauri::{Manager, Window, Wry};

use tauri::api::process::{Command as TauriCommand, CommandEvent};

#[derive(Default)]
pub struct MidiState {
    pub input: Mutex<Option<MidiInputConnection<()>>>,
}

#[derive(Clone, Serialize)]
struct MidiMessage {
    message: Vec<u8>,
}

#[tauri::command]
fn open_midi_connection(
    midi_state: tauri::State<'_, MidiState>,
    window: Window<Wry>,
    input_idx: usize,
) {
    let handle = Arc::new(window).clone();
    let mut midi_in = MidiInput::new("My Test Input").unwrap();
    midi_in.ignore(Ignore::None);
    let midi_in_ports = midi_in.ports();
    if let Some(in_port) = midi_in_ports.get(input_idx) {
        let conn_in = midi_in
            .connect(
                in_port,
                "midir-test",
                move |stamp, message, _log| {
                    // The last of the three callback parameters is the object that we pass in as last parameter of `connect`.

                    let _ = handle.emit_all(
                        "midi_message",
                        MidiMessage {
                            message: message.to_vec(),
                        },
                    );

                    println!("{}: {:?} (len = {})", stamp, message, message.len());
                },
                (),
            )
            .unwrap();
        *midi_state.input.lock().unwrap() = Some(conn_in);
    }
}

fn main() {
    let (mut rx, child) = TauriCommand::new_sidecar("pocketbase")
        .expect("failed to create `pocketbase` binary command")
        .args(["serve"])
        .spawn()
        .expect("Failed to spawn sidecar: `pocketbase`");

    dbg!(
        "Spawned Pocketbase sidecar process `pocketbase` with PID: {}",
        child.pid()
    );

    tauri::async_runtime::spawn(async move {
        // read events such as stdout
        while let Some(event) = rx.recv().await {
            if let CommandEvent::Stdout(line) = &event {
                println!(" ++-> pocketbase: {line}");
            }

            if let CommandEvent::Stderr(line) = &event {
                println!(" ++-> pocketbase: {line}");
            }

            if let CommandEvent::Terminated(_line) = &event {
                println!(" ++-> pocketbase: terminated.");

                panic!("pocketbase went away :'(");
            }
        }
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_midi_connection])
        .manage(MidiState {
            ..Default::default()
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
