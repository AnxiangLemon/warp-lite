use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"List all Laravel application's events listeners"####.into(),
        command: r####"php artisan event:list"####.into(),
        tags: vec![r####"laravel"####.into(), r####"php"####.into()]
            .into_iter()
            .collect(),
        description: Some(r####"List the application's events and listeners"####.into()),
        arguments: vec![].into_iter().collect(),
        source_url: Some(r####"https://artisan.page/#eventlist"####.into()),
        author: Some(r####"Charles Adu Boakye"####.into()),
        author_url: Some(r####"https://github.com/4cyberlord"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
