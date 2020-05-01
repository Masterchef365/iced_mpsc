use iced::*;

fn main() {
    App::run(Default::default())
}

struct App {
    clicky: button::State,
    n_clicks: usize,
}

#[derive(Debug, Clone)]
enum Message {
    Clicky,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let instance = Self {
            clicky: button::State::new(),
            n_clicks: 0,
        };
        (instance, Command::none())
    }

    fn title(&self) -> String {
        "MPSC Demonstration".into()
    }

    fn view(&mut self) -> Element<Self::Message> {
        Button::new(&mut self.clicky, Text::new(format!("{} Clicks", self.n_clicks))).on_press(Message::Clicky).into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Clicky => self.n_clicks += 1,
        }
        Command::none()
    }
}
