#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "runtime", derive(strum_macros::EnumDiscriminants))]
pub enum PtyWriteMode {
    #[default]
    Raw,
    Line,
    Block,
}

impl PtyWriteMode {
    /// Decorates input bytes according to the write mode.
    pub fn decorate_bytes(
        self,
        bytes: impl Into<Vec<u8>>,
        is_bracketed_paste_enabled: bool,
    ) -> Vec<u8> {
        let bytes = bytes.into();
        match self {
            Self::Raw => bytes,
            Self::Line => {
                let mut decorated = Vec::with_capacity(bytes.len() + 2);
                // Readline and prompt-toolkit use SOH to move to the beginning of the line.
                decorated.push(b'\x01');
                decorated.extend_from_slice(&bytes);
                decorated.push(if cfg!(target_os = "windows") {
                    b'\r'
                } else {
                    b'\n'
                });
                decorated
            }
            Self::Block => {
                if is_bracketed_paste_enabled {
                    b"\x1b[200~"
                        .iter()
                        .copied()
                        .chain(bytes)
                        .chain(b"\x1b[201~".iter().copied())
                        .collect()
                } else {
                    bytes
                }
            }
        }
    }
}

#[cfg(test)]
#[path = "pty_tests.rs"]
mod tests;
