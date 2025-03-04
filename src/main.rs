use rand::Rng; 

#[derive(Debug, Clone)]
struct Coat {
    colour: String,
    length: String,
}

impl Coat {
    fn random() -> Self {
        Coat {
            colour: "Purple".to_string(),
            length: "Short".to_string(),
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
        
        if rng.gen_bool(0.5) {
            Gender::Female
        } else {
            Gender::Male
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
    coat: Coat,
}

impl Cat {
    // Method to generate a random cat with randomly assigned gender
    fn random_cat(name: String, age: u8, breed: String) -> Self {
        let colour = "Purple";
        let length = "Short";
        
        Cat {
            name,
            age,
            breed,
            gender: Gender::random(),
            coat: Coat::random()
        }
    }

    // Method to get cat information
    fn get_info(&self) -> String {
        format!(
            "Name: {}, Age: {}, Breed: {}, Gender: {}, Coat colour: {}, Coat length: {}",
            self.name, 
            self.age, 
            self.breed, 
            self.gender.to_string(),
            self.coat.colour,
            self.coat.length,
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

    // Generate a vector of random cats
    (0..num_cats).map(|i| {
        Cat::random_cat(
            format!("Cat {}", i + 1),  // Generate sequential names
            rng.gen_range(0..15),      // Random age between 0-14
            breeds[rng.gen_range(0..breeds.len())].to_string()  // Random breed
        )
    }).collect()
}

fn main() {
    // Generate 5 random cats
    let cats = generate_random_cats(5);
    
    // Print information about each cat
    for cat in cats {
        println!("{}", cat.get_info());
    }
}