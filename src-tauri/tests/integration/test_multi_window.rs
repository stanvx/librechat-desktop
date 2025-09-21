#[derive(Debug)]
pub struct WindowSpec {
    pub label: &'static str,
    pub title: &'static str,
    pub width: u32,
    pub height: u32,
}

#[tokio::test]
async fn quick_capture_window_can_be_spawned() {
    let spec = WindowSpec {
        label: "quick-capture",
        title: "Quick Capture",
        width: 420,
        height: 520,
    };

    let _handle = open_additional_window(spec).await;
}

#[tokio::test]
async fn closing_additional_window_returns_focus() {
    let spec = WindowSpec {
        label: "settings",
        title: "Settings",
        width: 640,
        height: 480,
    };

    let handle = open_additional_window(spec).await;
    close_window(handle).await;
}

pub struct WindowHandle {
    pub label: String,
}

async fn open_additional_window(_spec: WindowSpec) -> WindowHandle {
    todo!("Implement WindowManager::open to satisfy T017 integration test");
}

async fn close_window(_handle: WindowHandle) {
    todo!("Implement WindowManager::close to satisfy T017 integration test");
}
