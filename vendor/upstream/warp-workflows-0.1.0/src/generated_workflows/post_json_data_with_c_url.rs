use warp_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"POST JSON data with cURL"####.into(),command: r####"curl --header "Content-Type: application/json" \
  --request POST \
  --data '{{json_data}}' \
 {{url}}"####.into(),tags: vec![r####"curl"####.into()].into_iter().collect(),description: Some(r####"Sends a POST request with JSON data using curl by setting the content-type of the request to "application/json"."####.into()),arguments: vec![Argument {name: r####"json_data"####.into(),description: Some(r####"The JSON data to encode."####.into()),default_value: None,},Argument {name: r####"url"####.into(),description: Some(r####"The url where the request should be sent."####.into()),default_value: None,}].into_iter().collect(),source_url: Some(r####"https://stackoverflow.com/questions/7172784/how-do-i-post-json-data-with-curl"####.into()),author: Some(r####"Sean Patrick Floyd"####.into()),author_url: Some(r####"https://stackoverflow.com/users/342852/sean-patrick-floyd"####.into()),shells: vec![].into_iter().collect(),}
}
