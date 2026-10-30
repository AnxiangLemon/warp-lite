use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Laravel maintenance mode disabled"####.into(),command: r####"php artisan up"####.into(),tags: vec![r####"laravel"####.into(),r####"php"####.into()].into_iter().collect(),description: Some(r####"Disable laravel maintenance mode"####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://laravel.com/docs/9.x/configuration#pre-rendering-the-maintenance-mode-view"####.into()),author: Some(r####"Charles Adu Boakye"####.into()),author_url: Some(r####"https://github.com/4cyberlord"####.into()),shells: vec![].into_iter().collect(),}
}
