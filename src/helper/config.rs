use druid::{FontDescriptor, FontFamily, FontWeight};

/* UI */
pub const DISPLAY_WIDTH: f64 = 1000.0;
pub const DISPLAY_HEIGHT: f64 = 800.0;
pub const APP_NAME: &str = "EBook Reader";
pub const TITLE_SIZE: f64 = 24.0;
//pub const AUTHOR_SIZE: f64 = 14.0;
// const ALERT_SIZE: f64 = 32.0;
// pub const PADDING_SM: f64 = 8.0;
// pub const PADDING_MD: f64 = 16.0;
pub const PADDING_LG: f64 = 32.0;
// pub const PADDING_XL: f64 = 64.0;

/* Text Styles */
pub const TITLE: FontDescriptor = FontDescriptor::new(FontFamily::SERIF)
    .with_size(TITLE_SIZE)
    .with_weight(FontWeight::BOLD);

/*pub const AUTHOR: FontDescriptor = FontDescriptor::new(FontFamily::SERIF)
.with_size(AUTHOR_SIZE)
.with_style(FontStyle::Italic);*/

// pub const ALERT: FontDescriptor = FontDescriptor::new(FontFamily::SANS_SERIF)
//     .with_size(ALERT_SIZE)
//     .with_weight(FontWeight::BOLD);

// From Google
// h1 | 2em    | 32px
// h2 | 1.5em  | 24px
// h3 | 1.17em | 18.72px
// h4 | 1em    | 16px
// h5 | 0.83em | 13.28px
// h6 | 0.67em | 10.72px

// pub let h_sizes: HashMap<i32, f64> = HashMap::from([
//     (1, 32_f64),
//     (2, 24_f64),
//     (3, 18.72_f64),
//     (4, 16_f64),
//     (5, 13.28_f64),
//     (6, 10.72_f64),
// ]);

/* Data */
pub const LIBRARY_PATH: &str = "./src/library";
pub const COVERS_PATH: &str = "./src/library/covers/";

/* Icons */

/* Cover placeholder */
pub const COVER_PLACEHOLDER: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>
<!-- Created with Inkscape (http://www.inkscape.org/) -->
<svg xmlns:svg=\"http://www.w3.org/2000/svg\" xmlns=\"http://www.w3.org/2000/svg\" version=\"1.0\" width=\"200\" height=\"250\" id=\"svg2\">
  <defs id=\"defs5\"/>
  <rect x=\"0\" y=\"0\" width=\"200\" height=\"250\" style=\"fill:#d4d4d4\" id=\"rect1310\"/>
  <path d=\"M 64.255859,62.075068 L 63.136719,62.075068 C 63.128904,62.047727 63.092772,61.948118 63.02832,61.77624 C 62.963866,61.604368 62.931639,61.453977 62.931641,61.325068 C 62.931639,61.078978 62.965819,60.84265 63.03418,60.616084 C 63.102537,60.389525 63.203123,60.177611 63.335938,59.980341 C 63.468748,59.783081 63.731443,59.467651 64.124023,59.034052 C 64.516598,58.600465 64.712887,58.243043 64.712891,57.961787 C 64.712887,57.414919 64.355466,57.141482 63.640625,57.141474 C 63.292967,57.141482 62.929686,57.31531 62.550781,57.662959 L 61.947266,56.532099 C 62.451171,56.137576 63.113279,55.940311 63.933594,55.940302 C 64.566403,55.940311 65.094723,56.116092 65.518555,56.467646 C 65.942378,56.819216 66.154292,57.286013 66.154297,57.868037 C 66.154292,58.266481 66.077144,58.603394 65.922852,58.878779 C 65.76855,59.154175 65.497066,59.477417 65.108398,59.848505 C 64.719723,60.219604 64.466794,60.528197 64.349609,60.774287 C 64.232419,61.020384 64.173825,61.289915 64.173828,61.58288 C 64.173825,61.645383 64.201169,61.809446 64.255859,62.075068 L 64.255859,62.075068 z M 63.757813,62.871943 C 64.023435,62.871945 64.249021,62.965695 64.43457,63.153193 C 64.620114,63.340694 64.712887,63.567257 64.712891,63.83288 C 64.712887,64.098506 64.620114,64.325068 64.43457,64.512568 C 64.249021,64.700068 64.023435,64.793818 63.757813,64.793818 C 63.492185,64.793818 63.265623,64.700068 63.078125,64.512568 C 62.890623,64.325068 62.796874,64.098506 62.796875,63.83288 C 62.796874,63.567257 62.890623,63.340694 63.078125,63.153193 C 63.265623,62.965695 63.492185,62.871945 63.757813,62.871943 L 63.757813,62.871943 z \" transform=\"matrix(10.52848,0,0,10.52848,-561.8574,-560.5734)\" style=\"font-size:12px;font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;text-align:start;line-height:125%;writing-mode:lr-tb;text-anchor:start;fill:white;font-family:Trebuchet MS\" id=\"flowRoot1875\"/>
</svg>";
