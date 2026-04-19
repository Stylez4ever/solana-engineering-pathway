pub trait Caloric {
    fn calories(&self) -> u32;
}
 
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Vegetable {
    Tomato,
    Cucumber,
    SweetPotato,
}
 
impl Caloric for Vegetable {
    fn calories(&self) -> u32 {
        match self {
            Self::Tomato => 20,
            Self::Cucumber => 15,
            Self::SweetPotato => 100,
        }
    }
}
 
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Protein {
    CrispyChicken,
    FriedChicken,
    Steak,
    Tofu,
}
 
impl Caloric for Protein {
    fn calories(&self) -> u32 {
        match self {
            Self::CrispyChicken => 400,
            Self::FriedChicken => 500,
            Self::Steak => 300,
            Self::Tofu => 200,
        }
    }
}
 
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Dressing {
    Ranch,
    Vinaigrette,
    Italian,
}
 
impl Caloric for Dressing {
    fn calories(&self) -> u32 {
        match self {
            Self::Ranch => 150,
            Self::Vinaigrette => 120,
            Self::Italian => 130,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Salad {
    protein: Protein,
    vegetables: Vec<Vegetable>,
    dressing: Dressing,
}

impl  Salad {
    pub fn new(protein: Protein, vegetables: Vec<Vegetable>, dressing: Dressing) -> Salad {
        Salad { protein, vegetables, dressing }
    }

    pub fn is_valid(self: Salad) -> bool {
        if self.vegetables.len() > 0 {
            true
        } else {
            false
        }
    }

    pub fn has_duplicate_vegetables(self: Salad) -> bool {
        if self.vegetables.len() > 1 {
            true
        } else {
            false
        }

    }

}

impl Caloric for Salad {
     fn calories(&self) -> u32 {
        let protein_calories = self.protein.calories();
        let dressing_calories = self.dressing.calories();

        let vegies: u32 = self.vegetables
        .iter()
        .map(|value| value.calories()).sum();

        let total = protein_calories + dressing_calories + vegies;
        total
    }

}


