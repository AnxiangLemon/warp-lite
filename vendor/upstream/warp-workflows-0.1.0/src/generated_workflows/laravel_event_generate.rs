use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Generate missing Laravel events and listeners"####.into(),
        command: r####"php artisan event:generate"####.into(),
        tags: vec![r####"laravel"####.into(), r####"php"####.into()]
            .into_iter()
            .collect(),
        description: Some(
            r####"Generate the missing events and listeners based on registration"####.into(),
        ),
        arguments: vec![].into_iter().collect(),
        source_url: Some(r####"https://artisan.page/#eventgenerate"####.into()),
        author: Some(r####"Charles Adu Boakye"####.into()),
        author_url: Some(r####"https://github.com/4cyberlord"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
