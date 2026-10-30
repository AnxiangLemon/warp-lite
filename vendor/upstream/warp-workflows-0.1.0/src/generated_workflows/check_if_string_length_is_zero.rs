use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Check if string length is zero"####.into(),
        command: r####"[[ -n {{string}} ]]"####.into(),
        tags: vec![r####"shell"####.into()].into_iter().collect(),
        description: Some(
            r####"Returns an exit code of 0 if the length of the string is zero."####.into(),
        ),
        arguments: vec![Argument {
            name: r####"string"####.into(),
            description: Some(r####"The string to test."####.into()),
            default_value: None,
        }]
        .into_iter()
        .collect(),
        source_url: Some(r####"https://linuxhint.com/bash-test-command/"####.into()),
        author: Some(r####"Sidratul Muntaha"####.into()),
        author_url: Some(r####"https://linuxhint.com/author/lh_sidratul_muntaha/"####.into()),
        shells: vec![Shell::Zsh, Shell::Bash].into_iter().collect(),
    }
}
