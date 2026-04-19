#[cfg(test)]


use pretty_assertions::{assert_eq, assert_ne};
use crate::final_project::*;

#[test]
fn salad_field_correctly_populated() {
    let salads = Salad::new(
        Protein::CrispyChicken,
         vec![Vegetable::Tomato], 
        Dressing::Italian);

    let salads_2 = Salad::new(
        Protein::CrispyChicken,
         vec![Vegetable::Tomato], 
        Dressing::Italian);    

    assert_eq!(salads, salads_2);  
}

#[test]
fn checking_if_salads_has_vegies() {
    let salads = Salad::new(
        Protein::CrispyChicken,
         vec![Vegetable::Tomato, Vegetable::Cucumber], 
        Dressing::Italian);

    let is_valid_test = salads.is_valid();
    
    assert!(is_valid_test);

}


#[test]
fn calculate_total_calories_in_salad() {
    let salads = Salad::new(
        Protein::CrispyChicken,
         vec![Vegetable::Tomato, Vegetable::Cucumber], 
        Dressing::Italian);
    
    let cal = salads.calories();
    assert_eq!(cal, 565);
}

#[test]
fn salads_includes_any_vegetable_more_than_once() {
    let salads = Salad::new(
        Protein::CrispyChicken,
         vec![Vegetable::Tomato, Vegetable::Cucumber], 
        Dressing::Italian);

    let duplicate = salads.has_duplicate_vegetables();
    assert!(duplicate);
}



