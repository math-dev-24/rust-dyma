use std::fs::{File, Metadata, OpenOptions};
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};

struct FileStruct {
    file: File,
}

impl FileStruct {
    fn new(path: &str) -> Self {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .unwrap();
        FileStruct { file }
    }
    fn get_content(&mut self) -> String {
        let mut content = String::new();
        self.file.read_to_string(&mut content).unwrap();
        content
    }
    fn write_all(&mut self, content: String) -> Result<(), std::io::Error> {
        self.file.set_len(0)?;
        self.file.write_all(content.as_bytes())
    }
    fn get_meta(&mut self) -> Result<Metadata, std::io::Error> {
        self.file.metadata()
    }
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    surname: String,
    age: u8,
    description: String
}

impl Person {
    fn encoded(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}


pub fn json_bin() -> std::io::Result<()> {
    let mut file = FileStruct::new("person.json");

    let person_1 = Person {
        name: String::from("John"),
        surname: String::from("Doe"),
        age: 20, description: String::from("Hello World!")
    };

    file.write_all(person_1.encoded())?;

    Ok(())
}