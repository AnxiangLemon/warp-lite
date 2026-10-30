use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Optimize Laravel Framework"####.into(),
        command: r####"php artisan optimize"####.into(),
        tags: vec![r####"php"####.into(), r####"laravel"####.into()]
            .into_iter()
            .collect(),
        description: Some(r####"Cache the framework bootstrap files"####.into()),
        arguments: vec![].into_iter().collect(),
        source_url: Some(r####"https://artisan.page/#optimize"####.into()),
        author: Some(r####"Charles Adu Boakye"####.into()),
        author_url: Some(r####"https://github.com/4cyberlord"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
