use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Search for a package on a Android device"####.into(),
        command: r####"adb shell pm list packages | grep "{{query}}""####.into(),
        tags: vec![r####"android"####.into(), r####"adb"####.into()]
            .into_iter()
            .collect(),
        description: Some(r####"Use ADB to search for packages on a Android device."####.into()),
        arguments: vec![Argument {
            name: r####"query"####.into(),
            description: Some(r####"A search query for which package you want to find."####.into()),
            default_value: None,
        }]
        .into_iter()
        .collect(),
        source_url: None,
        author: Some(r####"Odin Asbjørnsen"####.into()),
        author_url: Some(r####"https://github.com/oas004"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
