#[inline(always)]
fn is_double_quote(c: u8) -> bool {
    matches!(c, b'"')
}

#[inline(always)]
pub(crate) fn is_not_double_quote(c: u8) -> bool {
    !is_double_quote(c)
}

#[inline(always)]
pub(crate) fn is_whitesapce(c: u8) -> bool {
    matches!(c, b' ')
}

#[inline(always)]
pub(crate) fn is_line(c: u8) -> bool {
    matches!(c, b'\n')
}

#[inline(always)]
pub(crate) fn is_not_line(c: u8) -> bool {
    !is_line(c)
}

#[inline(always)]
fn is_symbol(c: u8) -> bool {
    matches!(
        c,
        b'{' | b'}' | b'[' | b']' | b'<' | b'>' | b'=' | b'!' | b':' | b'.' | b'@'
    )
}

#[inline(always)]
fn is_boundary(c: u8) -> bool {
    is_whitesapce(c) || is_line(c) || is_symbol(c)
}

#[inline(always)]
pub(crate) fn is_not_boundary(c: u8) -> bool {
    !is_boundary(c)
}

#[inline(always)]
pub(crate) fn is_numeric(c: u8) -> bool {
    matches!(c, b'0'..=b'9')
}
