use iced::widget::{button, column, row};
use iced::widget::container;
use iced::widget::Svg;
use iced::{Element, Length, Sandbox, Settings};

pub const SVG_PATH: &str =
    "graph_svg/graph.svg";

pub fn run_app() {
    PlotterWindow::run(Settings::default())
        .expect("Failed running the iced application(sandbox)");
}

struct PlotterWindow {
    state: State,
}

enum State {
    Empty,
    Populated,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    PlotGraph,
    Reset,
}

impl Sandbox for PlotterWindow {
    type Message = Message;
    fn title(&self) -> String {
        "Iced-Plotters plotter".to_string()
    }
    fn new() -> Self {
        Self {
            state: State::Empty,
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Reset => {
                println!("Reset!");
                self.state = State::Empty
            }
            Message::PlotGraph => self.state = State::Populated,
        }
    }

    fn view(&self) -> Element<Message> {
        let svg = Svg::from_path(SVG_PATH)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(iced::theme::Svg::default());
        container(
            column!(
                svg,
                row!(
                    button("Reset").on_press(Message::Reset).width(100),
                    button("Plot").on_press(Message::PlotGraph).width(100)
                ).spacing(100)
            ).align_items(iced::Alignment::Center)

        )
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }
}
