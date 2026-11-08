fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_location = "content.md";
    let output_location = "output.md";

    // Get the contents of the input file
    let contents = std::fs::read_to_string(input_location)?; // Need to change this in some way for usage in CLI

    // Process the file and turn markdown into HTML
    let html = convert(contents);

    // Output the html file (need to find a way to save it as a file)
    std::fs::write(output_location, html)?;
    Ok(())
}

fn convert(input: String) -> String {
    let output = input;
    return output.to_string();
}
