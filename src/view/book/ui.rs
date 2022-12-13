use crate::controller::app_delegate::CLOSE_BOOK;
use crate::helper::config::{DISPLAY_WIDTH, PADDING_LG};
use crate::model::book::Book;
use druid::widget::{
    Button, CrossAxisAlignment, Flex, Label, MainAxisAlignment, Padding, SizedBox,
};
use druid::widget::{Scroll, ViewSwitcher};
use druid::{Color, FontStyle};
use druid::{FontDescriptor, FontFamily, FontWeight, Insets};
use druid::{Widget, WidgetExt};
use html2text::from_read_rich;
use html2text::render::text_renderer::{RichAnnotation, TaggedLine};
use std::collections::HashMap;

pub fn book_view() -> impl Widget<Book> {
    // Return a widget that can be used to display a book
    let book_text = Scroll::new(book_text()).vertical().fix_height(550.0);
    let book_menu = book_menu();
    let book_controls = book_controls();

    Padding::new(
        Insets::new(PADDING_LG, PADDING_LG, PADDING_LG, PADDING_LG),
        Flex::column()
            .with_child(book_menu)
            .with_child(book_text)
            .with_child(book_controls),
    )
}

fn book_menu() -> impl Widget<Book> {
    let back_button = Button::new("Back").on_click(|ctx, _data: &mut Book, _env| {
        println!("Going back");
        ctx.submit_command(CLOSE_BOOK);
    });

    let edit_button = Button::new("Edit").on_click(|ctx, _data: &mut Book, _env| {
        println!("Editing book");
        // For now just close the book
        ctx.submit_command(CLOSE_BOOK);
    });

    let change_font_button = Button::new("Change font").on_click(|ctx, _data: &mut Book, _env| {
        println!("Changing font");
        // For now just close the book
        ctx.submit_command(CLOSE_BOOK);
    });

    let flex = Flex::row()
        .with_child(back_button)
        .with_child(edit_button)
        .with_child(change_font_button)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .must_fill_main_axis(true)
        .padding(Insets::new(0.0, 0.0, 0.0, PADDING_LG));

    flex
}

fn book_controls() -> impl Widget<Book> {
    let control_next = ViewSwitcher::new(
        |data: &Book, _env| data.has_next_page(),
        |f, _data, _env| {
            if *f {
                Box::new(Button::new("Next").on_click(|ctx, data: &mut Book, _env| {
                    data.next_page();
                    ctx.request_update();
                }))
            } else {
                Box::new(SizedBox::empty())
            }
        },
    );

    let control_prev = ViewSwitcher::new(
        |data: &Book, _env| data.has_prev_page(),
        |f, _data, _env| {
            if *f {
                Box::new(Button::new("Prev").on_click(|ctx, data: &mut Book, _env| {
                    data.prev_page();
                    ctx.request_update();
                }))
            } else {
                Box::new(SizedBox::empty())
            }
        },
    );

    Flex::row()
        .with_child(control_prev)
        .with_child(control_next)
        .must_fill_main_axis(true)
        .main_axis_alignment(MainAxisAlignment::End)
}

fn book_text() -> impl Widget<Book> {
    let doc = Scroll::new(ViewSwitcher::new(
        |data: &Book, _env| data.get_doc().is_some() && data.get_current_page() > 0,
        move |f, data, _env| {
            if *f {
                let doc = data.get_doc().unwrap(); //Cosi prendo il clone fatto tramite Arc, lo unwrappo e ho il mutex
                let mut doc_mut = doc.lock().unwrap(); //Prendo il mutex, lo blocco, e poi posso usarlo
                doc_mut.set_current_page(data.get_current_page()).unwrap(); // Setto la pagina corrente a 0
                let mut vect = Vec::<Vec<TaggedLine<Vec<RichAnnotation>>>>::new();

                let page = doc_mut.get_current_str().unwrap();
                vect.push(from_read_rich(page.as_bytes(), 100));
                match doc_mut.go_next() {
                    Ok(_) => (),
                    Err(err) => println!("Error: {}", err),
                }

                let new_vector = vect.concat();
                let mut flex: Flex<Book> =
                    Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);

                // Render
                for (i, line) in new_vector.iter().enumerate() {
                    if i < 1000 {
                        println!("Line {} -> {:?}", i, line);
                    }

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

                Box::new(flex) // Dentro data ho il Book
            } else {
                Box::new(Flex::column())
            }
        },
    ))
    .fix_width(DISPLAY_WIDTH - PADDING_LG * 2.0);

    Flex::column().with_child(doc)
}

// fn book_chapter() -> impl Widget<Book> {}

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
        flex = flex.with_child(default_with_descriptor(
            s,
            FontDescriptor::new(FontFamily::SYSTEM_UI)
                .with_style(FontStyle::Italic)
                .with_size(16_f64),
        ));
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

fn default_with_descriptor(s: &str, descr: FontDescriptor) -> impl Widget<Book> {
    Label::new(s).with_font(descr)
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
