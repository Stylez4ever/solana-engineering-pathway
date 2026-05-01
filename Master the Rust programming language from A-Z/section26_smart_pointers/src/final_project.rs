use std::error::Error;
use std::fmt::{Display, write};

trait TextTransformer {
    fn transform(&self, text: &str) -> Result<String, Box<dyn Error>>;
}

struct WhitespaceTransformer {
    start: bool,
    end: bool,
}

#[derive(Debug)]
struct PizzaEmojiInString;

impl Display for PizzaEmojiInString {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "hey, there's a pizza emoji in the text -- so cheesy.")
    }
}

#[derive(Debug)]
struct NoContentInString;

impl Display for NoContentInString {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
         write!(formatter, "The string has nothing left in it.")
    }
}

impl Error for NoContentInString {}

impl Error for PizzaEmojiInString {}

impl TextTransformer for WhitespaceTransformer {
    fn transform(&self, text: &str) -> Result<String, Box<dyn Error>> {
        if text.contains("🍕") {
            return  Err(Box::new(PizzaEmojiInString));
        }

        let transformed = if self.start && self.end {
            text.trim()
        } else if self.start {
            text.trim_start()
        } else if self.end {
            text.trim_end()
        } else {
            text
        };

        if transformed.is_empty() {
            return Err(Box::new(NoContentInString));
        }

        Ok(transformed.to_string())
    }
}

    enum Case {
        Uppercase,
        Lowercase,
    }

    struct CaseTransformer {
        case: Case,
    }

    impl TextTransformer for CaseTransformer {
        fn transform(&self, text: &str) -> Result<String, Box<dyn Error>> {
            match self.case {
                Case::Uppercase => Ok(text.to_uppercase()),
                Case::Lowercase => Ok(text.to_lowercase())
            }
        }
    }


fn main() {
    // Input
    //let text = String::from("  homer simpson  ");
    // Output
    

    // Content: "HOMER SIMPSON"
 
    // Input
    //let text = String::from("  data  🍕  ");
    // Output
    // Error Message: Something went wrong: Hey, there's a pizza emoji in the text. So cheesy. Moving on to next transform
    // Content: "  DATA  🍕  "
 
    // Input
    let text = String::from("    ");
    // Output:
    // Error Message: Something went wrong: The string has nothing left in it. Moving on to next transform
    // Content: "    "
 
    let pipeline: Vec<Box<dyn TextTransformer>> = vec![
        Box::new(WhitespaceTransformer {
            start: true,
            end: true,
        }),
        Box::new(CaseTransformer {
            case: Case::Uppercase,
        }),
    ];
 
    let transformed_text = apply_transformations(text, pipeline);
    println!("Output: {transformed_text}");
}

fn apply_transformations(text: String, pipeline: Vec<Box<dyn TextTransformer>>) -> String {
    pipeline.iter().into_iter().fold(text, |accumulator, transformer| {
        match transformer.transform(&accumulator) {
            Ok(new_value) => new_value,
            Err(error) => {
                eprintln!("Something went wrong: {error} Moving on to next transform");
                accumulator
            }
        }
    })
}