use std::{fs::File, io::Write};
use hound::{WavReader, WavSpec};
fn show_info() {
    eprintln!("MUSI-6106 Assignment Executable");
    eprintln!("(c) 2024 Stephen Garrett & Ian Clester");
}

fn main() {
   show_info();

    // Parse command line arguments
    // First argument is input .wav file, second argument is output text file.
    // let args: Vec<String> = std::env::args().collect();
    // TODO: your code here
    // Specify the path to the audio file
    let file_path = "sweep.wav";
    // Open the input wave file and determine number of channels
    // TODO: your code here; see `hound::WavReader::open`.
    // Use a Result to handle potential errors when opening the file
    if let Ok(reader) = WavReader::open(file_path) {
        // Extract the WavSpec to get information about the audio file
        let spec: WavSpec = reader.spec();

        // Print some information about the audio file
        println!("Channels: {}", spec.channels);
        println!("Sample rate: {}", spec.sample_rate);
        println!("Bits per sample: {}", spec.bits_per_sample);

        // Create a writer for the text file
        let mut text_writer = std::fs::File::create("output.txt").expect("Error creating text file");
        let mut result_left=0.0; 
        let mut result_right=0.0; 
        
        let mut count=0;
        // Iterate over the audio samples and write to the text file
        for sample_left in reader.into_samples::<i32>() {
            match sample_left {
                Ok(value) => {
                    count += 1;
                    
                    let float_value = value as f32;
                    let denominator = 32768.0;
                    // Perform division
                    let result = float_value / denominator;

                    if count % 2 == 1 {
                        result_left = result;
                    } else {
                        result_right = result;
                        writeln!(text_writer, "{:.6}\t{:.6}", result_left,result_right).expect("Error writing to text file");
                    }
                    
                }
                Err(e) => eprintln!("Error reading sample: {:?}", e),
            }
        }
        
    } else {
        eprintln!("Error opening the WAV file");
    }

    println!("done");
}
