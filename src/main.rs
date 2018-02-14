#[macro_use]
extern crate clap;
extern crate hound;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
use clap::{Arg, App};
use std::cmp::max;

// ./wavtojson tests/test.wav 4800 1200 1200

#[derive(Serialize)]
struct OutputData {
	width: usize,
	height: u64,
	resolution: u64,
	data: Vec<u64>
}

fn main() {
	let matches = App::new("Rusty Wav-to-JSON")
		.version("0.1")
		.author("JonathanUsername")
		.about("JSONifies wav files for use in waveforms")
		.arg(Arg::with_name("WAVFILE")
			.help("Path to wav file to parse")
			.required(true)
			.index(1))
		.arg(Arg::with_name("WIDTH")
			.help("Width of waveform")
			.required(true)
			.index(2))
		.arg(Arg::with_name("HEIGHT")
			.help("Height of waveform")
			.required(true)
			.index(3))
		.arg(Arg::with_name("RESOLUTION")
			.help("Resolution of waveform")
			.required(true)
			.index(4))
		.get_matches();

	let filename = matches.value_of("WAVFILE").unwrap();
	let width = value_t!(matches, "WIDTH", usize).unwrap();
	let height = value_t!(matches, "HEIGHT", u64).unwrap();
	let resolution = value_t!(matches, "RESOLUTION", u64).unwrap();

	// Initialise a vector to the right width. One element for each pixel.
	let sections = vec![0; width];

	let mut reader = hound::WavReader::open(filename).unwrap();
	// Samples-per-pixel taking into account channels
	let samples_per_pixel = reader.len() / width as u32;

	reader.seek(0).unwrap();
	let mut samples = reader.samples();
	{
		let mut data: Vec<u32> = vec![];
		for _ in sections.iter() {
			let mut prev = 0;
			let mut sum: u32 = 0;
			for _ in 0..samples_per_pixel {
				let mut next_sample = samples.next();
				match next_sample {
					Some(Ok(sample)) => {
						let diff: i32 = sample - prev;
						sum += diff.abs() as u32;
						prev = sample;
					},
					_ => () // Skip if null
				}
			}
			data.push(sum);
		}
		// Scale it down
		let max_value = data.iter()
			.fold(0, |i, sum| max(i, *sum));

		// TODO: Tidy up type casting
		let scaled_data = data.iter()
			.map(|i| ((*i as f32 / max_value as f32).powf(0.3) * height as f32) as u64)
			.collect::<Vec<u64>>();

		let output = OutputData {
			width: width,
			height: height,
			resolution: resolution,
			data: scaled_data
		};
		print!("{}", serde_json::to_string(&output).unwrap());
	}
}
