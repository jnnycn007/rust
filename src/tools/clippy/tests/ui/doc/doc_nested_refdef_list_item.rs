// https://github.com/rust-lang/rust/issues/133150
#![warn(clippy::doc_nested_refdefs)]
#[rustfmt::skip]
/// - [link]: def
//~^ ERROR: link reference defined in list item
///
/// - [link]: def (title)
//~^ ERROR: link reference defined in list item
///
/// - [link]: def "title"
//~^ ERROR: link reference defined in list item
///
/// - [link]: not def
///
/// - [link][]: notdef
///
/// - [link]\: notdef
pub struct Empty;

#[rustfmt::skip]
/// - [link]: def
//~^ ERROR: link reference defined in list item
/// - [link]: def (title)
//~^ ERROR: link reference defined in list item
/// - [link]: def "title"
//~^ ERROR: link reference defined in list item
/// - [link]: not def
/// - [link][]: notdef
/// - [link]\: notdef
pub struct EmptyTight;

#[rustfmt::skip]
/// - [link]: def
//~^ ERROR: link reference defined in list item
///   inner text
///
/// - [link]: def (title)
//~^ ERROR: link reference defined in list item
///   inner text
///
/// - [link]: def "title"
//~^ ERROR: link reference defined in list item
///   inner text
///
/// - [link]: not def
///   inner text
///
/// - [link][]: notdef
///   inner text
///
/// - [link]\: notdef
///   inner text
pub struct NotEmpty;

#[rustfmt::skip]
/// - [link]: def
//~^ ERROR: link reference defined in list item
///   inner text
/// - [link]: def (title)
//~^ ERROR: link reference defined in list item
///   inner text
/// - [link]: def "title"
//~^ ERROR: link reference defined in list item
///   inner text
/// - [link]: not def
///   inner text
/// - [link][]: notdef
///   inner text
/// - [link]\: notdef
///   inner text
pub struct NotEmptyTight;

/// ## Heading
///
/// - [x] - Done
/// - [ ] - Not Done
pub struct GithubCheckboxes;
