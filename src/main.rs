// mod coat_genes;
// mod gender;
// use gender::Gender;
// use coat_genes::CoatGenes;
use rand::Rng; 


// Import the Gender enum from the gender module

#[derive(Debug, Clone)]
enum BlackAllele {
    DominantB,
    RecessiveB,
    RecessiveBDash,
}

impl BlackAllele {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        
        match rng.gen_range(0..3) {
            0 => BlackAllele::DominantB,
            1 => BlackAllele::RecessiveB,
            _ => BlackAllele::RecessiveBDash,
        }
    }
    fn to_string(&self) -> String {
        match self {
            BlackAllele::DominantB => "B (Dominant Black)".to_string(),
            BlackAllele::RecessiveB => "b (Recessive Black)".to_string(),
            BlackAllele::RecessiveBDash => "b' (Chocolate/Cinnamon)".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
enum WhiteAllele {
    DominantW,
    RecessiveW,
}

impl WhiteAllele {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        
        match rng.gen_range(0..2) {
            0 => WhiteAllele::DominantW,
            _ => WhiteAllele::RecessiveW,
        }
    }
    fn to_string(&self) -> String {
        match self {
            WhiteAllele::DominantW => "W (Dominant White)".to_string(),
            WhiteAllele::RecessiveW => "w (Recessive White)".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
enum OrangeAllele {
    DominantO,
    RecessiveO,
}

impl OrangeAllele {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        
        match rng.gen_range(0..2) {
            0 => OrangeAllele::DominantO,
            _ => OrangeAllele::RecessiveO,
        }
    }
    fn to_string(&self) -> String {
        match self {
            OrangeAllele::DominantO => "O (Orange)".to_string(),
            OrangeAllele::RecessiveO => "o (Non-Orange)".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
struct CoatGenes {
    black_first_allele: BlackAllele,
    black_second_allele: BlackAllele, 
    white_first_allele: WhiteAllele,
    white_second_allele: WhiteAllele,
    orange_first_x_chromosome: OrangeAllele,
    orange_second_x_chromosome: Option<OrangeAllele>,
}

impl CoatGenes {
    // Male cats have no second x chromosome, so second is optional
    fn random(gender: &Gender) -> Self {
        let orange_second_x_chromosome = if *gender == Gender::Male {
            None
        } else {
            Some(OrangeAllele::random())
        };
        CoatGenes{
            black_first_allele: BlackAllele::random(),
            black_second_allele: BlackAllele::random(), 
            white_first_allele: WhiteAllele::random(),
            white_second_allele: WhiteAllele::random(),
            orange_first_x_chromosome: OrangeAllele::random(),
            orange_second_x_chromosome: orange_second_x_chromosome,
        }
    }
    fn to_string(&self) -> String {
        let black_info = format!("Black genes: {} and {}", 
            self.black_first_allele.to_string(), 
            self.black_second_allele.to_string());
            
        let white_info = format!("White genes: {} and {}", 
            self.white_first_allele.to_string(), 
            self.white_second_allele.to_string());
            
        let orange_info = match &self.orange_second_x_chromosome {
            Some(second_allele) => format!("Orange genes: {} and {}", 
                self.orange_first_x_chromosome.to_string(), 
                second_allele.to_string()),
            None => format!("Orange genes: {} (male, single X chromosome)", 
                self.orange_first_x_chromosome.to_string()),
        };
        
        format!("{}\n{}\n{}", black_info, white_info, orange_info)
    }
}

#[derive(Debug, Clone, PartialEq)]
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
    coat_genes: CoatGenes,
    gender: Gender,
}

impl Cat {
    fn random_cat(name: String, age: u8, gender_option: Option<Gender>) -> Self {
        let gender = gender_option.unwrap_or_else(Gender::random);
        Cat {
            name,
            age,
            coat_genes: CoatGenes::random(&gender),
            gender,                      
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