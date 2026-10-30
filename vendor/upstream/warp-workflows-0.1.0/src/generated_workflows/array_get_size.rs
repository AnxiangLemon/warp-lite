use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Get number of elements in an array"####.into(),
        command: r####"echo ${#{{array_name}}[@]}"####.into(),
        tags: vec![r####"shell"####.into()].into_iter().collect(),
        description: Some(
            r####"Gets the number of elements in an array, and prints it to stdout."####.into(),
        ),
        arguments: vec![Argument {
            name: r####"array_name"####.into(),
            description: Some(r####"The name of the array to get the size of"####.into()),
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
