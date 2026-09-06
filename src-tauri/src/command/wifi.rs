use encoding_rs::GBK;
use serde::{Deserialize, Serialize};
use std::env;
use std::process::Command;

// 定义一个结构体来表示 Wi-Fi 配置文件的信息
#[derive(Serialize, Deserialize)]
struct WifiProfile {
    id: i32,
    name: String,
    password: String,
}

fn get_windows_wifi_passwords() -> Result<Vec<WifiProfile>, String> {
    // 执行命令获取所有 Wi-Fi 配置文件名称
    let output = Command::new("netsh")
        .arg("wlan")
        .arg("show")
        .arg("profiles")
        .output()
        .map_err(|e| e.to_string())?;

    // 使用 GBK 编码将字节数组转换为字符串
    let (profiles_output, _, _) = GBK.decode(&output.stdout);
    let profiles_output = profiles_output.to_string();

    // 提取所有 Wi-Fi 配置文件名称
    let mut profiles: Vec<String> = Vec::new();
    for line in profiles_output.lines() {
        if line.contains("所有用户配置文件") {
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() > 1 {
                let profile_name = parts[1].trim().to_string();
                profiles.push(profile_name);
            }
        }
    }

    let mut wifi_profiles: Vec<WifiProfile> = Vec::new();
    let mut id = 0;
    for profile in profiles {
        // 执行命令获取每个 Wi-Fi 配置文件的详细信息，包括密码
        let output = Command::new("netsh")
            .arg("wlan")
            .arg("show")
            .arg("profile")
            .arg(format!("name={}", profile))
            .arg("key=clear")
            .output()
            .map_err(|e| e.to_string())?;

        // 使用 GBK 编码将字节数组转换为字符串
        let (profile_output, _, _) = GBK.decode(&output.stdout);
        let profile_output = profile_output.to_string();

        let mut wifi_name = String::new();
        let mut wifi_password = String::new();

        for line in profile_output.lines() {
            if line.contains("名称") && !line.contains("SSID") {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if parts.len() > 1 {
                    wifi_name = parts[1].trim().to_string();
                }
            } else if line.contains("关键内容") {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if parts.len() > 1 {
                    wifi_password = parts[1].trim().to_string();
                }
            }
        }
        if !wifi_name.is_empty() {
            wifi_profiles.push(WifiProfile {
                id,
                name: wifi_name,
                password: wifi_password,
            });
        }
        id += 1;
    }
    Ok(wifi_profiles)
}

fn get_macos_wifi_passwords() -> Result<Vec<WifiProfile>, String> {
    let output = Command::new("networksetup")
        .arg("-listallnetworkservices")
        .output()
        .map_err(|e| e.to_string())?;
    let services = String::from_utf8_lossy(&output.stdout);
    let mut wifi_profiles = Vec::new();
    let mut id = 0;
    for line in services.lines() {
        if line.contains("Wi-Fi") {
            let service = line.trim();
            let output = Command::new("security")
                .arg("find-generic-password")
                .arg("-wa")
                .arg(service)
                .output();
            if let Ok(output) = output {
                let password = String::from_utf8_lossy(&output.stdout).trim().to_string();
                wifi_profiles.push(WifiProfile {
                    id,
                    name: service.to_string(),
                    password,
                });
                id += 1;
            }
        }
    }
    Ok(wifi_profiles)
}

fn get_linux_wifi_passwords() -> Result<Vec<WifiProfile>, String> {
    use std::fs;
    let mut wifi_profiles = Vec::new();
    let mut id = 0;
    if let Ok(entries) = fs::read_dir("/etc/NetworkManager/system-connections/") {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Ok(content) = fs::read_to_string(path) {
                    let mut wifi_name = String::new();
                    let mut wifi_password = String::new();
                    for line in content.lines() {
                        if line.starts_with("id=") {
                            wifi_name = line.split('=').nth(1).unwrap_or("").to_string();
                        } else if line.starts_with("psk=") {
                            wifi_password = line.split('=').nth(1).unwrap_or("").to_string();
                        }
                    }
                    if !wifi_name.is_empty() {
                        wifi_profiles.push(WifiProfile {
                            id,
                            name: wifi_name,
                            password: wifi_password,
                        });
                        id += 1;
                    }
                }
            }
        }
    }
    Ok(wifi_profiles)
}

#[tauri::command]
pub fn get_wifi_passwords() -> Result<String, String> {
    let os = env::consts::OS;
    let wifi_profiles = match os {
        "windows" => get_windows_wifi_passwords()?,
        "macos" => get_macos_wifi_passwords()?,
        "linux" => get_linux_wifi_passwords()?,
        _ => return Err("Unsupported operating system".to_string()),
    };
    let result = serde_json::to_string(&wifi_profiles).map_err(|e| e.to_string())?;
    Ok(result)
}
