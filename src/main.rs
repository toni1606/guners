use std::time::Duration;
use std::{fmt::Debug, time::Instant};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Sample, SampleFormat,
};

fn main() {
    // Get the defaul host (probably alsa).
    let host = cpal::default_host();
    // Get the default input device (microphone).
    let dev = host
        .default_input_device()
        .expect("could not query default input device");

    // Get the config of the microphone (e.g.: sample rate in Hz, sample format)
    let sup_cfg = dev.default_input_config().expect("no supported config!");
    eprintln!("cfg: {sup_cfg:?}");

    // Get the sample format.
    let sample_fmt = sup_cfg.sample_format();
    // Convert from SupportedStreamConfig into StreamConfig for use in the stream building.
    let cfg = sup_cfg.into();

    // Error handler for stream.
    let err_handler = |err| eprintln!("An error occurred on the input audio stream: {err}");

    // Check the most common sample formats and create the input stream accordingly.
    // (note the turbofish operator)
    //
    // Note:
    // The last arguments is the timeout. If it is set to None the handler will block the worker
    // thread. If not Some(Duration) must be specified.
    let stream = match sample_fmt {
        SampleFormat::F32 => dev.build_input_stream(&cfg, data_handler::<f32>, err_handler, None),
        SampleFormat::I16 => dev.build_input_stream(&cfg, data_handler::<i16>, err_handler, None),
        SampleFormat::U16 => dev.build_input_stream(&cfg, data_handler::<u16>, err_handler, None),
        SampleFormat::U8 => dev.build_input_stream(&cfg, data_handler::<u8>, err_handler, None),
        sample_fmt => panic!("Unsuported sample format: {sample_fmt}"),
    }
    .expect("could not create mic stream");

    // Some devices do not start the stream automatically so you have to do it.
    stream.play().expect("Could not start stream");

    // Performs the 500ms cycle 5 times.
    for _ in 0..5 {
        std::thread::sleep(Duration::from_millis(500));
        println!("\n");
    }
}

// The function which takes each of the datapoints from the stream and performs operations on them.
fn data_handler<T: Sample + Debug>(data: &[T], _: &cpal::InputCallbackInfo) {
    println!("{:?}\tlen: {}", Instant::now(), data.len());
}
