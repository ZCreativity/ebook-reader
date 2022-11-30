use crate::helper::config::{COVER_PLACEHOLDER, PADDING_LG};
use crate::model::book::Book;
use crate::model::library::Library;
use crate::AppState;

use druid::widget::{CrossAxisAlignment, Flex, Label, List, Padding};
use druid::widget::{FillStrat, Image, Scroll, Svg, ViewSwitcher};
use druid::Widget;

use druid::{Color, FontStyle};
use druid::{FontDescriptor, FontFamily, FontWeight, Insets, LensExt, WidgetExt};

use html2text::from_read_rich;
use html2text::render::text_renderer::{RichAnnotation, TaggedLine};
use std::collections::HashMap;

pub fn build_ui_book() -> impl Widget<AppState> {
    let books_texts = Scroll::new(List::new(book_text)).vertical();
    let books_texts_lens = books_texts.lens(AppState::library.then(Library::books));
    let layout = Flex::row().with_child(books_texts_lens);
    Padding::new(
        Insets::new(PADDING_LG, PADDING_LG, PADDING_LG, PADDING_LG),
        layout,
    )
}

fn book_text() -> impl Widget<Book> {
    let doc = Scroll::new(ViewSwitcher::new(
        |data: &Book, _env| data.get_doc().is_some(),
        move |f, data, _env| {
            if *f {
                let doc = data.get_doc().unwrap(); //Cosi prendo il clone fatto tramite Arc, lo unwrappo e ho il mutex
                let mut doc_mut = doc.lock().unwrap(); //Prendo il mutex, lo blocco, e poi posso usarlo
                let length = doc_mut.spine.len();
                let mut vect = Vec::<Vec<TaggedLine<Vec<RichAnnotation>>>>::new();

                println!("{:?}", doc_mut.resources);

                for _ in 0..length {
                    let page = doc_mut.get_current_str().unwrap();
                    vect.push(from_read_rich(page.as_bytes(), 100));
                    match doc_mut.go_next() {
                        Ok(_) => (),
                        Err(err) => println!("Error: {}", err),
                    }
                }

                let new_vector = vect.concat();
                let mut flex: Flex<Book> =
                    Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);

                for line in new_vector.iter() {
                    let mut h: i32 = 0;
                    let mut flag: bool = false;
                    let mut line_str = String::from("");
                    let mut tag_vect = Vec::<RichAnnotation>::new();

                    //If TaggedLine is not empty but does not have TaggedStrings => skip = true
                    //so that no useless lines are added
                    let skip: bool = line.iter().peekable().peek().is_some()
                        && line.tagged_strings().peekable().peek().is_none();

                    if !skip {
                        //Check elements in the vector of tagged string, each TaggedLine can contain multiple
                        //TaggedString(s), this loop set the h flags, add text to the label and produce a vector of
                        //RichAnnotation, after the loop we generate a Label with the overall text of the line
                        //and with the given attributes, like font_size, font_style etc...
                        for tagged_string in line.tagged_strings() {
                            //If h has not been set yet, check if possible h label is being handled,
                            //if a tag is already being handled (h > 0), just go ahead
                            if h == 0 {
                                (h, flag) = check_h(tagged_string.s.as_str());
                            }

                            //Each TaggedString can have multiple tags (uncommon), tag_vec makes a copy of the said vec
                            let tags = tagged_string.clone().tag;

                            //If not in a h specifier, add a label with given attributes, an h specifier with this library
                            //is formatted as a TaggedString with no tag before the actual string that need styling
                            //just checking h > 0 is not exhaustive, as we can have just normal string where h is 0
                            if !flag {
                                //Add the text to label
                                line_str = [line_str, tagged_string.s.clone()].join("");

                                //Else save all the flags in a vector
                                for tag in tags.iter() {
                                    tag_vect.push(tag.clone());
                                }
                            }

                            flag = false;
                        }

                        use RichAnnotation::*;

                        //If no tag are present, just append a simple Label with normal text
                        if tag_vect.is_empty() {
                            flex = no_tag(line_str.as_str(), flex, h);
                        }

                        //Else add Label with correct style
                        //TODO: add missing case
                        //TODO: if more than one tag, this doesn't work, it's gonna add multiple child (even if rare)
                        //TODO: lines with h should be centered
                        for tag in tag_vect.iter() {
                            match tag {
                                Default => {}
                                Link(link_str) => {
                                    flex = link(line_str.as_str(), flex, h);
                                    // Print the link as test
                                    println!("{}", link_str);
                                }
                                Image => (),
                                Emphasis => {
                                    flex = emphasis(line_str.as_str(), flex, h);
                                }
                                Strong => (),
                                Strikeout => (),
                                Code => (),
                                Preformat(_) => (),
                            }
                        }
                    }
                }

                Box::new(flex) //Dentro data ho il Book
            } else {
                Box::new(Flex::column())
            }
        },
    ));

    Flex::column().with_child(doc)
}

fn no_tag(s: &str, mut flex: Flex<Book>, h: i32) -> Flex<Book> {
    if h > 0 {
        flex = flex.with_child(h_label(s, h));
    } else {
        flex = flex.with_child(default(s));
    }

    flex
}

fn emphasis(s: &str, mut flex: Flex<Book>, h: i32) -> Flex<Book> {
    if h > 0 {
        flex = flex.with_child(h_label_emphasis(s, h));
    } else {
        flex = flex.with_child(default(s));
    }

    flex
}

fn link(s: &str, mut flex: Flex<Book>, h: i32) -> Flex<Book> {
    if h > 0 {
        flex = flex.with_child(h_label_link(s, h));
    } else {
        flex = flex.with_child(default_with_color(s, Color::AQUA));
    }

    flex
    //TODO: underline string
}

fn default(s: &str) -> impl Widget<Book> {
    Label::new(s)
}

fn default_with_color(s: &str, color: Color) -> impl Widget<Book> {
    Label::new(s).with_text_color(color)
}

fn h_label(s: &str, h: i32) -> Label<Book> {
    Label::new(s).with_font(
        FontDescriptor::new(FontFamily::SYSTEM_UI)
            .with_size(h_font_size(h))
            .with_weight(FontWeight::BOLD),
    )
}

fn h_label_emphasis(s: &str, h: i32) -> Label<Book> {
    Label::new(s).with_font(
        FontDescriptor::new(FontFamily::SYSTEM_UI)
            .with_size(h_font_size(h))
            .with_weight(FontWeight::BOLD)
            .with_style(FontStyle::Italic),
    )
}

fn h_label_link(s: &str, h: i32) -> Label<Book> {
    Label::new(s)
        .with_text_color(Color::AQUA)
        .with_text_size(h_font_size(h))
}

fn check_h(s: &str) -> (i32, bool) {
    match s {
        "# " => (1, true),
        "## " => (2, true),
        "### " => (3, true),
        "#### " => (4, true),
        "##### " => (5, true),
        "###### " => (6, true),
        _ => (0, false), //This is basically useless, just to have a, exhaustive match
    }
}

fn h_font_size(h: i32) -> f64 {
    // From Google
    // h1 | 2em    | 32px
    // h2 | 1.5em  | 24px
    // h3 | 1.17em | 18.72px
    // h4 | 1em    | 16px
    // h5 | 0.83em | 13.28px
    // h6 | 0.67em | 10.72px

    let h_sizes: HashMap<i32, f64> = HashMap::from([
        (1, 32_f64),
        (2, 24_f64),
        (3, 18.72_f64),
        (4, 16_f64),
        (5, 13.28_f64),
        (6, 10.72_f64),
    ]);

    *h_sizes.get(&h).unwrap()
}
