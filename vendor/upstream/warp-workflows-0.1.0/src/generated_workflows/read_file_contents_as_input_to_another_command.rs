use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Read file contents as input to another command"####.into(),
        command: r####"{{command}} < {{file}}"####.into(),
        tags: vec![r####"shell"####.into()].into_iter().collect(),
        description: Some(
            r####"Reads the contents within file and passes it as input to the command."####.into(),
        ),
        arguments: vec![
            Argument {
                name: r####"command"####.into(),
                description: Some(
                    r####"The command whose input should be read from a file."####.into(),
                ),
                default_value: None,
            },
            Argument {
                name: r####"file"####.into(),
                description: Some(
                    r####"The path to the file from where the input should be read."####.into(),
                ),
                default_value: None,
            },
        ]
        .into_iter()
        .collect(),
        source_url: Some(r####"https://tldp.org/LDP/abs/html/io-redirection.html"####.into()),
        author: Some(r####"Mendel Cooper"####.into()),
        author_url: None,
        shells: vec![Shell::Zsh, Shell::Bash].into_iter().collect(),
    }
}
