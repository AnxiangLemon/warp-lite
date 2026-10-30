use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Get a value from an array"####.into(),
        command: r####"echo ${{{array_name}}[{{index}}]}"####.into(),
        tags: vec![r####"shell"####.into()].into_iter().collect(),
        description: Some(r####"Gets a value from an array and prints it to stdout."####.into()),
        arguments: vec![
            Argument {
                name: r####"array_name"####.into(),
                description: Some(r####"The name of the array to get a value from"####.into()),
                default_value: None,
            },
            Argument {
                name: r####"index"####.into(),
                description: Some(r####"The index of the value to get (starting at 0)"####.into()),
                default_value: None,
            },
        ]
        .into_iter()
        .collect(),
        source_url: Some(
            r####"https://opensource.com/article/18/5/you-dont-know-bash-intro-bash-arrays"####
                .into(),
        ),
        author: Some(r####"Wyatt-Stanke"####.into()),
        author_url: Some(r####"https://github.com/Wyatt-Stanke"####.into()),
        shells: vec![Shell::Zsh, Shell::Bash].into_iter().collect(),
    }
}
