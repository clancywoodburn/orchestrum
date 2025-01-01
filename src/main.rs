use rodio::{OutputStream, Source};

struct TripleOscillator {
    oscillators: [WaveTableOscillator; 3],
}

impl TripleOscillator {
    fn new(oscillators: [WaveTableOscillator; 3]) -> Self {
        Self { oscillators }
    }
}

impl Iterator for TripleOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.oscillators[0].get_sample())
    }
}

impl Source for TripleOscillator {
    fn channels(&self) -> u16 {
        1
    }
    fn sample_rate(&self) -> u32 {
        self.oscillators[0].sample_rate
    }
    fn current_frame_len(&self) -> Option<usize> {
        None
    }
    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

struct WaveTableOscillator {
    sample_rate: u32,
    wave_table: Vec<f32>,
    index: f32,
    index_increment: f32,
}

impl WaveTableOscillator {
    fn new(sample_rate: u32, wave_table: Vec<f32>) -> Self {
        Self {
            sample_rate,
            wave_table,
            index: 0.0,
            index_increment: 0.0,
        }
    }

    fn set_frequency(&mut self, frequency: f32) {
        self.index_increment = frequency * self.wave_table.len() as f32 / self.sample_rate as f32;
    }

    fn get_sample(&mut self) -> f32 {
        let sample = self.lerp();
        self.index += self.index_increment;
        self.index %= self.wave_table.len() as f32;
        sample
    }

    fn lerp(&self) -> f32 {
        let floored_index = self.index as usize;
        let next_index = (floored_index + 1) % self.wave_table.len();

        let next_weight = self.index - floored_index as f32;
        let floored_weight = 1.0 - next_weight;

        self.wave_table[floored_index] * floored_weight + self.wave_table[next_index] * next_weight
    }
}

impl Iterator for WaveTableOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.get_sample())
    }
}

impl Source for WaveTableOscillator {
    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

fn main() {
    let wave_table_size = 64;
    let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);
    for n in 0..wave_table_size {
        wave_table.push((2.0 * std::f32::consts::PI * n as f32 / wave_table_size as f32).sin());
    }

    let mut oscillators = [
        WaveTableOscillator::new(44100, wave_table.clone()),
        WaveTableOscillator::new(44100, wave_table.clone()),
        WaveTableOscillator::new(44100, wave_table.clone()),
    ];

    oscillators[0].set_frequency(220.00);
    oscillators[1].set_frequency(440.00);
    oscillators[2].set_frequency(110.00);

    let triple_oscillator = TripleOscillator::new(oscillators);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let _result = stream_handle.play_raw(triple_oscillator.convert_samples());

    std::thread::sleep(std::time::Duration::from_secs(5));
}
