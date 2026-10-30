use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Open a file or directory in a IntelliJ Idea editor"####.into(),
        command: r####"idea {{file_or_directory}}"####.into(),
        tags: vec![r####"idea"####.into()].into_iter().collect(),
        description: Some(r####"Use the command line to open a file in IntelliJ Idea."####.into()),
        arguments: vec![Argument {
            name: r####"file_or_directory"####.into(),
            description: Some(r####"The file or directory that you want to open."####.into()),
            default_value: None,
        }]
        .into_iter()
        .collect(),
        source_url: Some(
            r####"https://www.jetbrains.com/help/idea/opening-files-from-command-line.html"####
                .into(),
        ),
        author: Some(r####"Patrick van Zadel"####.into()),
        author_url: Some(r####"https://github.com/Shuyinsama"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
