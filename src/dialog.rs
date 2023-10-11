use rusnap::{
    handler,
    snap::{alert, confirm, notify, prompt, ui, NotifyType, UiComponent},
};

pub fn build_ui() -> UiComponent {
    ui::panel([
        ui::heading("heading"),
        ui::divider(),
        ui::text("Text"),
        ui::divider(),
        ui::copyable("Copyable"),
        ui::divider(),
        ui::spinner(),
    ])
}

#[handler]
pub async fn handle_alert() {
    alert(build_ui()).await.unwrap()
}

#[handler]
pub async fn handle_confirm() -> bool {
    confirm(build_ui()).await.unwrap()
}

#[handler]
pub async fn handle_promat() -> Option<String> {
    prompt(build_ui(), "Hello").await.unwrap()
}

#[handler]
pub async fn handle_notify() {
    notify(NotifyType::Native, "Hello RuSnap").await.unwrap();
}
