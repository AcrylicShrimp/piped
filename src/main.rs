mod core;

fn main() {
    let task = core::task::Task::new("my-task".to_owned());
    println!("{}", task.name());
}
