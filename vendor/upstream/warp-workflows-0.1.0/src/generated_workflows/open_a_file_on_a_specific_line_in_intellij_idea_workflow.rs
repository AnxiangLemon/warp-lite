use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Open a file on a specific line in IntelliJ Idea"####.into(),
        command: r####"idea --line {{line_number}} {{file}}"####.into(),
        tags: vec![r####"idea"####.into()].into_iter().collect(),
        description: Some(
            r####"Use the command line to open a file on a specific line in IntelliJ Idea."####
                .into(),
        ),
        arguments: vec![
            Argument {
                name: r####"line_number"####.into(),
                description: Some(r####"The line number you want the file to open two"####.into()),
                default_value: Some(r####"1"####.into()),
            },
            Argument {
                name: r####"file"####.into(),
                description: Some(r####"The file that you want to open."####.into()),
                default_value: None,
            },
        ]
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
