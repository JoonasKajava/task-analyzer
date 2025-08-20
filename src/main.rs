use iced::widget::{column, text, Column};


#[derive(Debug, Clone)]
enum Message {}


#[derive(Default)]
struct TaskTracker {}

impl TaskTracker {
    fn update(&mut self, message: Message) {
        match message {

        }
    }

    fn view(&self) -> Column<Message> {
        column![
            text("Hello, Task Tracker!"),
        ]
    }
}

fn main() -> iced::Result {
    iced::run("Task Tracker", TaskTracker::update, TaskTracker::view)
}
