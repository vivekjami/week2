use std::io::{self, Write};

// Define the WordCounter struct
struct WordCounter {
    text: String,
}

// Implement the functions for WordCounter
impl WordCounter {
    // Constructor
    fn new(text: &str) -> WordCounter {
        WordCounter {
            text: String::from(text),
        }
    }

    // Function to count words
    fn count_words(&self) -> usize {
        // Split the text into words based on whitespace characters
        let words: Vec<&str> = self.text.split_whitespace().collect();
        words.len()
    }
}

fn main() {
    // Prompt user for text input
    print!("Enter some text: ");
    io::stdout().flush().unwrap();

    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("Failed to read line");

    // Remove leading and trailing whitespaces
    let input_text = input_text.trim();

    // Check if the text is empty
    if input_text.is_empty() {
        println!("Error: Text is empty.");
        return;
    }

    // Create a WordCounter instance using the input text
    let word_counter = WordCounter::new(&input_text);

    // Call the count_words function
    let word_count = word_counter.count_words();

    // Print the obtained word count to the screen
    println!("Word count: {}", word_count);
}

// Create a struct named WordCounter.

// In the WordCounter struct, have the following field:

// text (text to count words from)

// Implement the following functions for WordCounter:

// new(text: &str) -> WordCounter: This function should take a text and create an instance of the WordCounter struct.

// count_words() -> usize: This function should count the number of words in the text and return the result as an integer. A word is defined as a string separated by whitespace characters.

// In the main function, prompt the user to enter a text.

// In the count_words function, check if the text is empty. If it is empty, return an error message. 
 

// Create an instance of WordCounter using the text entered by the user.

// Call the count_words function on the created WordCounter instance.

// Print the obtained word count to the screen.

// Compile and run the program to test its functionality.

// Checklist:

// Define a struct named WordCounter.

// In the WordCounter struct, define a field named text.

// Implement the new function.

// Implement the count_words function.

// Prompt the user for text input.

// Create a WordCounter instance using the input text.

// Call the count_words function and print the word count to the screen.

// Compile and run the program to test its functionality.