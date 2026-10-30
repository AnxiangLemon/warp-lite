use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Install Laravel Vapor using Composer"####.into(),
        command: r####"composer require laravel/vapor-cli --update-with-dependencies"####.into(),
        tags: vec![
            r####"composer"####.into(),
            r####"laravel"####.into(),
            r####"php"####.into(),
        ]
        .into_iter()
        .collect(),
        description: Some(r####"Install Laravel Vapor using Composer"####.into()),
        arguments: vec![].into_iter().collect(),
        source_url: Some(
            r####"https://docs.vapor.build/1.0/introduction.html#installing-the-vapor-cli"####
                .into(),
        ),
        author: Some(r####"Rob Mellett"####.into()),
        author_url: Some(r####"https://robmellett.com"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
