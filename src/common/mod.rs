#[inline(always)]
fn is_double_quote(c: char) -> bool {
    matches!(c, '"')
}

#[inline(always)]
pub(crate) fn is_not_double_quote(c: char) -> bool {
    !is_double_quote(c)
}

#[inline(always)]
pub(crate) fn is_whitesapce(c: char) -> bool {
    matches!(c, ' ')
}

#[inline(always)]
pub(crate) fn is_line(c: char) -> bool {
    matches!(c, '\n')
}

#[inline(always)]
pub(crate) fn is_not_line(c: char) -> bool {
    !is_line(c)
}

#[inline(always)]
fn is_symbol(c: char) -> bool {
    matches!(
        c,
        '{' | '}' | '[' | ']' | '<' | '>' | '=' | '!' | ':' | '.' | '@'
    )
}

#[inline(always)]
fn is_boundary(c: char) -> bool {
    is_whitesapce(c) || is_line(c) || is_symbol(c)
}

#[inline(always)]
pub(crate) fn is_not_boundary(c: char) -> bool {
    !is_boundary(c)
}

#[inline(always)]
pub(crate) fn is_numeric(c: char) -> bool {
    matches!(c, '0'..='9')
}
