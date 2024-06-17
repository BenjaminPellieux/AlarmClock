use iced::{button, Button, Column, Element, Sandbox, Settings, Text};

#[derive(Default)]
struct HelloWorld {
    button: button::State,
    message: String,
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed,
}

impl Sandbox for HelloWorld {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Hello Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed => {
                self.message = String::from("Hello, world!");
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .push(
                Button::new(&mut self.button, Text::new("Press me"))
                    .on_press(Message::ButtonPressed),
            )
            .push(Text::new(&self.message))
            .into()
    }
}

fn main() {
    HelloWorld::run(Settings::default());
}
