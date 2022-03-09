use rodio::{buffer::SamplesBuffer, OutputStream};
use hound::{WavSpec, WavWriter, SampleFormat};

fn main() {
    
	let sample_rate = 44100;

	// base frequency, which effectively is the increment for the phase
	let carrier_freq = 120.0 / sample_rate as f32;

	// fof frequency, for the symplectic integrator
	let modulator_freq = 480.0  / sample_rate as f32;

	// decay bandwidth
	let bandwidth = 2.2;
	// phase of the pulse
	let mut phase = 0.0;

	// symplectic integrator states
	let mut sine_carrier = 0.0;
	let mut cosine_carrier = 1.0;
	let mut sine_modulator = 0.0;
	let mut cosine_modulator = 1.0;

	// array to put sound into
	let mut audio_output = Vec::with_capacity(sample_rate);

	// and make sound
	for _ in 0..sample_rate {

		// increment the phase
		phase += carrier_freq;

		// if it hits 1, wrap it around and generate an impulse
		if phase >= 1.0 {

			sine_carrier = 0.0;
			cosine_carrier = 1.0;
			sine_modulator = 0.0;
			cosine_modulator = 1.0;

			phase = 0.0;
		}

		// integrate the sines
		cosine_carrier -= sine_carrier * carrier_freq * std::f32::consts::TAU;
		sine_carrier += cosine_carrier * carrier_freq * std::f32::consts::TAU;
		cosine_modulator -= sine_modulator * modulator_freq * std::f32::consts::TAU;
		sine_modulator += cosine_modulator * modulator_freq * std::f32::consts::TAU;

		// resulting sound
		let sample = (bandwidth * (cosine_carrier - 1.0)).exp() * cosine_modulator;

		// add it to the output
		audio_output.push(sample);

	}

	// get the max amplitude
	let max_amplitude = audio_output.iter().fold(0.0 as f32, |acc, x| acc.max(x.abs()));

	// some info
	println!("Max amplitude: {}", max_amplitude);

	// normalize it to avoid playback issues
	for s in &mut audio_output {
		*s /= max_amplitude;
	}

	// playback
	// Get a output stream handle to the default physical sound device
	let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to open stream");

	// actually play it
	stream_handle.play_raw(SamplesBuffer::new(1, sample_rate as u32, audio_output.clone())).expect("Failed to play");

	// get hound to get the spec so we can save the file
    let spec = WavSpec {
        channels: 1,
        sample_rate: sample_rate as u32,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

	// save it
	// and where to write the file to
    let mut writer =
        WavWriter::create("fof_sound.wav", spec).expect("failed to make writer");

    // write all samples to the file
    for sample in &audio_output {
        writer
            .write_sample((*sample * std::i16::MAX as f32) as i16)
            .expect("failed to write");
    }

    // and save it
    writer.finalize().expect("failed to save");

	// wait for it to finish
	std::thread::sleep(std::time::Duration::from_secs_f32(1.2));
}
