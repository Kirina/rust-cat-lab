use crate::cat::Gender;
use rand::Rng; 

#[derive(Debug, Clone)]
pub enum BlackAllele {
    DominantB,
    RecessiveB,
    RecessiveBDash,
}

impl BlackAllele {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        
        match rng.gen_range(0..3) {
            0 => BlackAllele::DominantB,
            1 => BlackAllele::RecessiveB,
            _ => BlackAllele::RecessiveBDash,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            BlackAllele::DominantB => "B (Dominant Black)".to_string(),
            BlackAllele::RecessiveB => "b (Recessive Black)".to_string(),
            BlackAllele::RecessiveBDash => "b' (Chocolate/Cinnamon)".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum WhiteAllele {
    DominantW,
    RecessiveW,
}

impl WhiteAllele {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        
        match rng.gen_range(0..2) {
            0 => WhiteAllele::DominantW,
            _ => WhiteAllele::RecessiveW,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            WhiteAllele::DominantW => "W (Dominant White)".to_string(),
            WhiteAllele::RecessiveW => "w (Recessive White)".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum OrangeAllele {
    DominantO,
    RecessiveO,
}

impl OrangeAllele {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        
        match rng.gen_range(0..2) {
            0 => OrangeAllele::DominantO,
            _ => OrangeAllele::RecessiveO,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            OrangeAllele::DominantO => "O (Orange)".to_string(),
            OrangeAllele::RecessiveO => "o (Non-Orange)".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CoatGenes {
    black_first_allele: BlackAllele,
    black_second_allele: BlackAllele, 
    white_first_allele: WhiteAllele,
    white_second_allele: WhiteAllele,
    orange_first_x_chromosome: OrangeAllele,
    orange_second_x_chromosome: Option<OrangeAllele>,
}

impl CoatGenes {
    // Male cats have no second x chromosome, so second is optional
    pub fn random(gender: &Gender) -> Self {
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
    pub fn to_string(&self) -> String {
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