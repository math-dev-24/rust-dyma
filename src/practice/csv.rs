use std::fs::{File, OpenOptions};
use csv::{ReaderBuilder, WriterBuilder};

struct Person {
    nom: String,
    age: u32,
}

impl Person {
    fn get_vec(&self) -> Vec<String> {
        vec![self.nom.clone(), self.age.to_string()]
    }
}

struct FileStruct {
    file: File,
}

impl FileStruct {
    fn new(path: &str) -> FileStruct {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(path)
            .unwrap();
        FileStruct { file }
    }

    fn print_record(&mut self) -> Result<(), csv::Error> {
        let mut rdr = ReaderBuilder::new().from_reader(&self.file);
        for result in rdr.records() {
            let record = result?;
            for field in record.iter() {
                print!("{} ", field);
            }
            println!();
        }
        Ok(())
    }

    fn add_data(&mut self, person: &Person) -> Result<(), csv::Error> {
        let mut wtr = WriterBuilder::new().from_writer(&self.file);
        wtr.write_record(person.get_vec())?;
        wtr.flush()?;
        Ok(())
    }
}

pub fn csv_practice() -> Result<(), csv::Error> {
    let mut file = FileStruct::new("data.csv");
    file.print_record().expect("Une erreur est survenue lors de l'affichage des enregistrements.");
    file.add_data(&Person {
        nom: String::from("Jean"),
        age: 30,
    })
        .expect("Une erreur est survenue lors de l'ajout de données.");
    Ok(())
}