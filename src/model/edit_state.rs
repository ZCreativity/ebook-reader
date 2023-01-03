// use std::sync::Arc;

// use druid::{widget::ScopeTransfer, Data, Lens};

// use super::{app_state::AppState, book::Book};

// // this holds state that will be used when on the edit page
// #[derive(Clone, Data, Lens, Debug)]
// pub struct EditState {
//     pub book: Book,
//     pub index: usize,
//     pub was_saved: bool,
// }

// impl EditState {
//     pub fn new(data: AppState) -> Self {
//         let (contact, index) = if let Some(idx) = data.selected {
//             (data.contacts[idx].clone(), idx)
//         } else {
//             (
//                 Contact::new("".to_owned(), "".to_owned(), 31, "".to_owned()),
//                 0,
//             )
//         };
//         Self {
//             contact,
//             index,
//             was_saved: false,
//         }
//     }
// }

// pub struct EditTransfer;

// impl ScopeTransfer for EditTransfer {
//     type In = AppState;

//     type State = EditState;

//     fn read_input(&self, state: &mut Self::State, inner: &Self::In) {
//         // only read data in if the input was saved
//         if state.was_saved {
//             let selected = inner.selected;
//             let idx = if let Some(idx) = selected { idx } else { 0 };
//             state.contact = inner.contacts[idx].clone();
//             state.index = idx;
//             state.was_saved = false;
//         }
//     }

//     fn write_back_input(&self, state: &Self::State, inner: &mut Self::In) {
//         if state.was_saved {
//             let contacts = Arc::make_mut(&mut inner.contacts);
//             contacts[state.index] = state.contact.clone();
//             inner.contacts = Arc::new(contacts.to_owned());
//         }
//     }
// }
