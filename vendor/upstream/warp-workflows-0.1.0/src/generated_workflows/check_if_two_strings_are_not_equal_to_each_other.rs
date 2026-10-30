use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Check if two strings are not equal to each other"####.into(),
        command: r####"[[ {{string_1}} != {{string_2}} ]]"####.into(),
        tags: vec![r####"shell"####.into()].into_iter().collect(),
        description: Some(
            r####"Tests if two strings are not equal and returns an exit code of 0 if so."####
                .into(),
        ),
        arguments: vec![
            Argument {
                name: r####"string_1"####.into(),
                description: None,
                default_value: None,
            },
            Argument {
                name: r####"string_2"####.into(),
                description: None,
                default_value: None,
            },
        ]
        .into_iter()
        .collect(),
        source_url: Some(r####"https://linuxhint.com/bash-test-command/"####.into()),
        author: Some(r####"Sidratul Muntaha"####.into()),
        author_url: Some(r####"https://linuxhint.com/author/lh_sidratul_muntaha/"####.into()),
        shells: vec![Shell::Zsh, Shell::Bash].into_iter().collect(),
    }
}
