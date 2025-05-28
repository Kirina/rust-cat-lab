use rand::Rng; 

#[derive(Debug, Clone, PartialEq)]
pub enum Gender {
    Male,
    Female,
}

impl Gender {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        
        match rng.gen_range(0..2) {
            0 => Gender::Male,
            _ => Gender::Female,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Gender::Male => "Male".to_string(),
            Gender::Female => "Female".to_string(),
        }
    }
}