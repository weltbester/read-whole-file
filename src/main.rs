use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string("test.txt")?;
    
    // Process the entire file content
    println!("Total characters: {}", content.len());
    println!("Total lines: {}", content.lines().count());
    
    // Process line by line
    for (line_number, line) in content.lines().enumerate() {
        // Your line processing logic here
        println!("Line {}: {}", line_number + 1, line);
        
        // Example: count words per line
        let word_count = line.split_whitespace().count();
        println!("  Words: {}", word_count);
    }
    
    Ok(())
}
