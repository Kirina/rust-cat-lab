use rand::Rng; 

#[derive(Debug, Clone)]
enum CoatColour {
    Brown,
    Black,
    Orange,
    White,
}

impl CoatColour {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        
        match rng.gen_range(0..4) {
            0 => CoatColour::Brown,
            1 => CoatColour::Black,
            2 => CoatColour::Orange,
            _ => CoatColour::White,
        }
    }

    fn to_string(&self) -> String {
        match self {
            CoatColour::Brown => "Brown".to_string(),
            CoatColour::Black => "Black".to_string(),
            CoatColour::Orange => "Orange".to_string(),
            CoatColour::White => "White".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
enum Gender {
    Male,
    Female,
}

impl Gender {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        
        match rng.gen_range(0..2) {
            0 => Gender::Male,
            _ => Gender::Female,
        }
    }

    fn to_string(&self) -> String {
        match self {
            Gender::Male => "Male".to_string(),
            Gender::Female => "Female".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
struct Cat {
    name: String,
    age: u8,
    breed: String,
    gender: Gender,
    coat: CoatColour,
}

impl Cat {
    // Method to generate a random cat with randomly assigned gender
    fn random_cat(name: String, age: u8, breed: String, gender: Option<Gender>, coat: Option<CoatColour>) -> Self {
        Cat {
            name,
            age,
            breed,
            gender: gender.unwrap_or_else(Gender::random),
            coat: coat.unwrap_or_else(CoatColour::random)
        }
    }

    // Method to get cat information
    fn get_info(&self) -> String {
        format!(
            "Name: {}, Age: {}, Breed: {}, Gender: {}, Coat colour: {}",
            self.name, 
            self.age, 
            self.breed, 
            self.gender.to_string(),
            self.coat.to_string(),
        )
    }
}

fn generate_random_cats(num_cats: usize) -> Vec<Cat> {
    let mut rng = rand::thread_rng();
    
    // Array of possible cat breeds
    let breeds = [
        "Persian", 
        "Siamese", 
        "Maine Coon", 
        "Bengal", 
        "Russian Blue",
        "Sphynx",
        "British Shorthair"
    ];
    // let cat1 = Cat::random_cat("Whiskers".to_string(), 3, "Maine Coon".to_string(), None, None);
    // println!("{}", cat1.get_info());
    // // With specified gender
    // let cat2 = Cat::random_cat("Mittens".to_string(), 5, "Siamese".to_string(), Some(Gender::Female), None);
    // println!("{}", cat2.get_info());
    // // Generate a vector of random cats
    (0..num_cats).map(|i| {
        Cat::random_cat(
            format!("Cat {}", i + 1),  // Generate sequential names
            rng.gen_range(0..15),      // Random age between 0-14
            breeds[rng.gen_range(0..breeds.len())].to_string(),  // Random breed
            None, 
            None,
        )
    }).collect()
}

fn main() {
    // Generate 5 random cats
    let cats = generate_random_cats(5);

    for cat in cats {
        println!("{}", cat.get_info());
    }
}