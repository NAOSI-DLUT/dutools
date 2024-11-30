use pnet::datalink;
use serde::Serialize;

#[derive(Serialize)]
struct NetworkInfo {
    name: String,
    ips: Vec<String>,
    mac: String,
}

#[tauri::command]
fn get_network_info() -> Vec<NetworkInfo> {
    let interfaces = datalink::interfaces();
    // don't use is_up() due to https://github.com/libpnet/libpnet/issues/564
    let mut network_info = Vec::new();

    for iface in interfaces {
        let ips: Vec<String> = iface.ips.iter().map(|ip| ip.ip().to_string()).collect();

        if !ips.is_empty() && ips.iter().any(|ip| ip != "0.0.0.0") {
            network_info.push(NetworkInfo {
                name: iface.name.clone(),
                ips,
                mac: iface.mac.unwrap().to_string(),
            });
        }
    }

    network_info
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_network_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
