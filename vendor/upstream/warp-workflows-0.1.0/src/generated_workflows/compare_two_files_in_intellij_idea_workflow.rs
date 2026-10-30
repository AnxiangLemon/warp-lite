use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Compare two file in IntelliJ Idea"####.into(),
        command: r####"idea diff {{file1}} {{file2}}"####.into(),
        tags: vec![r####"idea"####.into()].into_iter().collect(),
        description: Some(r####"Compare two files using IntelliJ Idea"####.into()),
        arguments: vec![
            Argument {
                name: r####"file1"####.into(),
                description: Some(r####"The first file that you want to compare."####.into()),
                default_value: None,
            },
            Argument {
                name: r####"file2"####.into(),
                description: Some(r####"The second file that you want to compare."####.into()),
                default_value: None,
            },
        ]
        .into_iter()
        .collect(),
        source_url: Some(
            r####"https://www.jetbrains.com/help/idea/command-line-differences-viewer.html"####
                .into(),
        ),
        author: Some(r####"Patrick van Zadel"####.into()),
        author_url: Some(r####"https://github.com/Shuyinsama"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
