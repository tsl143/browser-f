use super::utils::{test_get_default_device, Scope};
use super::*;

// get_device_global_uid
// ------------------------------------
#[test]
fn test_get_device_global_uid() {
    // Input device.
    if let Some(input) = test_get_default_device(Scope::Input) {
        let uid = get_device_global_uid(input).unwrap();
        let uid = uid.into_string();
        assert!(!uid.is_empty());
    }

    // Output device.
    if let Some(output) = test_get_default_device(Scope::Output) {
        let uid = get_device_global_uid(output).unwrap();
        let uid = uid.into_string();
        assert!(!uid.is_empty());
    }
}

#[test]
#[should_panic]
fn test_get_device_global_uid_by_unknwon_device() {
    // Unknown device.
    assert!(get_device_global_uid(kAudioObjectUnknown).is_err());
}

// get_device_uid
// ------------------------------------
#[test]
fn test_get_device_uid() {
    // Input device.
    if let Some(input) = test_get_default_device(Scope::Input) {
        let uid = get_device_uid(input, DeviceType::INPUT).unwrap();
        let uid = uid.into_string();
        assert!(!uid.is_empty());
    }

    // Output device.
    if let Some(output) = test_get_default_device(Scope::Output) {
        let uid = get_device_uid(output, DeviceType::OUTPUT).unwrap();
        let uid = uid.into_string();
        assert!(!uid.is_empty());
    }
}

#[test]
#[should_panic]
fn test_get_device_uid_by_unknwon_device() {
    // Unknown device.
    assert!(get_device_uid(kAudioObjectUnknown, DeviceType::INPUT).is_err());
}

// get_device_source
// ------------------------------------
// Some USB headsets (e.g., Plantronic .Audio 628) fails to get data source.
#[test]
fn test_get_device_source() {
    if let Some(device) = test_get_default_device(Scope::Input) {
        if let Ok(source) = get_device_source(device, DeviceType::INPUT) {
            println!(
                "input: {:X}, {:?}",
                source,
                convert_uint32_into_string(source)
            );
        } else {
            println!("No input data source.");
        }
    } else {
        println!("No input device.");
    }

    if let Some(device) = test_get_default_device(Scope::Output) {
        if let Ok(source) = get_device_source(device, DeviceType::OUTPUT) {
            println!(
                "output: {:X}, {:?}",
                source,
                convert_uint32_into_string(source)
            );
        } else {
            println!("No output data source.");
        }
    } else {
        println!("No output device.");
    }
}

#[test]
#[should_panic]
fn test_get_device_source_by_unknown_device() {
    assert!(get_device_source(kAudioObjectUnknown, DeviceType::INPUT).is_err());
}

// get_device_source_name
// ------------------------------------
#[test]
fn test_get_device_source_name() {
    if let Some(device) = test_get_default_device(Scope::Input) {
        if let Ok(name) = get_device_source_name(device, DeviceType::INPUT) {
            println!("input: {}", name.into_string());
        } else {
            println!("No input data source name.");
        }
    } else {
        println!("No input device.");
    }

    if let Some(device) = test_get_default_device(Scope::Output) {
        if let Ok(name) = get_device_source_name(device, DeviceType::OUTPUT) {
            println!("output: {}", name.into_string());
        } else {
            println!("No output data source name.");
        }
    } else {
        println!("No output device.");
    }
}

#[test]
#[should_panic]
fn test_get_device_source_name_by_unknown_device() {
    assert!(get_device_source_name(kAudioObjectUnknown, DeviceType::INPUT).is_err());
}

// get_device_name
// ------------------------------------
#[test]
fn test_get_device_name() {
    if let Some(device) = test_get_default_device(Scope::Input) {
        let name = get_device_name(device, DeviceType::INPUT).unwrap();
        println!("input device name: {}", name.into_string());
    } else {
        println!("No input device.");
    }

    if let Some(device) = test_get_default_device(Scope::Output) {
        let name = get_device_name(device, DeviceType::OUTPUT).unwrap();
        println!("output device name: {}", name.into_string());
    } else {
        println!("No output device.");
    }
}

#[test]
#[should_panic]
fn test_get_device_name_by_unknown_device() {
    assert!(get_device_name(kAudioObjectUnknown, DeviceType::INPUT).is_err());
}

// get_device_label
// ------------------------------------
#[test]
fn test_get_device_label() {
    if let Some(device) = test_get_default_device(Scope::Input) {
        let name = get_device_label(device, DeviceType::INPUT).unwrap();
        println!("input device label: {}", name.into_string());
    } else {
        println!("No input device.");
    }

    if let Some(device) = test_get_default_device(Scope::Output) {
        let name = get_device_label(device, DeviceType::OUTPUT).unwrap();
        println!("output device label: {}", name.into_string());
    } else {
        println!("No output device.");
    }
}

#[test]
#[should_panic]
fn test_get_device_label_by_unknown_device() {
    assert!(get_device_label(kAudioObjectUnknown, DeviceType::INPUT).is_err());
}

// get_device_manufacturer
// ------------------------------------
#[test]
fn test_get_device_manufacturer() {
    if let Some(device) = test_get_default_device(Scope::Input) {
        // Some devices like AirPods cannot get the vendor info so we print the error directly.
        // TODO: Replace `map` and `unwrap_or_else` by `map_or_else`
        let name = get_device_manufacturer(device, DeviceType::INPUT)
            .map(|name| name.into_string())
            .unwrap_or_else(|e| format!("Error: {}", e));
        println!("input device vendor: {}", name);
    } else {
        println!("No input device.");
    }

    if let Some(device) = test_get_default_device(Scope::Output) {
        // Some devices like AirPods cannot get the vendor info so we print the error directly.
        // TODO: Replace `map` and `unwrap_or_else` by `map_or_else`
        let name = get_device_manufacturer(device, DeviceType::OUTPUT)
            .map(|name| name.into_string())
            .unwrap_or_else(|e| format!("Error: {}", e));
        println!("output device vendor: {}", name);
    } else {
        println!("No output device.");
    }
}

#[test]
#[should_panic]
fn test_get_device_manufacturer_by_unknown_device() {
    assert!(get_device_manufacturer(kAudioObjectUnknown, DeviceType::INPUT).is_err());
}

// get_device_buffer_frame_size_range
// ------------------------------------
#[test]
fn test_get_device_buffer_frame_size_range() {
    if let Some(device) = test_get_default_device(Scope::Input) {
        let range = get_device_buffer_frame_size_range(device, DeviceType::INPUT).unwrap();
        println!(
            "range of input buffer frame size: {}-{}",
            range.mMinimum, range.mMaximum
        );
    } else {
        println!("No input device.");
    }

    if let Some(device) = test_get_default_device(Scope::Output) {
        let range = get_device_buffer_frame_size_range(device, DeviceType::OUTPUT).unwrap();
        println!(
            "range of output buffer frame size: {}-{}",
            range.mMinimum, range.mMaximum
        );
    } else {
        println!("No output device.");
    }
}

#[test]
#[should_panic]
fn test_get_device_buffer_frame_size_range_by_unknown_device() {
    assert!(get_device_buffer_frame_size_range(kAudioObjectUnknown, DeviceType::INPUT).is_err());
}

// get_device_latency
// ------------------------------------
#[test]
fn test_get_device_latency() {
    if let Some(device) = test_get_default_device(Scope::Input) {
        let latency = get_device_latency(device, DeviceType::INPUT).unwrap();
        println!("latency of input device: {}", latency);
    } else {
        println!("No input device.");
    }

    if let Some(device) = test_get_default_device(Scope::Output) {
        let latency = get_device_latency(device, DeviceType::OUTPUT).unwrap();
        println!("latency of output device: {}", latency);
    } else {
        println!("No output device.");
    }
}

#[test]
#[should_panic]
fn test_get_device_latency_by_unknown_device() {
    assert!(get_device_latency(kAudioObjectUnknown, DeviceType::INPUT).is_err());
}

// get_device_streams
// ------------------------------------
#[test]
fn test_get_device_streams() {
    if let Some(device) = test_get_default_device(Scope::Input) {
        let streams = get_device_streams(device, DeviceType::INPUT).unwrap();
        println!("streams on the input device: {:?}", streams);
    } else {
        println!("No input device.");
    }

    if let Some(device) = test_get_default_device(Scope::Output) {
        let streams = get_device_streams(device, DeviceType::OUTPUT).unwrap();
        println!("streams on the output device: {:?}", streams);
    } else {
        println!("No output device.");
    }
}

#[test]
#[should_panic]
fn test_get_device_streams_by_unknown_device() {
    assert!(get_device_streams(kAudioObjectUnknown, DeviceType::INPUT).is_err());
}

// get_device_sample_rate
// ------------------------------------
#[test]
fn test_get_device_sample_rate() {
    if let Some(device) = test_get_default_device(Scope::Input) {
        let rate = get_device_sample_rate(device, DeviceType::INPUT).unwrap();
        println!("input sample rate: {}", rate);
    } else {
        println!("No input device.");
    }

    if let Some(device) = test_get_default_device(Scope::Output) {
        let rate = get_device_sample_rate(device, DeviceType::OUTPUT).unwrap();
        println!("output sample rate: {}", rate);
    } else {
        println!("No output device.");
    }
}

#[test]
#[should_panic]
fn test_get_device_sample_rate_by_unknown_device() {
    assert!(get_device_sample_rate(kAudioObjectUnknown, DeviceType::INPUT).is_err());
}

// get_ranges_of_device_sample_rate
// ------------------------------------
#[test]
fn test_get_ranges_of_device_sample_rate() {
    if let Some(device) = test_get_default_device(Scope::Input) {
        let ranges = get_ranges_of_device_sample_rate(device, DeviceType::INPUT).unwrap();
        println!("ranges of input sample rate: {:?}", ranges);
    } else {
        println!("No input device.");
    }

    if let Some(device) = test_get_default_device(Scope::Output) {
        let ranges = get_ranges_of_device_sample_rate(device, DeviceType::OUTPUT).unwrap();
        println!("ranges of output sample rate: {:?}", ranges);
    } else {
        println!("No output device.");
    }
}

#[test]
#[should_panic]
fn test_get_ranges_of_device_sample_rate_by_unknown_device() {
    assert!(get_ranges_of_device_sample_rate(kAudioObjectUnknown, DeviceType::INPUT).is_err());
}

// get_device_stream_format
// ------------------------------------
#[test]
fn test_get_device_stream_format() {
    if let Some(device) = test_get_default_device(Scope::Input) {
        let format = get_device_stream_format(device, DeviceType::INPUT).unwrap();
        println!("input stream format: {:?}", format);
    } else {
        println!("No input device.");
    }

    if let Some(device) = test_get_default_device(Scope::Output) {
        let format = get_device_stream_format(device, DeviceType::OUTPUT).unwrap();
        println!("output stream format: {:?}", format);
    } else {
        println!("No output device.");
    }
}

#[test]
#[should_panic]
fn test_get_device_stream_format_by_unknown_device() {
    assert!(get_device_stream_format(kAudioObjectUnknown, DeviceType::INPUT).is_err());
}

// get_device_stream_configuration
// ------------------------------------
#[test]
fn test_get_device_stream_configuration() {
    if let Some(device) = test_get_default_device(Scope::Input) {
        let buffers = get_device_stream_configuration(device, DeviceType::INPUT).unwrap();
        println!("input stream config: {:?}", buffers);
        dbg!(buffers);
    } else {
        println!("No input device.");
    }

    if let Some(device) = test_get_default_device(Scope::Output) {
        let buffers = get_device_stream_configuration(device, DeviceType::OUTPUT).unwrap();
        println!("output stream config: {:?}", buffers);
        dbg!(buffers);
    } else {
        println!("No output device.");
    }
}

#[test]
#[should_panic]
fn test_get_device_stream_configuration_by_unknown_device() {
    assert!(get_device_stream_configuration(kAudioObjectUnknown, DeviceType::INPUT).is_err());
}

// get_stream_latency
// ------------------------------------
#[test]
fn test_get_stream_latency() {
    if let Some(device) = test_get_default_device(Scope::Input) {
        let streams = get_device_streams(device, DeviceType::INPUT).unwrap();
        for stream in streams {
            let latency = get_stream_latency(stream, DeviceType::INPUT).unwrap();
            println!("latency of the input stream {} is {}", stream, latency);
        }
    } else {
        println!("No input device.");
    }

    if let Some(device) = test_get_default_device(Scope::Output) {
        let streams = get_device_streams(device, DeviceType::OUTPUT).unwrap();
        for stream in streams {
            let latency = get_stream_latency(stream, DeviceType::OUTPUT).unwrap();
            println!("latency of the output stream {} is {}", stream, latency);
        }
    } else {
        println!("No output device.");
    }
}

#[test]
#[should_panic]
fn test_get_stream_latency_by_unknown_device() {
    assert!(get_stream_latency(kAudioObjectUnknown, DeviceType::INPUT).is_err());
}
