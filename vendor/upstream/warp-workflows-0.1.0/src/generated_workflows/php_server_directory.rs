use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Start a local PHP server with a specific document root directory"####.into(),
        command: r####"php -S localhost:{{port}} -t {{directory}}"####.into(),
        tags: vec![r####"php"####.into()].into_iter().collect(),
        description: Some(
            r####"A command that starts a PHP server in the specified directory."####.into(),
        ),
        arguments: vec![
            Argument {
                name: r####"port"####.into(),
                description: Some(r####"The port number you want to run, e.g. 8080."####.into()),
                default_value: Some(r####"8080"####.into()),
            },
            Argument {
                name: r####"directory"####.into(),
                description: Some(
                    r####"The directory you want the server to run from, e.g. app/public."####
                        .into(),
                ),
                default_value: Some(r####"app"####.into()),
            },
        ]
        .into_iter()
        .collect(),
        source_url: Some(
            r####"https://www.php.net/manual/en/features.commandline.webserver.php"####.into(),
        ),
        author: Some(r####"Nate Finch"####.into()),
        author_url: Some(r####"https://github.com/n8finch"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
