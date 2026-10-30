use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Install Laravel Nova using Composer"####.into(),command: r####"composer config repositories.nova '{"type": "composer", "url": "https://nova.laravel.com"}' --file composer.json && composer require laravel/nova && composer update --prefer-dist"####.into(),tags: vec![r####"composer"####.into(),r####"laravel"####.into(),r####"php"####.into()].into_iter().collect(),description: Some(r####"Install Laravel Nova using Composer"####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://nova.laravel.com/docs/4.0/installation.html"####.into()),author: Some(r####"Rob Mellett"####.into()),author_url: Some(r####"https://robmellett.com"####.into()),shells: vec![].into_iter().collect(),}
}
