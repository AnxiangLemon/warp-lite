use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Generate Laravel cache table"####.into(),
        command: r####"php artisan cache:table"####.into(),
        tags: vec![r####"laravel"####.into(), r####"php"####.into()]
            .into_iter()
            .collect(),
        description: Some(r####"Create a migration for the cache database table"####.into()),
        arguments: vec![].into_iter().collect(),
        source_url: Some(r####"https://artisan.page/#cachetable"####.into()),
        author: Some(r####"Charles Adu Boakye"####.into()),
        author_url: Some(r####"https://github.com/4cyberlord"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
