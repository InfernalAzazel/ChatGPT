use tauri::{App};
use crate::app::menu;

pub fn init(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let docs_window = tauri::WindowBuilder::new(
        app,
        "main", /* the unique window label */
        tauri::WindowUrl::External("https://www.bing.com/search?q=Bing+AI&showconv=1&FORM=hpcodx&fbg=".parse().unwrap()),
    )
        .menu(menu::init())
        .title("ChatGPT")
        .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36 Edg/111.0.0.0")
        .build()?;
    Ok(())
}