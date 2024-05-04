// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

#[cxx::bridge(namespace = "farzi::tauri")]
pub mod ffi {
    unsafe extern "C++" {
        include!("src/hello.h");
        // Functions implemented in C++.
        pub fn print_hello_world();
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            // Step 1: get the HWND of the window
            let hwnd = window.hwnd().expect("couldn't get HWND");

            println!("HWND: {:?}", hwnd);

            // Step 2: pass HWND to our C++/C# code and run AppWindow methods.

            // TODO: call some C++/C# code here
            ffi::print_hello_world();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
