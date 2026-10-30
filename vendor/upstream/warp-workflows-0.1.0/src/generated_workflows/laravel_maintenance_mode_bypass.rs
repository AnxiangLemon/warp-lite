use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Bypass Laravel maintenance mode"####.into(),command: r####"php artisan down --secret="{{bypass_secret_key}}""####.into(),tags: vec![r####"laravel"####.into(),r####"php"####.into()].into_iter().collect(),description: Some(r####"This command helps you bypass laravel maintenance mode by setting the secret key."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://laravel.com/docs/9.x/configuration#pre-rendering-the-maintenance-mode-view"####.into()),author: Some(r####"Charles Adu Boakye"####.into()),author_url: Some(r####"https://github.com/4cyberlord"####.into()),shells: vec![].into_iter().collect(),}
}
