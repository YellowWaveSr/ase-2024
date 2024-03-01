use std::f32::consts::PI;

use crate::lfo::LFO;
use crate::ring_buffer::RingBuffer;
// The vibrato generator consists three main components
// First, the parameters that determine the characteristics of the vibration.
// These parameters include the sample rate, modulation frequency, vibration width.
// In a digital computer, time is represented using the indices instead of seconds.
// Therefore, we also convert these numbers into the number of samples (indices).
// Second, the vobrato generator also needs the sinusoidal wave generator.
// This generator determines the (fractional) delay that we want to use.
// Third, the ring_buffer is needed so that we can pick the correct signal. 
pub struct vibrato_generator{
    sample_rate : i32,
    mod_freq : f32, 
    vib_width : f32,
    delay_sample : i32,
    width_sample : i32, 
    lfo_generator : LFO,
    buffer_length : usize, 
    ring_buffer : RingBuffer<f32>,
}

impl vibrato_generator {
    pub fn new(sample_rate : i32, mod_freq : f32, vib_width : f32, vib_amplitude : f32) -> Self
    {
        let width_sample = (vib_width * sample_rate as f32).round() as i32;
        let delay_sample = (vib_width * sample_rate as f32).round() as i32;
        let buffer_length = (delay_sample + width_sample + 10) as usize;
        let mut ring_buffer : RingBuffer<f32> = RingBuffer::new(buffer_length);
        let mut lfo_generator = LFO::new(vib_amplitude, mod_freq * 2.0 * PI / sample_rate as f32, sample_rate);

        vibrato_generator{
            sample_rate,
            mod_freq, 
            vib_width,
            delay_sample,
            width_sample, 
            lfo_generator,
            buffer_length, 
            ring_buffer
        }

    }
    pub fn reset(&mut self)
    {
        self.ring_buffer.reset();
        self.lfo_generator.reset();
    }
    pub fn add_vibrato(&mut self, wav_data : &mut Vec<Vec<f32>>)
    {
        let channel_length = wav_data.len();
        if channel_length == 0
        {
            return;
        }

        let data_length = wav_data[0].len();
        if data_length == 0
        {
            return; 
        }
        for cur_channel in 0..channel_length
        {
            let cur_data = &mut wav_data[cur_channel];
            // fill up the buffer
            for data_index in ((data_length - self.buffer_length + 1)..(data_length)).rev()
            {
                self.ring_buffer.push(cur_data[data_index]);
            }

            for data_index in (0..data_length).rev()
            {
                let mod_scale = self.lfo_generator.get_sin_wave(data_index);
                let fraction_delay = 
                        (self.delay_sample as f32) + (self.width_sample as f32) * mod_scale; 
                //println!("{}, {}", fraction_delay, self.buffer_length);
                let delay_signal = self.ring_buffer.get_frac(fraction_delay);
                cur_data[data_index] = delay_signal;
                self.ring_buffer.pop();

                let next_signal_index = (data_index as i32) - (self.buffer_length as i32);                
                if next_signal_index >= 0
                {
                    self.ring_buffer.push(cur_data[next_signal_index as usize]);
                    //println!("{}", cur_data[next_signal_index as usize]);
                }
                else
                {
                    self.ring_buffer.push(0.0);
                }
            }
            
        }

    }
}


#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use super::*;

    #[test]
    fn test_zero_amplitude()
    {
        // if the amplitude is zero
        // then the output is pure delay
        let mut wav_data : Vec<Vec<f32>> = vec![vec![]];
        let mut wav_data_2 : Vec<Vec<f32>> = vec![vec![]];
        for t in 0..8000
        {
            let true_t = t as f32/8000.0;
            let cur_data = true_t.sin();
            wav_data[0].push(cur_data);
            wav_data_2[0].push(cur_data);
        }
        let mut vibrato_generator = vibrato_generator::new
                                            (8000 as i32, 5.0, 0.01, 0.0);

        vibrato_generator.add_vibrato(&mut wav_data);
        //println!("{}", vibrato_generator.delay_sample);
        for t in 0..2000
        {
            //println!("{},{}", wav_data[0][t], wav_data_2[0][t]);
            assert_eq!(wav_data[0][t + vibrato_generator.delay_sample as usize + 1], wav_data_2[0][t]);
        }
    }
    #[test]
    fn test_dc_input()
    {
        // test the dc input gives dc output
        // NOTE: this is true only if you ignore:
        // a) the leading zeros due to delay
        // b) the first non-zero element due to the interpolation. 
        let mut wav_data : Vec<Vec<f32>> = vec![vec![]];
        for t in 0..8000
        {
            let cur_data = 0.5;
            wav_data[0].push(cur_data);
        }
        let mut vibrato_generator = vibrato_generator::new
                                            (8000 as i32, 5.0, 0.01, 1.0);

        vibrato_generator.add_vibrato(&mut wav_data);
        //println!("{}", vibrato_generator.delay_sample);
        for t in 1000..2000
        {
            assert_eq!(wav_data[0][t], 0.5 as f32);
        }
    }
    
    #[test]
    fn test_zero_input()
    {
        // zero input gives zero output
        let mut wav_data : Vec<Vec<f32>> = vec![vec![]];
        for t in 0..8000
        {
            let cur_data = 0.0;
            wav_data[0].push(cur_data);
        }
        let mut vibrato_generator = vibrato_generator::new
                                            (8000 as i32, 5.0, 0.01, 0.0);

        vibrato_generator.add_vibrato(&mut wav_data);
        println!("{}", vibrato_generator.delay_sample);
        for t in 1000..2000
        {
            assert_eq!(wav_data[0][t], 0.0);
        }
    }

    #[test]
    fn test_block_size()
    {
        // in my implementation, the block size is set to the full length by default.
        // this is based on the following observation: if the input is spilted into blocks
        // then there will be extra zeros at the beginning of the block
        // this is due to the delay in this vibrato
        // Instead of testing blocks, I'll test different data length for this test function.
        let mut wav_data : Vec<Vec<f32>> = vec![vec![]];
        let mut wav_data_2 : Vec<Vec<f32>> = vec![vec![]];
        for t in 0..8000
        {
            let true_t = t as f32/8000.0;
            let cur_data = true_t.sin();
            wav_data[0].push(cur_data);
            
        }
        for t in 0..6000
        {
            let true_t = t as f32/8000.0;
            let cur_data = true_t.sin();
            wav_data_2[0].push(cur_data);
        }
        let mut vibrato_generator = vibrato_generator::new
                                            (8000 as i32, 5.0, 0.01, 1.0);

        vibrato_generator.add_vibrato(&mut wav_data);
        vibrato_generator.reset();
        vibrato_generator.add_vibrato(&mut wav_data_2);
        //println!("{}", vibrato_generator.delay_sample);
        for t in 0..2000
        {
            //println!("{},{}", wav_data[0][t], wav_data_2[0][t]);
            assert_eq!(wav_data[0][t], wav_data_2[0][t]);
        }
    }


    #[test]
    fn test_empty_input()
    {
        // one extra test:
        // if the input is empty, then the processor does not do anything.
        let mut wav_data : Vec<Vec<f32>> = vec![vec![]];
        
        let mut vibrato_generator = vibrato_generator::new
                                            (8000 as i32, 5.0, 0.01, 1.0);

        vibrato_generator.add_vibrato(&mut wav_data);
        
       assert_eq!(wav_data.len(), 1);
       assert_eq!(wav_data[0].len(), 0);

    }
    #[test]
    fn test_max_input()
    {
        // another extra test:
        // the output contains only the numbers in the original signal if the amplitude is zero.
        // again the leading zeros should be ignored. 
        let mut wav_data : Vec<Vec<f32>> = vec![vec![]];
        
        for t in 0..8000
        {
            if t%2 == 0
            {
                wav_data[0].push(1.0);
            }
            else  
            {
                wav_data[0].push(-1.0);
            }
        }
        let mut vibrato_generator = vibrato_generator::new
                                            (8000 as i32, 5.0, 0.01, 0.0);

        vibrato_generator.add_vibrato(&mut wav_data);
        println!("{}", vibrato_generator.delay_sample);
        for t in 1000..2000
        { 
            //println!("{}", wav_data[0][t]);
            assert_eq!(wav_data[0][t].abs(), 1.0);
        }
        
    }



}
