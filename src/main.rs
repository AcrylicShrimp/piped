mod core;

fn main() {
    let mut pipeline = core::pipeline::Pipeline::new(
        "copy".to_owned(),
        Some("copy the src file to dst".to_owned()),
    );

    pipeline.add_attributes(core::attribute::Attribute::new(
        "src".to_owned(),
        Some("a source file to be copied".to_owned()),
        core::value::ValueType::Str,
        None,
    ));
    pipeline.add_attributes(core::attribute::Attribute::new(
        "dst".to_owned(),
        Some("a destination path for the copied file to be placed".to_owned()),
        core::value::ValueType::Str,
        None,
    ));

    println!("{:?}", pipeline);
}
