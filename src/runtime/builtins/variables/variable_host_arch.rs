#[cfg(target_arch = "x86")]
define_variable!("hostArch", Value::String("x86".to_owned()));

#[cfg(target_arch = "x86_64")]
define_variable!("hostArch", Value::String("x86_64".to_owned()));

#[cfg(target_arch = "arm")]
define_variable!("hostArch", Value::String("arm".to_owned()));
