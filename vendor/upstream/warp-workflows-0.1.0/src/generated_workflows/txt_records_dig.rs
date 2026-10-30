use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Get the text records for a domain"####.into(),command: r####"dig +short {{domain}} txt"####.into(),tags: vec![r####"dig"####.into()].into_iter().collect(),description: Some(r####"Use dig to get the txt records for a domain. Useful for a quick lookup of SPF records."####.into()),arguments: vec![Argument {name: r####"domain"####.into(),description: Some(r####"The domain you want to lookup."####.into()),default_value: None,}].into_iter().collect(),source_url: None,author: Some(r####"Kia Matthews"####.into()),author_url: Some(r####"https://www.github.com/kiamatthews"####.into()),shells: vec![].into_iter().collect(),}
}
