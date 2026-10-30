use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Run a single Playwright test file"####.into(),
        command: r####"npx playwright test {{pathToFile}}"####.into(),
        tags: vec![r####"playwright"####.into()].into_iter().collect(),
        description: Some(r####"Runs a single Playwright test file."####.into()),
        arguments: vec![Argument {
            name: r####"pathToFile"####.into(),
            description: Some(r####"The path to the test file to run."####.into()),
            default_value: None,
        }]
        .into_iter()
        .collect(),
        source_url: Some(r####"https://playwright.dev/docs/running-tests"####.into()),
        author: Some(r####"DominusKelvin"####.into()),
        author_url: None,
        shells: vec![].into_iter().collect(),
    }
}
