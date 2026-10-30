use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Install Laravel Sail using Composer"####.into(),
        command: r####"composer require laravel/sail --dev"####.into(),
        tags: vec![
            r####"composer"####.into(),
            r####"laravel"####.into(),
            r####"php"####.into(),
        ]
        .into_iter()
        .collect(),
        description: Some(r####"Install Laravel Sail using Composer"####.into()),
        arguments: vec![].into_iter().collect(),
        source_url: Some(
            r####"https://laravel.com/docs/9.x/sail#installing-sail-into-existing-applications"####
                .into(),
        ),
        author: Some(r####"Rob Mellett"####.into()),
        author_url: Some(r####"https://robmellett.com"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
