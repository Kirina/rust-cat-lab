mod coat_genes;
mod gender;

use gender::Gender;
use coat_genes::CoatGenes;
use rand::Rng; 

#[derive(Debug, Clone)]
struct Cat {
    name: String,
    age: u8,
    coat_genes: CoatGenes,
    gender: Gender,
}

impl Cat {
    fn random_cat(name: String, age: u8, gender_option: Option<Gender>) -> Self {
        let actual_gender  = gender_option.unwrap_or_else(Gender::random);
        Cat {
            name,
            age,
            coat_genes: CoatGenes::random(&actual_gender ),
            gender: actual_gender,                      
        }
    }

    fn get_info(&self) -> String {
        format!(
            "Name: {}, Age: {}, Gender: {}, Coat colour: {}",
            self.name, 
            self.age, 
            self.gender.to_string(),
            self.coat_genes.to_string(),
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
        let coat_genes = if rng.gen_bool(0.5) {
            parent1.coat_genes.clone()
        } else {
            parent2.coat_genes.clone()
        };
        
        // Create kitten with random gender, age 0, and inherited characteristics
        Ok(Cat {
            name: kitten_name,
            age: 0,
            gender: Gender::random(),
            coat_genes,
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
        )
    }).collect()
}

fn main() {
    let cats = generate_random_cats(5);

    for cat in cats {
        println!("{}", cat.get_info());
    }
    
    // Create two cats for breeding
    let tom = Cat::random_cat(
        "Tom".to_string(), 
        3, 
        Some(Gender::Male), 
    );
    
    let queen = Cat::random_cat(
        "Queen".to_string(), 
        2, 
        Some(Gender::Female), 
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