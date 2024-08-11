use serde::Serialize;
use tauri::{ipc::Channel, AppHandle, Emitter};

use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_shell::ShellExt;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
pub enum DownloadEvent<'a> {
    #[serde(rename_all = "camelCase")]
    Started {
        url: &'a str,
        download_id: usize,
        content_length: usize,
    },
    #[serde(rename_all = "camelCase")]
    Progress {
        download_id: usize,
        chunk_length: usize,
    },
    #[serde(rename_all = "camelCase")]
    Finished { download_id: usize },
}

#[tauri::command]
pub fn download(app: AppHandle, url: String, on_event: Channel<DownloadEvent>) {
    let content_length = 1000;
    let download_id = 1;

    let sidecar_command = app.shell().sidecar("yt-dlp").unwrap().args([
        &url,
        "--write-all-thumbnails",
        "--write-playlist-metafiles",
        "--write-info-json",
        "--write-description",
        "--write-link",
        "--write-subs",
        "--write-auto-subs",
        // "--progress-template '%(progress._default_template)s'",
    ]);
    let (mut rx, mut _child) = sidecar_command.spawn().expect("Failed to spawn sidecar");

    let app_handle = app.clone();
    app_handle
        .emit(
            "rs2js",
            "Starting",
        )
        .expect("Failed to emit event");

    tauri::async_runtime::spawn(async move {
        // Read events such as stdout
        while let Some(event) = rx.recv().await {
            if let CommandEvent::Stdout(line) = event {
                // println!("{:?}", String::from_utf8(line).unwrap());
                app_handle
                    .emit(
                        "rs2js",
                        Some(format!("'{}'", String::from_utf8(line).unwrap())),
                    )
                    .expect("Failed to emit event");
            }
        }
        app_handle
            .emit(
                "rs2js",
                "Completed",
            )
            .expect("Failed to emit event");
    });
}
