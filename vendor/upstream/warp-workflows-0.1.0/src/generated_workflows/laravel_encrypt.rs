use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Encrypt Laravel environment file"####.into(),
        command: r####"php artisan env:encrypt"####.into(),
        tags: vec![r####"laravel"####.into(), r####"php"####.into()]
            .into_iter()
            .collect(),
        description: Some(r####"command to encrypt Laravel environment file"####.into()),
        arguments: vec![].into_iter().collect(),
        source_url: Some(
            r####"https://laravel.com/docs/9.x/configuration#encrypting-environment-files"####
                .into(),
        ),
        author: Some(r####"Charles Adu Boakye"####.into()),
        author_url: Some(r####"https://github.com/4cyberlord"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
