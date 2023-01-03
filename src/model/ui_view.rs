// Here you define your view. It can be any type that implements `Hash`. You can define an Enum
// instead and use that to define your views instead of a string
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum UiView {
    Library,
    BookRead,
    BookEdit,
}
