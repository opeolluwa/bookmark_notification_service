use super::handler::ProcessNotification;

pub struct Email {}


impl ProcessNotification for Email{
    fn process_notification(&self) -> Result<(), anyhow::Error> {
        todo!()
    }
}