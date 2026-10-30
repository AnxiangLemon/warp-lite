use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Clear Laravel events and listeners"####.into(),
        command: r####"php artisan event:clear"####.into(),
        tags: vec![r####"laravel"####.into(), r####"php"####.into()]
            .into_iter()
            .collect(),
        description: Some(r####"Clear all cached events and listeners"####.into()),
        arguments: vec![].into_iter().collect(),
        source_url: Some(r####"https://artisan.page/#eventclear"####.into()),
        author: Some(r####"Charles Adu Boakye"####.into()),
        author_url: Some(r####"https://github.com/4cyberlord"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
