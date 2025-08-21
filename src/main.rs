use std::io::{self, Write};

fn main() {
    println!("Welcome to the Rust Test Word Game!");
    println!("Try to guess a 5-letter word. Type 'quit' to exit.");

    let target_word = "HELLO";
    let mut attempts = 0;
    let max_attempts = 6;

    loop {
        print!("Enter your guess: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let guess = input.trim().to_uppercase();

        if guess == "QUIT" {
            println!("Thanks for playing!");
            break;
        }

        if guess.len() != 5 {
            println!("Please enter a 5-letter word.");
            continue;
        }

        attempts += 1;
        let result = check_word(&guess, target_word);
        println!("Result: {}", result);

        if guess == target_word {
            println!(
                "Congratulations! You guessed the word in {} attempts!",
                attempts
            );
            break;
        }

        if attempts >= max_attempts {
            println!("Game over! The word was: {}", target_word);
            break;
        }

        println!("Attempts remaining: {}", max_attempts - attempts);
    }
}

fn check_word(guess: &str, target: &str) -> String {
    let target_chars: Vec<char> = target.chars().collect();
    let guess_chars: Vec<char> = guess.chars().collect();
    let mut result = vec!['❌'; 5];
    let mut target_used = vec![false; 5];

    // First pass: mark exact matches
    for i in 0..5 {
        if guess_chars[i] == target_chars[i] {
            result[i] = '✅';
            target_used[i] = true;
        }
    }

    // Second pass: mark partial matches
    for i in 0..5 {
        if result[i] == '❌' {
            for j in 0..5 {
                if !target_used[j] && guess_chars[i] == target_chars[j] {
                    result[i] = '🟡';
                    target_used[j] = true;
                    break;
                }
            }
        }
    }

    result.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match() {
        let result = check_word("HELLO", "HELLO");
        assert_eq!(result, "✅✅✅✅✅");
    }

    #[test]
    fn test_no_match() {
        let result = check_word("ABCDE", "FGHIJ");
        assert_eq!(result, "❌❌❌❌❌");
    }

    #[test]
    fn test_partial_match() {
        let result = check_word("HLELX", "HELLO");
        // H matches exactly, L in pos 1 is in word but wrong position,
        // E in pos 2 is in word but wrong position, L in pos 3 matches exactly, X is not in word
        assert_eq!(result, "✅🟡🟡✅❌");
    }

    #[test]
    fn test_mixed_match() {
        let result = check_word("LLEHO", "HELLO");
        // L in pos 0 is in word but wrong position, L in pos 1 is in word but wrong position,
        // E in pos 2 is in word but wrong position, H in pos 3 is in word but wrong position, O matches exactly
        assert_eq!(result, "🟡🟡🟡🟡✅");
    }
}
