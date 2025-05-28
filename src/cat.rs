mod coat_genes;
pub mod gender;

use gender::Gender;
use coat_genes::CoatGenes;
use rand::Rng; 


#[derive(Debug, Clone)]
pub struct Cat {
    name: String,
    age: u8,
    coat_genes: CoatGenes,
    gender: Gender,
}

impl Cat {
    pub fn random_cat(name: String, age: u8, gender_option: Option<Gender>) -> Self {
        let actual_gender  = gender_option.unwrap_or_else(Gender::random);
        Cat {
            name,
            age,
            coat_genes: CoatGenes::random(&actual_gender ),
            gender: actual_gender,                      
        }
    }

    pub fn get_info(&self) -> String {
        format!(
            "Name: {}, Age: {}, Gender: {}, Coat colour: {}",
            self.name, 
            self.age, 
            self.gender.to_string(),
            self.coat_genes.to_string(),
        )
    }
    
    // Method to breed two cats and produce an offspring
    pub fn breed(parent1: &Cat, parent2: &Cat, kitten_name: String) -> Result<Cat, &'static str> {
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