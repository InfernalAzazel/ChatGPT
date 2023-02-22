use std::borrow::BorrowMut;
use tauri::{CustomMenuItem, Manager, Menu, MenuItem, Submenu, utils, WindowMenuEvent};
use crate::app::window;

pub fn init() -> Menu {
    let bing = CustomMenuItem::new("BingAI".to_string(), "BingAI");
    let gpt = CustomMenuItem::new("ChatGPT".to_string(), "ChatGPT");
    let quit = CustomMenuItem::new("Quit".to_string(), "Quit");
    let chat_ai_submenu = Submenu::new(
        "ChatAI", Menu::new()
            .add_item(bing)
            .add_item(gpt)
            .add_item(quit),
    );
    let settings =  CustomMenuItem::new("Settings".to_string(), "Settings");
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_submenu(chat_ai_submenu)
        .add_item(settings);
    menu
}

pub fn on_event(event: WindowMenuEvent<tauri::Wry>) {
    let win = Some(event.window()).unwrap();
    let app = win.app_handle();
    match event.menu_item_id() {
        "BingAI" => {
            println!("666-------------555");
            let main_window = app.get_window("main").unwrap();
            main_window.set_title("6666").unwrap();
            let js = "
            // var customUserAgent = 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36 Edg/111.0.0.0';
            // alert('修改前：'+ window.navigator.userAgent);
            // Object.defineProperty(navigator, 'userAgent', {
            //     value: customUserAgent,
            //     writable: false
            // });
            // alert('修改后：'+window.navigator.userAgent);";
            // main_window.eval(js).unwrap();
            main_window.eval("window.location.replace('https://www.bing.com/search?q=Bing+AI&showconv=1&FORM=hpcodx&fbg=0')").unwrap();
        }
        "ChatGPT" => {
           let main_window = app.get_window("main").unwrap();
            main_window.eval("window.location.replace('https://chat.openai.com/chat')").unwrap();
        }
        "Quit" => {
            std::process::exit(0);
        }
        "Settings" => {
            window::open_settings(app);
        }
        _ => {}
    }
}