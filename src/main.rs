use rodio::{buffer::SamplesBuffer, OutputStream};
use hound::{WavSpec, WavWriter};

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

	// get hound to get the spec so we can save the file
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: sample_rate as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
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
