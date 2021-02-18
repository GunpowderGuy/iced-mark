use iced::{Element, Sandbox, Settings, Text};

mod mark;

/*fn main() {
    //let markdown_input = "Hello world, this is a ~~complicated~~ *very simple* example.";
    let markdown_input = " *italic* **bold** ~~complicated~~";
    let result = mark::parseo(markdown_input);

    println!("Hello, world2! {}", result);
}*/

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
        let markdown_input = //" ~~complicated~~ *italic* **bold**  

        "![alt text](resources/ferris.png \"Logo Title Text 1\")";

        // print!("{}", mark::semi_parseo(markdown_input));
        mark::parseo(markdown_input)
    }
}
