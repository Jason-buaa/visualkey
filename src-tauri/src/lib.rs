// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::{
    menu::{Menu, MenuItem},
    Manager,
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    async_runtime::spawn,
    AppHandle,
    State,
};
use std::sync::Mutex;
use tokio::time::{sleep, Duration};
struct SetupState {
    frontend_task: bool,
    backend_task: bool,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
// 一个用于设置初始化任务状态的自定义任务
#[tauri::command]
async fn set_complete(
    app: AppHandle,
    state: State<'_, Mutex<SetupState>>,
    task: String,
) -> Result<(), ()> {
    // 以只读方式锁定 `State`
    let mut state_lock = state.lock().unwrap();
    match task.as_str() {
        "frontend" => state_lock.frontend_task = true,
        "backend" => state_lock.backend_task = true,
        _ => panic!("invalid task completed!"),
    }
    // 检查两个任务是否都已完成
    if state_lock.backend_task && state_lock.frontend_task {
        // 设置都已完成，我们可以关闭启动画面并且显示 main 窗口了
        let splash_window = app.get_webview_window("splashscreen").unwrap();
        let main_window = app.get_webview_window("main").unwrap();
        splash_window.close().unwrap();
        main_window.show().unwrap();
    }
    Ok(())
}

// 一个异步函数，用于执行一些耗时的设置任务
async fn setup(app: AppHandle) -> Result<(), ()> {
    // 模拟执行一些耗时的设置任务，3秒后完成
    println!("Performing really heavy backend setup task...");
    sleep(Duration::from_secs(3)).await;
    println!("Backend setup task completed!");
    // 设置后端任务为已完成
    // 可以像普通函数一样运行命令，但需要自己处理输入参数
    set_complete(
        app.clone(),
        app.state::<Mutex<SetupState>>(),
        "backend".to_string(),
    )
    .await?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
            // 注册一个由 Tauri 管理的 `State`
        // 我们需要对它拥有写访问权限，因此我们将其包裹在 `Mutex` 中
        .manage(Mutex::new(SetupState {
            frontend_task: false,
            backend_task: false,
        }))
        // 添加我们用于检查的命令
        .invoke_handler(tauri::generate_handler![greet, set_complete])
        // 使用 setup 钩子来执行设置相关任务
        // 在主循环之前运行，因此尚未创建窗口
        .setup(|app| {
            // Spawn 操作设置为一个非阻塞任务，以便在它执行的同时可以创建并运行窗口。
            spawn(setup(app.handle().clone()));
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;
            let tray = TrayIconBuilder::new()
            .icon(app.default_window_icon().unwrap().clone())
            .menu(&menu)
            .show_menu_on_left_click(true)
            .on_menu_event(|app, event| match event.id.as_ref() {
                "quit" => {
                    println!("quit menu item was clicked");
                    app.exit(0);
                }
                _ => {
                    println!("menu item {:?} not handled", event.id);
                }    
            })
            .build(app)?;
            Ok(())
        })
        .on_tray_icon_event(|tray, event| match event {
            TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } => {
                println!("left click pressed and released");
                // 在这个例子中，当点击托盘图标时，将展示并聚焦于主窗口
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            _ => {
                println!("unhandled event {event:?}");
            }
        })
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
