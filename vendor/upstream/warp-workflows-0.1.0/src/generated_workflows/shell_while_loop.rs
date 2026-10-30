use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Shell while-loop"####.into(),
        command: r####"while {{condition}} do
     {{command}}
done"####
            .into(),
        tags: vec![r####"shell"####.into()].into_iter().collect(),
        description: Some(
            r####"A while loop, similar to one in other programming languages."####.into(),
        ),
        arguments: vec![
            Argument {
                name: r####"condition"####.into(),
                description: Some(r####"The condition for the while loop."####.into()),
                default_value: None,
            },
            Argument {
                name: r####"command"####.into(),
                description: Some(r####"The command to execute within the while loop."####.into()),
                default_value: None,
            },
        ]
        .into_iter()
        .collect(),
        source_url: Some(r####"https://tldp.org/HOWTO/Bash-Prog-Intro-HOWTO-7.html"####.into()),
        author: Some(r####"Mike G"####.into()),
        author_url: None,
        shells: vec![Shell::Zsh, Shell::Bash].into_iter().collect(),
    }
}
