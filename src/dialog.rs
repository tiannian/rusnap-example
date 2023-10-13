use rusnap::{
    api::{alert, confirm, notify, prompt, ui, NotifyType, Result, UiComponent},
    exports::handler,
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
pub async fn handle_alert() -> Result<()> {
    alert(build_ui()).await?;
    Ok(())
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
