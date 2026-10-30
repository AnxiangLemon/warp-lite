use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Create a new Laravel App using Composer"####.into(),
        command: r####"composer create-project laravel/laravel example-app"####.into(),
        tags: vec![
            r####"composer"####.into(),
            r####"laravel"####.into(),
            r####"php"####.into(),
        ]
        .into_iter()
        .collect(),
        description: None,
        arguments: vec![].into_iter().collect(),
        source_url: Some(
            r####"https://laravel.com/docs/9.x/installation#installation-via-composer"####.into(),
        ),
        author: Some(r####"Rob Mellett"####.into()),
        author_url: Some(r####"https://robmellett.com"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
