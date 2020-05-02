use iced::*;
mod mpsc;

fn main() {
    App::run(Default::default())
}

struct App {
    clicky: button::State,
    n_clicks: usize,
    sender: Option<mpsc::Sender<()>>,
}

#[derive(Debug, Clone)]
enum Message {
    Clicky,
    Mpsc(mpsc::Message<()>),
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let instance = Self {
            clicky: button::State::new(),
            sender: None,
            n_clicks: 0,
        };
        (instance, Command::none())
    }

    fn title(&self) -> String {
        "MPSC Demonstration".into()
    }

    fn view(&mut self) -> Element<Self::Message> {
        Button::new(
            &mut self.clicky,
            Text::new(format!("{} Clicks", self.n_clicks)),
        )
        .on_press(Message::Clicky)
        .into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Mpsc(mpsc::Message::Received(_)) => self.n_clicks += 1,
            Message::Mpsc(mpsc::Message::Sender(tx)) => self.sender = Some(tx.clone()),
            Message::Clicky => {
                if let Some(tx) = &mut self.sender {
                    tx.try_send(()).expect("Sender vanished!")
                } else {
                    panic!("Sender not set yet!")
                }
            }
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        mpsc::channel(512, 0).map(Message::Mpsc)
    }
}
