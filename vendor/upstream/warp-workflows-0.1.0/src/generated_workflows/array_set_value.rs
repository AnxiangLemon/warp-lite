use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Set an index in an array to a value"####.into(),
        command: r####"{{array_name}}[{{index}}]={{value}}"####.into(),
        tags: vec![r####"shell"####.into()].into_iter().collect(),
        description: Some(
            r####"Makes an empty array and assigns it to variable "array_name""####.into(),
        ),
        arguments: vec![
            Argument {
                name: r####"array_name"####.into(),
                description: Some(r####"The name of the array to set the value"####.into()),
                default_value: None,
            },
            Argument {
                name: r####"array_index"####.into(),
                description: Some(r####"The index of the array to set"####.into()),
                default_value: None,
            },
            Argument {
                name: r####"value"####.into(),
                description: Some(r####"The value to set the array index to"####.into()),
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
