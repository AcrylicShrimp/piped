mod core;

fn main() {
    let mut pipelines = core::pipelines::python::load_all();

    let mut task = core::task::Task::new("copy-asset".to_owned(), None, pipelines.swap_remove(0));

    task.add_value(
        "src".to_owned(),
        core::value::Value::Str("assets/from.txt".to_owned()),
    );
    task.add_value(
        "dst".to_owned(),
        core::value::Value::Str("assets/to.txt".to_owned()),
    );

    task.execute();
}
