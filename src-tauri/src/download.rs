
use tauri::{AppHandle, ipc::Channel, Emitter};
use serde::Serialize;

use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::CommandEvent;


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
  Finished {
    download_id: usize,
  },
}

#[tauri::command]
pub fn download(app: AppHandle, url: String, on_event: Channel<DownloadEvent>) {
    let content_length = 1000;
    let download_id = 1;

    let sidecar_command = app.shell()
        .sidecar("yt-dlp")
        .unwrap()
        .args([&url]);
    let (mut rx, mut _child) = sidecar_command
        .spawn()
        .expect("Failed to spawn sidecar");

    on_event.send(DownloadEvent::Started {
        url: &url,
        download_id,
        content_length,
    }).unwrap();

    let app_handle = app.clone();
    tauri::async_runtime::spawn(async move {
        // read events such as stdout
        while let Some(event) = rx.recv().await {
            if let CommandEvent::Stdout(line) = event {
                // println!("{:?}", String::from_utf8(line).unwrap());
                app_handle.emit("rs2js",
                    Some(format!("'{}'", String::from_utf8(line).unwrap()))
                ).expect("failed to emit event");
            }
        }
    });

    for chunk_length in [15, 150, 35, 500, 300] {
        on_event.send(DownloadEvent::Progress {
            download_id,
            chunk_length,
        }).unwrap();
    }

    on_event.send(DownloadEvent::Finished { download_id }).unwrap();
}
