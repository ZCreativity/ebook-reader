// let view_switcher = ViewSwitcher::new(
//     |data: &AppState, _env| data.current_view,
//     |selector, data, _env| match selector {
//         0 => {
//             let mut book_list_layout = Flex::column();

//             let label = Padding::new(20.0, Label::new("Libri Disponibili")
//                 .with_font(UI_FONT_BOLD)
//                 .with_text_color(BLUE_200)
//                 .with_text_size(30.0));

//             book_list_layout.add_child(Align::new(UnitPoint::CENTER, label));

//             let mut book_list = List::new(|| {

//                 let mut single_book = Flex::row();

//                 let sized_box = SizedBox::new(Label::new(|book: &Book, _: &_| {
//                     let mut title = book.get_title();
//                     ///TODO: aggiungere anche il timestamp
//                     if book.get_path().contains("modified"){
//                         title.push_str(" modified");
//                     }
//                     title
//                 })
//                     .with_text_color(theme::BLUE_200)
//                     .on_click(move |ctx: &mut EventCtx, book: &mut Book, _env: &Env| {
//                         ctx.submit_command(cmd::OPEN_BOOK.with(book.clone()));
//                     })
//                 );

//                 single_book.add_flex_child(sized_box, 1.0);
//                 single_book
//             }).lens(AppState::books).padding(PADDING);

//             book_list_layout.add_flex_child(Scroll::new(book_list).vertical(), 1.0);

//             return Box::new(book_list_layout);
//         }
//         1 => {
//             let button_up_row = Flex::row()
//                 .with_child(home_button_widget())
//                 .with_child(customization_widget())
//                 .with_child(double_page_widget())
//                 .with_child(edit_widget())

//                 .main_axis_alignment(MainAxisAlignment::SpaceEvenly);

//             let button_below_row = Flex::row()
//                 .with_child(previous_page_button())
//                 .with_child(next_page_button())
//                 .main_axis_alignment(MainAxisAlignment::SpaceEvenly);

//             let mut window = Flex::column()

//                 .with_child(button_up_row)
//                 .with_flex_child(build_page(data.current_book.as_ref().unwrap(), data.current_view), 1.0)
//                 .with_child(button_below_row)
//                 .padding(theme::grid(2.0));

//             Box::new(window)
//         }
//         2 => {
//             let button_down_row = Flex::row()
//                 .with_child(previous_page_button())
//                 .with_child(next_page_button())
//                 .main_axis_alignment(MainAxisAlignment::SpaceEvenly);

//             let button_up_row = Flex::row()
//                 .with_child(home_button_widget())
//                 .with_child(customization_widget())
//                 .with_child(single_page_widget());

//             let mut window1 = Flex::column()
//                 .with_flex_child(build_page(data.current_book.as_ref().unwrap(), 1), 1.0)

//                 .padding(theme::grid(2.0));

//             let mut window2 = Flex::column()
//                 .with_flex_child(build_page(data.current_book.as_ref().unwrap(), data.current_view), 1.0)
//                 .padding(theme::grid(2.0));
//             //.debug_paint_layout();
//             let res = Flex::column()
//                 .with_child(button_up_row)
//                 .with_flex_child(Split::columns(window1.expand(), window2.expand()), 1.0)
//                 .with_child(button_down_row);
//             //Flex::row().with_flex_child(window1, 1.0).with_flex_child(window2, 1.0),1.0

//             Box::new(res)
//         }
//         //correttore di bozza
//         3 => {
//             let button_up_row = Flex::row()
//                 .with_child(home_button_widget());
//                 let button_change_paragraph = Flex::row()
//                 .with_child(previous_par_button())
//                 .with_child(next_par_button())
//                 .main_axis_alignment(MainAxisAlignment::SpaceEvenly);
//             let save_row = Flex::row()
//                 .with_child(Button::from_label(Label::new("Save").with_text_color(theme::BLUE_200))
//                                 .on_click(move |ctx: &mut EventCtx, _, _env: &Env| {
//                                     ctx.submit_command(SAVE_EDIT);
//                                 }));

//             let mut window1 = Flex::column()
//                 .with_flex_child(
//                     build_page(data.current_book.as_ref().unwrap(), 3), 1.0)
//                 .with_child(button_change_paragraph)
//                 .expand().padding(theme::grid(2.0));

//             let mut window2 = Flex::column()
//                 .with_flex_child(
//                     TextBox::multiline()
//                         .with_text_color(BLUE_200).lens(AppState::raw_page)
//                         .expand()
//                         .padding(5.0), 1.0)
//                 .with_child(save_row)
//                 .padding(theme::grid(2.0));

//             let res = Flex::column()
//                 .with_child(button_up_row)
//                 .with_flex_child(Flex::row().with_flex_child(window1, 1.0).with_flex_child(window2, 1.0), 1.0)
//                              ;

//             Box::new(res)
//         }

//         _ => Box::new(Label::new("Something wrong happened").center()),
//     },
// ).controller(OnClick);

// layout.add_flex_child(view_switcher, (1.0));
// layout
