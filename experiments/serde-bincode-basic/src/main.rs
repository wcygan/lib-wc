use serde_derive::{Serialize, Deserialize};

fn main() -> Result<(), bincode::Error> {
    // Create the struct
    let chicago = City {
        name: "Chicago".into(),
        country: Country::US,
    };

    // Serialize it 
    let serialized = bincode::serialize(&chicago)?;

    // Deserialize it
    let deserialized: City = bincode::deserialize(&serialized)?;

    println!("Debug:\n\t{:?}", chicago);
    println!("\nbincode serialized:\n\t{:?}", serialized);
    println!("\nbincode deserialized:\n\t{:?}", deserialized);

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct City {
    name: String,
    country: Country,
}

#[derive(Serialize, Deserialize, Debug)]
enum Country {
    US,
}