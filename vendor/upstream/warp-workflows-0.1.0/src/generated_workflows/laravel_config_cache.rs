use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Create Laravel config cache file"####.into(),
        command: r####"php artisan config:clear"####.into(),
        tags: vec![r####"laravel"####.into(), r####"php"####.into()]
            .into_iter()
            .collect(),
        description: Some(r####"Create a cache file for faster configuration loading"####.into()),
        arguments: vec![].into_iter().collect(),
        source_url: Some(r####"https://artisan.page/#configclear"####.into()),
        author: Some(r####"Charles Adu Boakye"####.into()),
        author_url: Some(r####"https://github.com/4cyberlord"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
