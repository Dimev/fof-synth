use rodio::{buffer::SamplesBuffer, OutputStream};

fn main() {
    
	let sample_rate = 44100;

	// base frequency, which effectively is the increment for the phase
	let base_freq = 100.0 / sample_rate as f32;

	// fof frequency, for the symplectic integrator
	let fof_freq = 500.0 * std::f32::consts::TAU  / sample_rate as f32;

	// fof rise
	// TODO

	// fof decay, or peak bandwidth
	let fof_decay = (50.0 * -std::f32::consts::PI / sample_rate as f32).exp();

	// phase of the pulse
	let mut phase = 0.0;

	// symplectic integrator states
	let mut sine = 0.0;
	let mut cosine = 0.0;

	// array to put sound into
	let mut audio_output = Vec::with_capacity(sample_rate);

	// and make sound
	for _ in 0..sample_rate {

		// increment the phase
		phase += base_freq;

		// if it hits 1, wrap it around and generate an impulse
		let pulse = if phase >= 1.0 {

			// wrap around
			phase -= 1.0;

			// and an impulse
			1.0

		} else {

			// no impulse
			0.0

		};

		// add it to the integrator
		cosine += pulse;

		// integrate the sine
		cosine -= sine * fof_freq;
		sine += cosine * fof_freq;

		// decay
		sine *= fof_decay;
		cosine *= fof_decay;

		// resulting sound
		let sample = sine;

		// add it to the output
		audio_output.push(sample);

	}

	// some info
	println!("Max amplitude: {}", audio_output.iter().fold(0.0 as f32, |acc, x| acc.max(x.abs())));

	// playback
	// Get a output stream handle to the default physical sound device
	let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to open stream");

	// actually play it
	stream_handle.play_raw(SamplesBuffer::new(1, sample_rate as u32, audio_output.clone())).expect("Failed to play");

	// save it

	// wait for it to finish
	std::thread::sleep(std::time::Duration::from_secs_f32(1.2));
}
