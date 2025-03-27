use crate::interfaces::file_chooser::FileChooserOptions;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
#[derive(Debug, Default)]
pub struct FileChooserHandler {
    pub file_chooser_open: bool,
}

impl FileChooserHandler {
    pub fn default() -> Self {
        Self {
            file_chooser_open: false,
        }
    }

    pub fn handle_request(&mut self, options: &FileChooserOptions) {
        match options {
            FileChooserOptions::OpenFile(_) => {
                self.file_chooser_open = true;
                println!("Handling Open File Request... in handler i got the message hehe");
            }
            FileChooserOptions::SaveFile(opts) => {
                self.file_chooser_open = true;
                // dbg!(&opts);
                println!("Handling Save File Request...in handler i got the message hehe");
            }
            FileChooserOptions::SaveFiles(opts) => {
                self.file_chooser_open = true;
                println!(
                    "Handling Save Multiple Files Request...in handler i got the message hehe"
                );
            }
        }
    }
}
