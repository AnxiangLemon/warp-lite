use super::PtyWriteMode;

#[test]
fn raw_input_preserves_arbitrary_bytes() {
    assert_eq!(
        PtyWriteMode::Raw.decorate_bytes(b"\0\xff\r\n".to_vec(), true),
        b"\0\xff\r\n"
    );
}

#[test]
fn block_input_uses_bracketed_paste_when_enabled() {
    assert_eq!(
        PtyWriteMode::Block.decorate_bytes(b"printf 'one\ntwo'".to_vec(), true),
        b"\x1b[200~printf 'one\ntwo'\x1b[201~"
    );
}

#[test]
fn block_input_preserves_bytes_without_bracketed_paste() {
    assert_eq!(
        PtyWriteMode::Block.decorate_bytes(b"echo hello\r\n".to_vec(), false),
        b"echo hello\r\n"
    );
}

#[test]
fn empty_block_keeps_the_bracketed_paste_pair() {
    assert_eq!(
        PtyWriteMode::Block.decorate_bytes(Vec::new(), true),
        b"\x1b[200~\x1b[201~"
    );
}

#[cfg(target_os = "windows")]
#[test]
fn line_input_moves_to_start_and_submits_with_carriage_return() {
    assert_eq!(
        PtyWriteMode::Line.decorate_bytes(b"echo hello".to_vec(), true),
        b"\x01echo hello\r"
    );
}

#[cfg(not(target_os = "windows"))]
#[test]
fn line_input_moves_to_start_and_submits_with_line_feed() {
    assert_eq!(
        PtyWriteMode::Line.decorate_bytes(b"echo hello".to_vec(), true),
        b"\x01echo hello\n"
    );
}
