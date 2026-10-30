use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Installing Laravel Installer"####.into(),
        command: r####"composer global require laravel/installer"####.into(),
        tags: vec![
            r####"composer"####.into(),
            r####"laravel"####.into(),
            r####"php"####.into(),
        ]
        .into_iter()
        .collect(),
        description: Some(r####"Install Laravel Installer"####.into()),
        arguments: vec![].into_iter().collect(),
        source_url: Some(r####"https://laravel.com/docs/5.4/installation"####.into()),
        author: Some(r####"Charles Adu Boakye"####.into()),
        author_url: Some(r####"https://www.linkedin.com/in/charles-adu-boakye/"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
