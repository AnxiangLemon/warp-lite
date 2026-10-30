use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Discover and cache all Laravel application events and listeners"####.into(),
        command: r####"php artisan event:cache"####.into(),
        tags: vec![r####"laravel"####.into(), r####"php"####.into()]
            .into_iter()
            .collect(),
        description: Some(
            r####"Discover and cache the application's events and listeners"####.into(),
        ),
        arguments: vec![].into_iter().collect(),
        source_url: Some(r####"https://artisan.page/#eventcache"####.into()),
        author: Some(r####"Charles Adu Boakye"####.into()),
        author_url: Some(r####"https://github.com/4cyberlord"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
