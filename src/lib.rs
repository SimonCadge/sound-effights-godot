#[macro_use]
extern crate gdnative;

use pitch_detection::{McLeodDetector, PitchDetector, Pitch};

#[derive(gdnative::NativeClass)]
#[inherit(gdnative::Reference)]
struct AudioAnalyser;

#[derive(ToVariant)]
struct Analysis {
    sample: gdnative::AudioStreamSample,
    duration: f32,
    volume: f32,
    frequency: Option<f32>,
    clarity: Option<f32>,
}

#[gdnative::methods]
impl AudioAnalyser {
    fn _init(_owner: gdnative::Reference) -> Self {
        AudioAnalyser
    }

    // TODO:
    // Figure out propogating errors up.
    // Consider using enums for errors, or something other than strings.

    #[export]
    fn trim_and_analyse_audio(&self, _owner: gdnative::Reference, audio_stream: gdnative::AudioStreamSample) -> Result<Analysis, String> {
        // Convert data into a standard format that we can analyse.
        let audio_data = get_samples_from_audiostream(audio_stream.clone(), 0, 0);
        let audio_data = match audio_data {
            Ok(audio_data) => audio_data,
            Err(error) => return Err(error),
        };

        // Find the first sample with sound.
        let mut start_index: usize = 0;
        let mut found = false;
        for (index, sample) in audio_data.iter().enumerate() {
            if sample.abs() > 0.05 {
                start_index = index;
                found = true;
                break;
            }
        };
        // If no samples had data over 0.05 it seems that perhaps we failed to record anything.
        if !found {
            godot_error!("The recording is silent");
            return Err(String::from("The recording is silent"));
        };

        // Find the last sample with sound.
        let mut end_index: usize = audio_data.len();
        for (index, sample) in audio_data.iter().enumerate().rev() {
            if sample.abs() > 0.05 {
                end_index = index;
                break;
            }
        };

        if start_index == end_index {
            godot_error!("The calculated start index and end index are the same");
            return Err(String::from("The calculated start index and end index are the same"));
        };

        // Determine how many bytes in the byte array correspond to one f32 sample.
        let audio_format = audio_stream.get_format();
        let bytes_per_sample = match audio_format {
            gdnative::AudioStreamSampleFormat::Format8Bits => 1,
            gdnative::AudioStreamSampleFormat::Format16Bits => 2,
            gdnative::AudioStreamSampleFormat::ImaAdpcm => {
                godot_error!("ImaAdpcm Format Audio");
                return Err(String::from("ImaAdpcm Format Audio"));
            },
        };

        // If the audio data is stereo there are two samples in the byte array for each one in the f32 vector.
        let num_channels = match audio_stream.is_stereo() {
            true => 2,
            false => 1,
        };

        // Cut out silence at the start and end.
        let resized_audio_stream = resize_audio(audio_stream, start_index * bytes_per_sample * num_channels, end_index * bytes_per_sample * num_channels);
        let resized_audio_stream = match resized_audio_stream {
            Ok(new_stream) => new_stream,
            Err(error) => return Err(error),
        };

        return get_analysis(resized_audio_stream, 0, 0);
    }

    #[export]
    fn analyse_audio(&self, _owner: gdnative::Reference, audio_stream: gdnative::AudioStreamSample) -> Result<Analysis, String> {
        return get_analysis(audio_stream, 0, 0);
    }

    #[export]
    fn analyse_end_of_audio(&self, _owner: gdnative::Reference, audio_stream: gdnative::AudioStreamSample, duration: f32) -> Result<Analysis, String> {
        // Determine how many bytes in the byte array correspond to one f32 sample.
        let audio_format = audio_stream.get_format();
        let bytes_per_sample: usize = match audio_format {
            gdnative::AudioStreamSampleFormat::Format8Bits => 1,
            gdnative::AudioStreamSampleFormat::Format16Bits => 2,
            gdnative::AudioStreamSampleFormat::ImaAdpcm => {
                godot_error!("ImaAdpcm Format Audio");
                return Err(String::from("ImaAdpcm Format Audio"));
            },
        };

        // If the audio data is stereo there are two samples in the byte array for each one in the f32 vector.
        let num_channels: usize = match audio_stream.is_stereo() {
            true => 2,
            false => 1,
        };
        
        let end_index = audio_stream.get_data().len() as usize;
        let num_samples = (duration * audio_stream.get_mix_rate() as f32).floor() as usize;
        let start_index = end_index - (num_samples * bytes_per_sample * num_channels);

        if start_index > end_index {
            return get_analysis(audio_stream, 0, 0);
        } else {
            return get_analysis(audio_stream, start_index, num_samples);
        }
    }
}

fn get_analysis(audio_stream: gdnative::AudioStreamSample, data_offset: usize, num_samples: usize) -> Result<Analysis, String> {
    // Convert abstract bytes representation into Vector of f32 samples.
    let audio_samples = get_samples_from_audiostream(audio_stream.clone(), data_offset, num_samples).unwrap();

    // Calculate duration in seconds of recorded audio.
    let duration = audio_samples.len() as f32 / audio_stream.get_mix_rate() as f32;

    let volume = audio_samples.iter().fold(0.0, |accumulator, x| accumulator + x.abs()) / audio_samples.len() as f32;

    // Determine pitch of audio sample. If the person isn't singing it might not succeed.
    let pitch = get_pitch_from_audio_samples(audio_samples, audio_stream.get_mix_rate() as usize);
    let (frequency, clarity) = match pitch {
        Some(pitch) => (Some(pitch.frequency), Some(pitch.clarity)),
        None => (None, None),
    };

    return Ok(Analysis {
        sample: audio_stream,
        duration,
        volume,
        frequency,
        clarity,
    });
}

fn get_samples_from_audiostream(audio_stream: gdnative::AudioStreamSample, data_offset: usize, num_samples: usize) -> Result<Vec<f32>, String> {
    let audio_data = audio_stream.get_data();
    let audio_format = audio_stream.get_format();

    // Determine whether a sample is comprised of one or two bytes in the byte array.
    // An adaptive length encoding is supported but I think it's unlikely that audio will be
    // recorded in that format.
    let bytes_per_sample: usize = match audio_format {
        gdnative::AudioStreamSampleFormat::Format8Bits => 1,
        gdnative::AudioStreamSampleFormat::Format16Bits => 2,
        gdnative::AudioStreamSampleFormat::ImaAdpcm => {
            godot_error!("ImaAdpcm Format Audio");
            return Err(String::from("ImaAdpc Format Audio"));
        },
    };

    // If the audio is in a stereo format then there are two channels interleaved, meaning twice as many samples.
    let num_channels = match audio_stream.is_stereo() {
        true => 2,
        false => 1,
    };

    let num_samples: usize = if num_samples == 0 {
        (audio_data.len() as usize - data_offset) / (bytes_per_sample * num_channels)
    } else {
        num_samples
    };

    // Convert samples into an array of f32 samples.
    let mut audio_samples: Vec<f32> = vec![0.0; num_samples];
    if num_channels == 2 {
        // If we've got stereo sound then we average the sound between the two channels.
        for samples_index in 0..audio_samples.len() {
            let data_index = (samples_index * bytes_per_sample * 2 + data_offset) as i32;
            match bytes_per_sample {
                // Signed 8 bit PCM data ranges from -128 to 127.
                1 => audio_samples[samples_index] = ((audio_data.get(data_index) as f32 / 128.0) + (audio_data.get(data_index + 1) as f32 / 128.0)) / 2.0,
                // Signed 16 bit PCM data ranges from -32768 to 32767.
                2 => audio_samples[samples_index] = ((i16::from_le_bytes([audio_data.get(data_index), audio_data.get(data_index + 1)]) as f32 / 32768.0) + (i16::from_le_bytes([audio_data.get(data_index + 2), audio_data.get(data_index + 3)]) as f32 / 32768.0)) / 2.0,
                _ => {
                    godot_error!("Unexpected number of bytes per sample {}", bytes_per_sample);
                    return Err(format!("Unexpected number of bytes per sample {}", bytes_per_sample));
                },
            }
        }
    } else {
        // If we have mono sound then we just convert each sample to f32 format.
        for samples_index in 0..audio_samples.len() {
            let data_index = (samples_index * bytes_per_sample * 2 + data_offset) as i32;
            match bytes_per_sample {
                1 => audio_samples[samples_index] = audio_data.get(data_index) as f32 / 128.0,
                2 => audio_samples[samples_index] = i16::from_le_bytes([audio_data.get(data_index), audio_data.get(data_index + 1)]) as f32 / 32768.0,
                _ => {
                    godot_error!("Unexpected number of bytes per sample {}", bytes_per_sample);
                    return Err(format!("Unexpected number of bytes per sample {}", bytes_per_sample));
                },
            }
        }
    };
    return Ok(audio_samples);
}

fn resize_audio(audio_stream: gdnative::AudioStreamSample, start_index: usize, end_index: usize) -> Result<gdnative::AudioStreamSample, String> {
    // Create a new mutable audio stream with the same setup as the old one.
    let mut new_stream = gdnative::AudioStreamSample::new();
    new_stream.set_format(audio_stream.get_format() as i64);
    new_stream.set_mix_rate(audio_stream.get_mix_rate());
    new_stream.set_stereo(audio_stream.is_stereo());

    let byte_array = audio_stream.get_data();
    let mut new_byte_array = gdnative::ByteArray::new();
    // Make the new byte array the right size to store the resized audio.
    new_byte_array.resize((end_index - start_index) as i32);
    // Set the bytes in the new byte array to those between start index and end index of the old byte array.
    for index in 0 .. new_byte_array.len() {
        new_byte_array.set(index, byte_array.get(index + (start_index as i32)));
    }
    new_stream.set_data(new_byte_array);
    return Ok(new_stream);
}

fn get_pitch_from_audio_samples(audio_data: Vec<f32>, sample_rate: usize) -> Option<Pitch<f32>> {
    let mut detector: McLeodDetector<f32> = McLeodDetector::new(audio_data.len(), audio_data.len() / 2);
    let pitch = detector.get_pitch(&audio_data, sample_rate, 5.0, 0.7);
    return pitch;
}

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<AudioAnalyser>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();