use super::directory_group_name;

#[test]
fn directory_group_names_support_windows_unix_and_chinese_paths() {
    assert_eq!(
        directory_group_name("C:\\Projects\\终端工具\\"),
        Some("终端工具".to_string())
    );
    assert_eq!(
        directory_group_name("/work/project/"),
        Some("project".to_string())
    );
    assert_eq!(
        directory_group_name("\\\\server\\share\\项目"),
        Some("项目".to_string())
    );
}

#[test]
fn directory_group_names_keep_roots_readable() {
    assert_eq!(directory_group_name("C:\\"), Some("C:\\".to_string()));
    assert_eq!(directory_group_name("/"), Some("/".to_string()));
    assert_eq!(directory_group_name("~"), Some("~".to_string()));
    assert_eq!(directory_group_name("  "), None);
}
