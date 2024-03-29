use std::{collections::HashMap, rc::Rc};

use druid::{
    widget::{Flex, Label, ViewSwitcher},
    Color, FontDescriptor, FontFamily, FontStyle, FontWeight, Widget, WidgetExt,
};

use html2text::{
    from_read_rich,
    render::text_renderer::{RichAnnotation, TaggedLine},
};

use crate::{helper::config::DEFAULT_FONT_SIZE, model::app_state::AppState};

/**
 * Parses a string (in html format) and returns a widget that can be used to display the text
 * The html text is parsed using the html2text crate.
 * Every element of the html is at the end converted to a Label widget with specific styling
 * based on the html tag.
 */
pub fn parse(page: String) -> impl Widget<AppState> {
    let view_switcher = ViewSwitcher::new(
        |data: &AppState, _env| data.get_font_size(),
        move |font_size_offset, _data, _env| {
            let mut flex =
                Flex::column().cross_axis_alignment(druid::widget::CrossAxisAlignment::Start);
            let mut vect = Vec::<Vec<TaggedLine<Vec<RichAnnotation>>>>::new();
            vect.push(from_read_rich(page.as_bytes(), 150));

            let new_vector = vect.concat();
            // Render
            for (_i, line) in new_vector.iter().enumerate() {
                let mut h: i32 = 0;
                let mut flag: bool = false;
                let mut line_str = String::from("");
                let mut tag_vect = Vec::<RichAnnotation>::new();

                //println!("Line: {:?}", line);

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
                        let no_tag = no_tag(line_str.as_str(), h, *font_size_offset);
                        flex.add_child(no_tag);
                    }

                    //Else add Label with correct style
                    //TODO: add missing case
                    //TODO: if more than one tag, this doesn't work, it's gonna add multiple child (even if rare)
                    //TODO: lines with h should be centered
                    for tag in tag_vect.iter() {
                        match tag {
                            Default => {}
                            Link(link_value) => {
                                //println!("link_value: {:?}", link_value);
                                let link = link(
                                    line_str.as_str(),
                                    h,
                                    *font_size_offset,
                                    link_value.to_string(),
                                );
                                flex.add_child(link);
                            }
                            Image => (),
                            Emphasis => {
                                let emphasis = emphasis(line_str.as_str(), h, *font_size_offset);
                                flex.add_child(emphasis);
                            }
                            Strong => (),
                            Strikeout => (),
                            Code => (),
                            Preformat(_) => (),
                        }
                    }
                }
            }

            Box::new(flex)
        },
    );

    view_switcher
}

pub fn no_tag(s: &str, h: i32, font_size_offset: f64) -> Label<AppState> {
    if h > 0 {
        return h_label(s, h, font_size_offset);
    }
    default(s, font_size_offset)
}

pub fn emphasis(s: &str, h: i32, font_size_offset: f64) -> Label<AppState> {
    if h > 0 {
        return h_label_emphasis(s, h, font_size_offset);
    }
    default_with_descriptor(
        s,
        FontDescriptor::new(FontFamily::SYSTEM_UI)
            .with_style(FontStyle::Italic)
            .with_size(DEFAULT_FONT_SIZE + font_size_offset),
    )
}

pub fn link(s: &str, h: i32, font_size_offset: f64, link: String) -> impl Widget<AppState> {
    let link_ref = Rc::new(link.clone());
    if h > 0 {
        return h_label_link(s, h, font_size_offset).on_click(move |_ctx, data, _env| {
            data.navigate_to_page(link_ref.clone());
        });
    }
    default_with_color(s, Color::AQUA, font_size_offset).on_click(move |_ctx, data, _env| {
        data.navigate_to_page(link_ref.clone());
    })
}

pub fn default(s: &str, font_size_offset: f64) -> Label<AppState> {
    Label::new(s).with_text_size(DEFAULT_FONT_SIZE + font_size_offset)
}

pub fn default_with_color(s: &str, color: Color, font_size_offset: f64) -> Label<AppState> {
    Label::new(s)
        .with_text_color(color)
        .with_text_size(DEFAULT_FONT_SIZE + font_size_offset)
}

pub fn default_with_descriptor(s: &str, descr: FontDescriptor) -> Label<AppState> {
    Label::new(s).with_font(descr)
}

pub fn h_label(s: &str, h: i32, font_size_offset: f64) -> Label<AppState> {
    Label::new(s).with_font(
        FontDescriptor::new(FontFamily::SANS_SERIF)
            .with_size(h_font_size(h) + font_size_offset)
            .with_weight(FontWeight::BOLD),
    )
}

pub fn h_label_emphasis(s: &str, h: i32, font_size_offset: f64) -> Label<AppState> {
    Label::new(s).with_font(
        FontDescriptor::new(FontFamily::SYSTEM_UI)
            .with_size(h_font_size(h) + font_size_offset)
            .with_weight(FontWeight::BOLD)
            .with_style(FontStyle::Italic),
    )
}

pub fn h_label_link(s: &str, h: i32, font_size_offset: f64) -> Label<AppState> {
    Label::new(s)
        .with_text_color(Color::AQUA)
        .with_text_size(h_font_size(h) + font_size_offset)
}

pub fn check_h(s: &str) -> (i32, bool) {
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

pub fn h_font_size(h: i32) -> f64 {
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

    *h_sizes.get(&h).unwrap_or(&10.72_f64)
}
