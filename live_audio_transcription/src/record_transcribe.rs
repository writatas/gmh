use whisper_rs::{WhisperContext, WhisperContextParameters, FullParams, SamplingStrategy};
use wavy::Microphone;
use anyhow::{Ok, Error};

pub fn record(whispercpp_path: &str) -> Result<String, Error>{
    let path_to_model = std::env::args().nth(1).unwrap();

    // load a context and model
    let ctx = WhisperContext::new_with_params(
        path_to_model,
        WhisperContextParameters::default()
    ).expect("failed to load model");

    // create a params object
    let params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });

    let mut mic = Microphone::new().expect("Failed to initialize microphone");
    mic.set_sample_rate(16000).expect("Failed to set sample rate");
    let mut audio_buffer = vec![0.0_f32; 1024];

    // Start recording
    mic.start().expect("Failed to start recording");

    // Continuously read audio data
    loop {
        let samples_read = mic.read(&mut audio_buffer).expect("Failed to read audio data");
        // now we can run the model
        let mut state = ctx.create_state().expect("failed to create state");
        state
            .full(params, &samples_read[..])
            .expect("failed to run model");

        // fetch the results
        let num_segments = state
            .full_n_segments()
            .expect("failed to get number of segments");
        for i in 0..num_segments {
            let segment = state
                .full_get_segment_text(i)
                .expect("failed to get segment");
            let start_timestamp = state
                .full_get_segment_t0(i)
                .expect("failed to get segment start timestamp");
            let end_timestamp = state
                .full_get_segment_t1(i)
                .expect("failed to get segment end timestamp");
            println!("[{} - {}]: {}", start_timestamp, end_timestamp, segment);
        }
    }
        Ok("Recorded successfully!".to_string())
}