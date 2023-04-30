#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release


#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "restruct-studio",
        native_options,
        Box::new(|cc| Box::new(restruct_studio::RestructApp::new(cc))),
    )
}


#[cfg(target_arch = "wasm32")]
fn main() {

    console_error_panic_hook::set_once(); // no point in the .expect() if you can't see the message
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "restruct_studio",
            web_options,
            Box::new(|cc| Box::new(eframe_template::TemplateApp::new(cc))),
        )
        .await
        .expect("failed to start eframe");
    });
}
