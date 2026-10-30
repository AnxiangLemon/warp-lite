use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Generate Laravel application key"####.into(),
        command: r####"php artisan key:generate"####.into(),
        tags: vec![r####"laravel"####.into(), r####"php"####.into()]
            .into_iter()
            .collect(),
        description: Some(r####"Set laravel application key"####.into()),
        arguments: vec![].into_iter().collect(),
        source_url: Some(r####"https://artisan.page/#keygenerate"####.into()),
        author: Some(r####"Charles Adu Boakye"####.into()),
        author_url: Some(r####"https://github.com/4cyberlord"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
