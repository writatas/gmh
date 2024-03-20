use anyhow::Error;
use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{FromSample, Sample};
use hound::WavSpec;
use std::fs::File;
use std::io::BufWriter;
use std::sync::{Arc, Mutex};
use std::path::Path;
use std::{i16, thread};
use whisper_rs::{FullParams, SamplingStrategy, WhisperContextParameters, WhisperContext};
use hound::{SampleFormat, WavReader};
use std::sync::mpsc::channel;

// types
type WavWriterHandle = Arc<Mutex<Option<hound::WavWriter<BufWriter<File>>>>>;

pub fn continuous_recording<F> (
    file_name: &str, 
    whisper_path: &str, 
    recording_bool: Arc<Mutex<bool>>,
    callback: F,
) -> Result<(), Error>
where
    F: Fn(&str, &str) -> std::io::Result<String> + std::marker::Sync + std::marker::Send +'static + Copy,
{
    loop {
        let (tx, rx) = channel(); // Create a new sender channel for each iteration
        if *recording_bool.lock().unwrap() == false {
            break;
        } else {
            record_audio(&file_name)?;

            let whisper_path_clone = whisper_path.to_owned();
            let file_path_clone = file_name.to_owned();
    
            thread::spawn(move || {
                let transcribed_text = transcribe_audio_file(&file_path_clone, &whisper_path_clone);
                tx.send(transcribed_text).unwrap(); // Handle send error if necessary
            });
    
            let received = rx.recv()?; // Handle receive error if necessary
            // Additional logic for ongoing recording
            let file_grand_parent = Path::new(&file_name).parent().unwrap().display().to_string();
            let transcribed_text_path = Path::new(&file_grand_parent).join("transcribed.txt").display().to_string();
            let save_text_closure = move |file_name: &str, text: &str| {
                callback(file_name, text)
            };
            thread::spawn(move ||{
                //Use the callback function to save to text file
                let _saved = save_text_closure(&transcribed_text_path, &received).unwrap();
            });
        }
    }

    Ok(())
}
// Recording of audio
fn record_audio(file_name: &str) -> Result<(), anyhow::Error> {
    let host = cpal::default_host();

    // Set up the input device and stream with the default input config.
    let device = host.default_input_device();

    let config = device.as_ref()
        .unwrap()
        .default_input_config()
        .expect("Failed to get default input config");

    // The WAV file we're recording to.
    let spec = WavSpec {
        channels: 1,
        sample_rate: 44_100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int
    };

    let writer = hound::WavWriter::create(file_name, spec)?;
    let writer = Arc::new(Mutex::new(Some(writer)));

    // A flag to indicate that recording is in progress.
    println!("Begin recording...");

    let err_fn = move |err| {
        eprintln!("an error occurred on stream: {}", err);
    };

    let writer_2 = writer.clone();

    let thread_handle = std::thread::spawn(move || {
        let stream = device.unwrap().build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<i16, i16>(data, &writer_2),
            err_fn,
            None,
        )?;

        loop {
            std::thread::sleep(std::time::Duration::from_secs(8));
            break;
        }

        drop(stream);
        writer.lock().unwrap().take().unwrap().finalize()?;
        Ok::<(), anyhow::Error>(())
    });

    thread_handle.join().unwrap()?;

    Ok(())
}

fn transcribe_audio_file(audio_file: &str, whisper_path: &str) -> String {
    let mut result_text = String::from("");
    let audio_out_string = convert_sample_rate(&audio_file).unwrap();
    let audio_output_path = Path::new(audio_out_string.as_str());
    let original_samples = parse_wav_file(&audio_output_path);
    let samples = whisper_rs::convert_integer_to_float_audio(&original_samples);

    let ctx = WhisperContext::new_with_params(
        whisper_path,
        WhisperContextParameters::default()
    ).expect("failed to load model");

    // Run model
    let mut state = ctx.create_state().expect("failed to create state");
    let params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    state.full(params, &samples[..]).expect("failed to run model");
    //Fetch text segments
    let num_segments = state.full_n_segments().expect("failed to get number of segments");
    for i in 0..num_segments {
        let segment = state.full_get_segment_text(i).expect("failed to get segment");
        // Process the segment
        result_text += segment.as_str();
    }
    result_text
}

fn write_input_data<T, U>(input: &[T], writer: &WavWriterHandle)
where
    T: Sample,
    U: Sample + hound::Sample + FromSample<T>,
{
    if let Ok(mut guard) = writer.try_lock() {
        if let Some(writer) = guard.as_mut() {
            for &sample in input.iter() {
                let sample: U = U::from_sample(sample);
                writer.write_sample(sample).ok();
            }
        }
    }
}

// Transcription of audio
fn parse_wav_file(path: &Path) -> Vec<i16> {
    let reader = WavReader::open(path).expect("failed to read file");

    if reader.spec().channels != 1 {
        panic!("expected mono audio file");
    }
    if reader.spec().sample_format != SampleFormat::Int {
        panic!("expected integer sample format");
    }
    if reader.spec().sample_rate != 16000 {
        panic!("expected 16KHz sample rate");
    }
    if reader.spec().bits_per_sample != 16 {
        panic!("expected 16 bits per sample");
    }

    reader
        .into_samples::<i16>()
        .map(|x| x.expect("sample"))
        .collect::<Vec<_>>()
}

fn convert_sample_rate(file_name: &str) -> Result<String, hound::Error> {
    // Open the input WAV file
    let mut reader = hound::WavReader::open(file_name)?;
    assert_eq!(reader.spec().channels, 1);
    assert_eq!(reader.spec().sample_format, hound::SampleFormat::Int);

    // Set up the output WAV file with the new sample rate store in test_wavs for now
    let stem = Path::new(&file_name).file_stem().unwrap().to_str().unwrap();
    let output_file_name = Path::new(&file_name).parent().unwrap().join(format!("{}_out.wav", stem));


    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 16000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(&output_file_name, spec)?;

    // Read samples from the input file and write them to the output file
    // Convert to correct sampleby only writting every nth sample, where n
    // is the ratio of the input and output sample rates
    const N: usize = 5;
    let ratio = reader.spec().sample_rate as f32 / writer.spec().sample_rate as f32;
    for (i, sample) in reader.samples::<i16>().enumerate() {
        if i % N == 0 {
            let sample = sample.unwrap();
            let resampled_sample = (sample as f32 * ratio) as i16;
            writer.write_sample(resampled_sample).unwrap();
        }
    }
    Ok(output_file_name.display().to_string())
}