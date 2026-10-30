use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Decrypt Laravel environment file with key"####.into(),
        command: r####"php artisan env:decrypt --key={{decrypt_key}}"####.into(),
        tags: vec![r####"laravel"####.into(), r####"php"####.into()]
            .into_iter()
            .collect(),
        description: Some(r####"Command to decrypt Laravel environment file"####.into()),
        arguments: vec![Argument {
            name: r####"decrypt_key"####.into(),
            description: Some(
                r####"Provide your laravel decrypt key to decrypt the environment file"####.into(),
            ),
            default_value: None,
        }]
        .into_iter()
        .collect(),
        source_url: Some(r####"https://laravel.com/docs/9.x/configuration#decryption"####.into()),
        author: Some(r####"Charles Adu Boakye"####.into()),
        author_url: Some(r####"https://github.com/4cyberlord"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
