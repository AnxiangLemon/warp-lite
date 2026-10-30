use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Delete empty lines in a file"####.into(),
        command: r####"sed '/^[[:space:]]*$/d' {{file_name}}"####.into(),
        tags: vec![r####"file manipulation"####.into(), r####"sed"####.into()]
            .into_iter()
            .collect(),
        description: Some(
            r####"Deletes all lines that contain only whitespace from a file."####.into(),
        ),
        arguments: vec![Argument {
            name: r####"file_name"####.into(),
            description: None,
            default_value: None,
        }]
        .into_iter()
        .collect(),
        source_url: Some(
            r####"https://stackoverflow.com/questions/16414410/delete-empty-lines-using-sed"####
                .into(),
        ),
        author: Some(r####"Kent"####.into()),
        author_url: Some(r####"https://stackoverflow.com/users/164835/kent"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
