use whisper_rs::{WhisperContext, WhisperContextParameters, FullParams, SamplingStrategy};
use anyhow::{Ok, Error};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::time::Duration;

pub fn record<'a>(whispercpp_path: &'a str) -> Result<String, Error>{
    let mut result_text: String = String::from("");
    // load a context and model
    let ctx = WhisperContext::new_with_params(
        whispercpp_path,
        WhisperContextParameters::default()
    ).expect("failed to load model");

    // Set up audio streaming
    let host = cpal::default_host();
    // Get the default input device (microphone)
    let input_device = host.default_input_device().expect("No input device available.");

    // Define the audio format (sample rate, channels, etc.)
    let config = cpal::StreamConfig {
        channels: 1, // Mono
        sample_rate: cpal::SampleRate(44100),
        buffer_size: cpal::BufferSize::Default,
    };

    // Build the input stream
    let buffer_duration = Duration::from_millis(10);
    let input_stream = input_device
        .build_input_stream(&config, move |data: &[f32], _: &cpal::InputCallbackInfo| {
          let mut i16_buffer: Vec<i16> = Vec::with_capacity(data.len());
          for &sample in data {
          let scaled_sample = (sample * i16::MAX as f32) as i16;
          i16_buffer.push(scaled_sample);
        }
      // Run model
      let samples_read = whisper_rs::convert_integer_to_float_audio(&i16_buffer);
      let mut state = ctx.create_state().expect("failed to create state");
      let params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
      state.full(params, &samples_read[..]).expect("failed to run model");
      //Fetch text segments
      let num_segments = state.full_n_segments().expect("failed to get number of segments");
      for i in 0..num_segments {
          let segment = state.full_get_segment_text(i).expect("failed to get segment");
          // Process the segment
          result_text += segment.as_str();
      }
       
    },
    err_fn, Some(buffer_duration))
        .expect("Error creating input stream.");
  
    // Start the input stream
    input_stream.play().expect("Error starting input stream.");
  
    Ok("Recorded successfully!".to_string())
}

 // Error callback function
 fn err_fn(err: cpal::StreamError) {
    eprintln!("An error occurred on the stream: {}", err);
}