use pulldown_cmark::{Event, Options, Parser, Tag};
extern crate math_text_transform;

use math_text_transform::MathTextTransform;

use iced::{Column, Container, Element, Image, Text, TextInput};

fn transformar(stri: &str, estado: &Tag) -> String {
    match estado {
        Tag::Emphasis => stri.to_math_italic(),
        Tag::Strong => stri.to_math_bold(),
        Tag::Strikethrough => stri.chars().fold(String::new(), |mut x: String, y: char| {
            x.push(y);
            x.push('̶');
            x
        }),
        _ => stri.to_string(),
    }
}

fn semi_parseo(markdown_input: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(markdown_input, options);

    let mut estado: Option<Tag> = None;

    parser.fold(String::from(""), |stri, x| {
        let mut temporal: String = "".to_string();

        match x {
            Event::Start(variante) => estado = Some(variante),
            Event::Text(contenido) => {
                temporal = contenido.to_string();
                if let Some(i) = &estado {
                    temporal = transformar(&contenido.to_string(), &i);
                    estado = None;
                }
            }
            _ => (),
        }

        stri + &temporal
    })
}

enum Message {}

fn parseo(markdown_input0: &str) -> Element<Message> {
    let markdown_input = semi_parseo(markdown_input0);
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&markdown_input, options);

    let mut estado: Option<Tag> = None;

    parser
        .fold(Column::new(), |colu, x| match x {
            Event::Start(variante) => match variante {
                Tag::Image(a, link, c) => colu.push(Image::new(link.to_string())),
                _ => colu,
            },
            _ => colu,
        })
        .into()
}
