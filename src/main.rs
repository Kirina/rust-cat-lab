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
    gender: Gender,
    coat: CoatColour,
}

impl Cat {
    // Method to generate a random cat with randomly assigned gender
    fn random_cat(name: String, age: u8, gender: Option<Gender>, coat: Option<CoatColour>) -> Self {
        Cat {
            name,
            age,
            gender: gender.unwrap_or_else(Gender::random),
            coat: coat.unwrap_or_else(CoatColour::random)
        }
    }

    // Method to get cat information
    fn get_info(&self) -> String {
        format!(
            "Name: {}, Age: {}, Gender: {}, Coat colour: {}",
            self.name, 
            self.age, 
            self.gender.to_string(),
            self.coat.to_string(),
        )
    }
    
    // Method to breed two cats and produce an offspring
    fn breed(parent1: &Cat, parent2: &Cat, kitten_name: String) -> Result<Cat, &'static str> {
        match (&parent1.gender, &parent2.gender) {
            (Gender::Male, Gender::Female) | (Gender::Female, Gender::Male) => {
            },
            _ => {
                return Err("Breeding requires one male and one female cat");
            }
        }
        
        let mut rng = rand::thread_rng();
        
        // Randomly determine coat color from parents
        let coat = if rng.gen_bool(0.5) {
            parent1.coat.clone()
        } else {
            parent2.coat.clone()
        };
        
        // Create kitten with random gender, age 0, and inherited characteristics
        Ok(Cat {
            name: kitten_name,
            age: 0,
            gender: Gender::random(),
            coat,
        })
    }
}

fn generate_random_cats(num_cats: usize) -> Vec<Cat> {
    let mut rng = rand::thread_rng();
    
    (0..num_cats).map(|i| {
        Cat::random_cat(
            format!("Cat {}", i + 1),  // Generate sequential names
            rng.gen_range(0..15),      
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
    
    // Create two cats for breeding
    let tom = Cat::random_cat(
        "Tom".to_string(), 
        3, 
        Some(Gender::Male), 
        Some(CoatColour::Orange)
    );
    
    let queen = Cat::random_cat(
        "Queen".to_string(), 
        2, 
        Some(Gender::Female), 
        Some(CoatColour::White)
    );
    
    println!("\nParents:");
    println!("Father: {}", tom.get_info());
    println!("Mother: {}", queen.get_info());
    
    // Breed the cats
    match Cat::breed(&tom, &queen, "Kitten".to_string()) {
        Ok(kitten) => {
            println!("\nKitten born!");
            println!("Kitten: {}", kitten.get_info());
        },
        Err(e) => {
            println!("Breeding failed: {}", e);
        }
    }
}