use crate::db::Database;
use std::process::Command;

#[derive(Debug,Default)]
pub struct App {
    pub counter: u8,
    pub barcode_input: String,
    pub should_quit: bool,
    pub new_barcode: bool,

}
// Calling App::default() will create an App initialized with counter set to 0, and exit set to false.
fn speak(text: &str) -> std::io::Result<()> {
    let status = Command::new("espeak")
    .arg("-vda")
    .arg(text)
    .status()?;

    if !status.success() {
        eprintln!("espeak exited with status: {status}");
    }
    Ok(())
}

impl App {

    pub fn new() ->Self{
        Self::default()
    }
    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) =self.counter.checked_add(1){
            self.counter = res;
        }
        else {
            self.counter = u8::MIN;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) =self.counter.checked_sub(1){
            self.counter = res;
        }
        else{
            self.counter = u8::MAX;
        }

    }

    pub fn process_barcode(&mut self, db: &Database){

        match db.get_name(&self.barcode_input){
            Ok(Some(name)) => {
                println!("{name}");
                speak(&name);
            },
            Ok(None) => {
                self.new_barcode = true;
                println!("Ny stregcode\n{}",self.barcode_input);
                speak("Jeg kender ikke den stregkode");
            },
            Err(error) => {
                eprintln!("Database error: {error}");
            }
        }

        self.barcode_input.clear();
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_app_increment_counter() {
        let mut app = App::default();
        app.increment_counter();
        assert_eq!(app.counter, 1);
    }

    #[test]
    fn test_app_decrement_counter() {
        let mut app = App::default();
        app.decrement_counter();
        assert_eq!(app.counter, 0);
    }
}
