use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Update all pip3 packages"####.into(),command: r####"pip3 list --outdated --format=columns | tail -n +3 | awk '{print $1}' | xargs -n1 pip3 install -U"####.into(),tags: vec![r####"python"####.into()].into_iter().collect(),description: Some(r####"Update all your pip3 packages in one go."####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://github.com/warpdotdev/workflows/issues/149"####.into()),author: Some(r####"Kirill Kulikov, csaper"####.into()),author_url: Some(r####"https://github.com/kikulikov, https://github.com/csaper"####.into()),shells: vec![].into_iter().collect(),}
}
