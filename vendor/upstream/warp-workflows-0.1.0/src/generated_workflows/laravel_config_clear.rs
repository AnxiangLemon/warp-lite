use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Clear Laravel configuration"####.into(),
        command: r####"php artisan config:clear"####.into(),
        tags: vec![r####"laravel"####.into(), r####"php"####.into()]
            .into_iter()
            .collect(),
        description: Some(r####"Remove the configuration cache file"####.into()),
        arguments: vec![].into_iter().collect(),
        source_url: Some(r####"https://artisan.page/#configclear"####.into()),
        author: Some(r####"Charles Adu Boakye"####.into()),
        author_url: Some(r####"https://github.com/4cyberlord"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
