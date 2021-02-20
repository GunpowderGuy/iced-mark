use pulldown_cmark::{Event, Options, Parser, Tag};
extern crate math_text_transform;

use math_text_transform::MathTextTransform;

use iced::{Column, Element, Image, Text};

fn _transformar(stri: &str, estado: &Tag) -> String {
    match estado {
        Tag::Emphasis => stri.to_math_italic(),
        Tag::Strong => stri.to_math_bold(),
        Tag::Strikethrough => stri.chars().fold('̶'.to_string(), |mut x: String, y: char| {
            x.push(y);
            x.push('̶');
            x
        }),
        _ => stri.to_string(),
    }
}

pub fn semi_parseo(markdown_input: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(markdown_input, options);

    let resultado: Vec<Event> = Parser::new_ext(markdown_input, options).collect();

    println!("{:?}", resultado);

    let mut estado: Option<Tag> = None;

    parser.fold(String::from(""), |stri, x| {
        let mut temporal: String = "".to_string();

        match x {
            Event::Start(variante) => estado = Some(variante),
            Event::Text(contenido) => {
                temporal = contenido.to_string();
                if let Some(i) = &estado {
                    temporal = _transformar(&contenido.to_string(), &i);
                    estado = None;
                }
            }
            _ => (),
        }

        stri + &temporal
    })
}

pub fn parseo<T: 'static>(markdown_input: &str) -> Element<T> {
    let markdown_input = semi_parseo(markdown_input);

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&markdown_input, options);

    //let estado: Option<Tag> = None;

    parser
        .fold(Column::new(), |colu, x| match x {
            Event::Start(variante) => match variante {
                Tag::Image(_a, link, _c) => colu.push(Image::new(link.to_string())),
                _ => colu,
            },
            Event::Text(contenido) => colu.push(Text::new(contenido.to_string())),
            _ => colu,
        })
        .into()
}
