extern crate csv;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use serde::{Serialize, Deserialize};
use std::error::Error;
use std::fs::{OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::vec::Vec;

#[derive(Debug, Serialize, Deserialize)]
struct Patient {
    name: String,
    age: u32,
    disease: String,
    contact: String,
    admission_date: String,
}

// Function to write the patient details to a CSV file
fn write_to_csv(patient: &Patient, file_path: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(file_path);

    // Open the file in append mode, or create it if it doesn't exist
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(OpenOptions::new().create(true).append(true).open(path)?);

    wtr.serialize(patient)?;
    Ok(())
}

// Function to read patient input from the user
fn get_patient_details() -> Patient {
    let mut name = String::new();
    let mut age = String::new();
    let mut disease = String::new();
    let mut contact = String::new();
    let mut admission_date = String::new();

    println!("Enter patient details");

    print!("Name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();
    
    print!("Age: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut age).unwrap();
    let age: u32 = age.trim().parse().unwrap();

    print!("Disease: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut disease).unwrap();

    print!("Contact: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut contact).unwrap();

    print!("Admission Date (YYYY-MM-DD): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut admission_date).unwrap();

    Patient {
        name: name.trim().to_string(),
        age,
        disease: disease.trim().to_string(),
        contact: contact.trim().to_string(),
        admission_date: admission_date.trim().to_string(),
    }
}

// Function to search for patients by name or contact number
fn search_patients(file_path: &str, search_term: &str) -> Result<Vec<Patient>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_path(file_path)?;
    let mut results: Vec<Patient> = Vec::new();

    for result in rdr.deserialize() {
        let record: Patient = result?;
        if record.name.contains(search_term) || record.contact.contains(search_term) {
            results.push(record);
        }
    }

    Ok(results)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Define file path for storing patient data
    let file_path = "patients.csv";
    
    loop {
        // Display menu for user
        println!("Healthcare Registration System");
        println!("1. Register New Patient");
        println!("2. Search Patient");
        println!("3. Exit");

        print!("Select an option: ");
        io::stdout().flush().unwrap();
        let mut option = String::new();
        io::stdin().read_line(&mut option).unwrap();

        match option.trim() {
            "1" => {
                // Register a new patient
                let patient = get_patient_details();
                write_to_csv(&patient, file_path)?;
                println!("Patient details saved successfully!");
            }
            "2" => {
                // Search for patient by name or contact
                print!("Enter patient name or contact to search: ");
                io::stdout().flush().unwrap();
                let mut search_term = String::new();
                io::stdin().read_line(&mut search_term).unwrap();
                let search_term = search_term.trim();

                let results = search_patients(file_path, search_term)?;

                if results.is_empty() {
                    println!("No patients found matching '{}'.", search_term);
                } else {
                    println!("Found {} patient(s):", results.len());
                    for patient in results {
                        println!("{:?}", patient);
                    }
                }
            }
            "3" => {
                // Exit the program
                println!("Exiting system...");
                break;
            }
            _ => {
                println!("Invalid option. Please try again.");
            }
        }
    }

    Ok(())
}
