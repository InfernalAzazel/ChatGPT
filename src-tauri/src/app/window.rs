pub fn open_main(handle: &tauri::AppHandle) {
    let app = handle.clone();

    tauri::async_runtime::spawn(async move {
        let chat_gpt = tauri::WindowBuilder::new(
            &app,
            "main", /* the unique window label */
            tauri::WindowUrl::External("https://www.baidu.com".parse().unwrap()),
        );
        chat_gpt.build().unwrap();
    });
}

#[tauri::command]
pub fn open_settings(handle: tauri::AppHandle) {
    let app = handle.clone();
    tauri::async_runtime::spawn(async move {
        let settings = tauri::WindowBuilder::new(
            &app,
            "settings", /* the unique window label */
            tauri::WindowUrl::default(),
        )
            .title("settings")
            .build().unwrap();
    });
}