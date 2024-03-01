use std::{fmt::write, fs::File, io::Write};

use hound::{SampleFormat, WavWriter};

use crate::vibrato::vibrato_generator;

mod ring_buffer;
mod vibrato;
mod lfo;

fn show_info() {
    eprintln!("MUSI-6106 Assignment Executable");
    eprintln!("(c) 2024 Stephen Garrett & Ian Clester");
}

fn main() {
    show_info();

    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input wave filename> <output text filename>", args[0]);
        return
    }

    // Open the input wave file
    let mut reader = hound::WavReader::open(&args[1]).unwrap();
    let spec = reader.spec();
    let channels = spec.channels;

    let mut wav_data : Vec<Vec<f32>> = vec![vec![];channels as usize];
    // put the data into a vector for easy process
    // each vector is a channel 
    for (i, sample) in reader.samples::<i16>().enumerate() {
        let sample = sample.unwrap() as f32 / (1 << 15) as f32;
        let cur_channel = i % channels as usize;
        wav_data[cur_channel].push(sample);
    }
    // check the size of the data
    let data_length = wav_data[0].len();
    let channel_length = wav_data.len();
    println!("Channel number: {}", channel_length);
    println!("Data length: {}", data_length);
    let sample_rate = spec.sample_rate;
    println!("Sample rate:{}", sample_rate);
    // only two parameters are needed: mod_freq in HZ, vib_width in second. 
    let mut vibrato_generator = vibrato::vibrato_generator::new
                                            (sample_rate as i32, 5.0, 0.01, 1.0);

    vibrato_generator.add_vibrato(&mut wav_data);


    //return;
    // Read audio data and write it to the output text file (one column per channel)
    /* 
    let mut out = File::create(&args[2]).expect("Unable to create file");
    for data_index in 0..data_length
    {
        for channel_index in 0..channel_length-1
        {
            write!(out, "{} ", wav_data[channel_index][data_index]).unwrap();
        }
        write!(out, "{}\n", wav_data[channel_length - 1][data_index]).unwrap();

    }
    */
    let writeSpec = hound::WavSpec {
        channels: channel_length as u16,
        sample_rate: sample_rate, 
        bits_per_sample: 32, 
        sample_format: SampleFormat::Float, // Integer samples
    };
    let mut writer = WavWriter::create(&args[2], writeSpec).unwrap();
    for data_index in 0..data_length
    {
        for channel_index in 0..channel_length
        {  
            let _ = writer.write_sample(wav_data[channel_index][data_index]);
        
        }
    }

    print!("{}", writer.duration());
    let _ = writer.finalize();

}
