#[cfg(target_os = "linux")]
define_variable!("hostOS", Value::String("linux".to_owned()));

#[cfg(target_os = "macos")]
define_variable!("hostOS", Value::String("macos".to_owned()));

#[cfg(target_os = "windows")]
define_variable!("hostOS", Value::String("windows".to_owned()));
