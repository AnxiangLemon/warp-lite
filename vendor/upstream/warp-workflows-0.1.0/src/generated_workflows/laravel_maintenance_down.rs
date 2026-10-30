use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Enable Laravel maintenance mode"####.into(),command: r####"php artisan down"####.into(),tags: vec![r####"Laravel"####.into(),r####"Php"####.into()].into_iter().collect(),description: Some(r####"This command enables maintenance mode in your laravel application"####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://laravel.com/docs/9.x/configuration#pre-rendering-the-maintenance-mode-view"####.into()),author: Some(r####"Charles Adu Boakye"####.into()),author_url: Some(r####"https://github.com/4cyberlord"####.into()),shells: vec![].into_iter().collect(),}
}
