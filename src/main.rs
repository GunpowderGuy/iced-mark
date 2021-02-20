use iced::{Element, Sandbox, Settings};

mod mark;

pub fn main() -> iced::Result {
    Hello::run(Settings::default())
}

struct Hello;

impl Sandbox for Hello {
    type Message = ();

    fn new() -> Hello {
        Hello
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(&mut self, _message: Self::Message) {
        // This application has no interactions
    }

    fn view(&mut self) -> Element<Self::Message> {
        //Text::new("Hello, world!").into()
        let _markdown_input = "~~complicated~~ *italic* **bold**  

        ![](resources/ferris.png \"Logo Title Text 1\")
             
        ~~simplex~~ *italic* **bold** ";

        let markdown_input = fs::read_to_string("resources/marco.markdown")
            .expect("Something went wrong reading the file");

        //print!("{}", mark::semi_parseo(markdown_input));
        mark::parseo(&markdown_input)
    }
}

use std::fs;
