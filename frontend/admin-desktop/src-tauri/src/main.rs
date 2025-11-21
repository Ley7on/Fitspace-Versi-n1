#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::process::{Command, Child};
use std::sync::Mutex;
use tauri::State;

struct DjangoProcess(Mutex<Option<Child>>);

#[tauri::command]
fn start_django(django_process: State<DjangoProcess>) -> Result<String, String> {
    let mut process_guard = django_process.0.lock().unwrap();
    
    if process_guard.is_some() {
        return Ok("Django ya está ejecutándose".to_string());
    }

    let child = Command::new("python")
        .args(&["manage.py", "runserver", "127.0.0.1:8000"])
        .current_dir("../profit")
        .spawn()
        .map_err(|e| format!("Error iniciando Django: {}", e))?;

    *process_guard = Some(child);
    Ok("Django iniciado correctamente".to_string())
}

#[tauri::command]
fn stop_django(django_process: State<DjangoProcess>) -> Result<String, String> {
    let mut process_guard = django_process.0.lock().unwrap();
    
    if let Some(mut child) = process_guard.take() {
        child.kill().map_err(|e| format!("Error deteniendo Django: {}", e))?;
        Ok("Django detenido correctamente".to_string())
    } else {
        Ok("Django no estaba ejecutándose".to_string())
    }
}

fn main() {
    tauri::Builder::default()
        .manage(DjangoProcess(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![start_django, stop_django])
        .setup(|app| {
            let app_handle = app.handle();
            
            // Iniciar Django automáticamente
            tauri::async_runtime::spawn(async move {
                std::thread::sleep(std::time::Duration::from_secs(2));
                let _ = app_handle.emit_all("start-django", ());
            });
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}