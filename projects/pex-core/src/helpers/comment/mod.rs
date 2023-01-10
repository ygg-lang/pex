use super::*;
use crate::StringView;

/// Parse a comment line, note this does not catch the newline,
/// catch all if reach [EOF]()
///
/// # Patterns
///
/// ```regex
/// # comment
/// // comment
/// ```
///
/// # Examples
///
/// ```
/// # use pex::{helpers::CommentLine, ParseState};
/// let test1 = ParseState::new("# comment hash\r text text");
/// let test2 = ParseState::new("// comment slash\n 123456");
/// assert_eq!(CommentLine::new("#")(test1).unwrap().body.as_string(), " comment hash");
/// assert_eq!(CommentLine::new("//")(test2).unwrap().body.as_string(), " comment slash");
/// ```
#[derive(Clone, Copy, Debug)]
pub struct CommentLine {
    /// The comment head
    pub head: &'static str,
}

impl CommentLine {
    /// Create a new comment line parser
    pub fn new(head: &'static str) -> Self {
        Self { head }
    }
}

impl<'i> FnOnce<(ParseState<'i>,)> for CommentLine {
    type Output = ParseResult<'i, SurroundPair<'i>>;
    #[inline]
    extern "rust-call" fn call_once(self, (input,): (ParseState<'i>,)) -> Self::Output {
        let (_, head) = input.match_str(self.head)?;
        let offset = match input.residual.find(&['\r', '\n']) {
            Some(s) => s,
            None => input.residual.len(),
        };
        // SAFETY: find offset always valid
        let body = unsafe { input.residual.get_unchecked(head.len()..offset) };
        input.advance(offset).finish(SurroundPair {
            head: StringView::new(head, input.start_offset),
            body: StringView::new(body, input.start_offset + head.len()),
            tail: StringView::new("", input.start_offset + offset),
        })
    }
}

/// Parse the comment block
///
/// # Patterns
///
/// ```regex
/// /** comment */
/// (* comment *)
/// ```
///
/// # Examples
///
/// ```
/// # use pex::{helpers::comment_block, ParseState};
/// let test1 = ParseState::new("(*  comment  *) 123456");
/// let test2 = ParseState::new("/** comment **/ 123456");
/// assert_eq!(comment_block(test1, "(*", "*)").unwrap().body.as_string(), "  comment  ");
/// assert_eq!(comment_block(test2, "/*", "*/").unwrap().body.as_string(), "* comment *");
/// ```
#[derive(Clone, Copy, Debug)]
pub struct CommentBlock {
    /// The comment head
    pub head: &'static str,
    /// The comment tail
    pub tail: &'static str,
    /// Whether the comment is nested
    pub nested: bool,
}

impl<'i> FnOnce<(ParseState<'i>,)> for CommentBlock {
    type Output = ParseResult<'i, SurroundPair<'i>>;
    #[inline]
    extern "rust-call" fn call_once(self, (input,): (ParseState<'i>,)) -> Self::Output {
        let (state, _) = input.match_str(self.head)?;
        match self.nested {
            true => {
                todo!()
            }
            false => {
                match state.residual.find(self.tail) {
                    Some(s) => {
                        let offset = s + self.tail.len();
                        // SAFETY: find offset always valid
                        let body = unsafe { state.residual.get_unchecked(self.head.len()..s) };
                        state.advance(offset).finish(SurroundPair {
                            head: StringView::new(self.head, input.start_offset),
                            body: StringView::new(body, input.start_offset + self.head.len()),
                            tail: StringView::new(self.tail, input.start_offset + offset),
                        })
                    }
                    None => StopBecause::missing_string(self.tail, state.start_offset)?,
                }
            }
        }
    }
}
