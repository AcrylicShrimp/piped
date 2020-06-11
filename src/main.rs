mod core;

use std::fs::read_to_string;

fn main() {
    let pipelines = core::pipelines::python::load_all();
    let task = core::task::Task::load(&read_to_string("tasks/test.ron").unwrap(), &pipelines);

    task.execute();
}
