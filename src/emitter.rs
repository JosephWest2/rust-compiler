use std::{
    fs::File,
    io::{Error, Write},
};

pub struct Emitter {
    output_buffer: String,
}

impl Emitter {
    pub fn emit_to_buffer(&mut self, code: &str) {
        self.output_buffer.push_str(code);
    }
    pub fn write_buffer_to_file(&self, file_name: &str) -> Result<(), Error> {
        let file = File::create(file_name);
        if file.is_err() {
            let err = file.unwrap_err();
            eprintln!("{}", err);
            return Err(err);
        }
        let res = file.unwrap().write(self.output_buffer.as_bytes());
        if res.is_err() {
            let err = res.unwrap_err();
            eprintln!("{}", err);
            return Err(err);
        }
        return Ok(());
    }
    pub fn new() -> Self {
        Emitter {
            output_buffer: String::new(),
        }
    }
}
