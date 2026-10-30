use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Pretend to run Laravel database migrations"####.into(),command: r####"php artisan migrate --pretend"####.into(),tags: vec![r####"Laravel"####.into(),r####"Php"####.into()].into_iter().collect(),description: Some(r####"This command will return the SQL statement that would be performed when we would run the migration"####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://laravel.com/docs/10.x/migrations#running-migrations"####.into()),author: Some(r####"Bert De Swaef"####.into()),author_url: Some(r####"https://github.com/BurtDS"####.into()),shells: vec![].into_iter().collect(),}
}
