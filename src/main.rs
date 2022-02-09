use rodio::{buffer::SamplesBuffer, OutputStream};
use hound::{WavSpec, WavWriter, SampleFormat};

fn main() {
    
	let sample_rate = 44100;

	// base frequency, which effectively is the increment for the phase
	let base_freq = 120.0 / sample_rate as f32;

	// fof frequency, for the symplectic integrator
	let fof_freq = 400.0 * std::f32::consts::TAU  / sample_rate as f32;

	// decay bandwidth
	let fof_decay_bw = 200.0 / sample_rate as f32;

	// fof decay, or peak bandwidth
	let fof_decay = (-fof_decay_bw * std::f32::consts::PI).exp();

	// softness
	let softness_bw = 100.0 / sample_rate as f32;

	// phase of the pulse
	let mut phase = 0.0;

	// blend phase
	let mut blend_phase = 0.0;

	// which side to reset
	let mut side = false;

	// symplectic integrator states
	let mut sine_left = 0.0;
	let mut cosine_left = 0.0;
	let mut sine_right = 0.0;
	let mut cosine_right = 0.0;

	// array to put sound into
	let mut audio_output = Vec::with_capacity(sample_rate);

	// and make sound
	for _ in 0..sample_rate {

		// increment the phase
		phase += base_freq;
		blend_phase += softness_bw;

		// if it hits 1, wrap it around and generate an impulse
		if phase >= 1.0 {

			// change the side
			side = !side;

			// wrap around
			phase = 0.0;
			blend_phase = 0.0;

			if side {
				sine_left = 0.0;
				cosine_left = 1.0;
			} else {
				sine_right = 0.0;
				cosine_right = 1.0;
			}
		}

		// integrate the sine
		cosine_left -= sine_left * fof_freq;
		sine_left += cosine_left * fof_freq;
		cosine_right -= sine_right * fof_freq;
		sine_right += cosine_right * fof_freq;

		// decay
		sine_left *= fof_decay;
		cosine_left *= fof_decay;
		sine_right *= fof_decay;
		cosine_right *= fof_decay;

		// blend amount
		let alpha = if side { blend_phase.min(1.0) } else { 1.0 - blend_phase.min(1.0) };

		// resulting sound
		let sample = sine_left * alpha + sine_right * (1.0 - alpha);

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
