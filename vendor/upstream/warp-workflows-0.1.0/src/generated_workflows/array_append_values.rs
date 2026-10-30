use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Append value(s) to an array"####.into(),
        command: r####"{{array_name}}+=({{values}})"####.into(),
        tags: vec![r####"shell"####.into()].into_iter().collect(),
        description: Some(
            r####"Loops through an array, running a command on each value."####.into(),
        ),
        arguments: vec![
            Argument {
                name: r####"array_name"####.into(),
                description: Some(r####"The name of the array to append values"####.into()),
                default_value: None,
            },
            Argument {
                name: r####"values"####.into(),
                description: Some(
                    r####"The values to append to the array, space-separated"####.into(),
                ),
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
