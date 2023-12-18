use colored::Colorize;

pub fn exit_with_error(message: &str) {
    eprintln!("{} {}", "[ERROR]".red(), message,);

    std::process::exit(1);
}

pub fn running_task_feedback(msg: &str) {
    eprintln!("{} {}", "[RUN]".green(), msg);
}
pub fn file_created_feedback(msg: &str) {
    eprintln!("{} {}", "[CREATED]".green(), msg);
}
