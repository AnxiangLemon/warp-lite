use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Encrypt Laravel environment file with custom encryption key"####.into(),
        command: r####"php artisan env:encrypt --key={{custom_key}}"####.into(),
        tags: vec![r####"laravel"####.into(), r####"php"####.into()]
            .into_iter()
            .collect(),
        description: Some(
            r####"command to encrypt your Laravel environment file with custom encryption key"####
                .into(),
        ),
        arguments: vec![Argument {
            name: r####"custom_key"####.into(),
            description: Some(
                r####"Specify custom encryption key for Laravel environment file."####.into(),
            ),
            default_value: None,
        }]
        .into_iter()
        .collect(),
        source_url: Some(
            r####"https://laravel.com/docs/9.x/configuration#encrypting-environment-files"####
                .into(),
        ),
        author: Some(r####"Charles Adu Boakye"####.into()),
        author_url: Some(r####"https://github.com/4cyberlord"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
