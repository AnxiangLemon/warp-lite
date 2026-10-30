use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Make a new, empty array"####.into(),
        command: r####"{{array_name}}=()"####.into(),
        tags: vec![r####"shell"####.into()].into_iter().collect(),
        description: Some(
            r####"Makes an empty array and assigns it to variable "array_name""####.into(),
        ),
        arguments: vec![Argument {
            name: r####"array_name"####.into(),
            description: Some(r####"The name of the array to create"####.into()),
            default_value: None,
        }]
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
