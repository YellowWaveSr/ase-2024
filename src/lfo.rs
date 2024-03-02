use hound::Sample;

use crate::ring_buffer::{self, RingBuffer};
pub struct LFO {
    amplitude : f32,
    frequency : f32,
    sample_rate : i32,
    buffer_length : usize,
    ring_buffer : ring_buffer::RingBuffer<f32>,
}


impl LFO{
    pub fn new(amplitude : f32, frequency : f32, sample_rate : i32) -> Self
    {
        let buffer_length = (1.0/frequency * sample_rate as f32).round() as usize;
        let mut ring_buffer = ring_buffer::RingBuffer::new(buffer_length as usize);
        for cur_index in 0..buffer_length
        {
            ring_buffer.push((frequency * cur_index as f32).sin() * amplitude)
        }
        return LFO{
            amplitude,
            frequency,
            sample_rate,
            buffer_length, 
            ring_buffer,
        }
    }
    pub fn reset(&mut self)
    {
        self.ring_buffer.reset();
        for cur_index in 0..self.buffer_length
        {
            self.ring_buffer.push((self.frequency * cur_index as f32).sin() * self.amplitude)
        }
    }
    pub fn get_sin_wave(&mut self, index : usize) -> f32
    {
        let buffer_index = index % self.buffer_length;
        self.ring_buffer.get(buffer_index)
    }
}


#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use super::*;

    #[test]
    fn test_sin_wave()
    {
        let amplitude = 1.0;
        let frequency = 2.0 * PI;
        let sample_rate = 8000.0;
        let frequency_in_sample = frequency / sample_rate;
        let mut my_lfo = LFO::new(amplitude, frequency_in_sample, sample_rate as i32);
        let sin_value_1 = my_lfo.get_sin_wave(8000);
        assert!(sin_value_1.abs() < 0.001);
        let sin_value_2 = my_lfo.get_sin_wave(2000);
        assert!((sin_value_2 - 1.0).abs() < 0.001);

    }
}
