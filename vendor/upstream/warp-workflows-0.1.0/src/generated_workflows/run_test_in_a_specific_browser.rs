use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Run Playwright test in a specific browser"####.into(),
        command: r####"npx playwright test --browser={{browser}}"####.into(),
        tags: vec![r####"playwright"####.into()].into_iter().collect(),
        description: Some(r####"Runs Playwright tests in a specific browser."####.into()),
        arguments: vec![Argument {
            name: r####"browser"####.into(),
            description: Some(
                r####"Browser to run tests in(either chromium, firefox, webkit or all)."####.into(),
            ),
            default_value: Some(r####"all"####.into()),
        }]
        .into_iter()
        .collect(),
        source_url: Some(r####"https://playwright.dev/docs/test-cli#reference"####.into()),
        author: Some(r####"DominusKelvin"####.into()),
        author_url: None,
        shells: vec![].into_iter().collect(),
    }
}
