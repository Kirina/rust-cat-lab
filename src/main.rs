mod cat;

use cat::Cat;
use cat::gender::Gender;
use rand::Rng; 



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