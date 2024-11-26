pub trait ProcessNotification {
     fn process_notification(&self) -> Result<(), anyhow::Error>;
}
