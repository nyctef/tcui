use serde::Deserialize;

#[derive(Debug, Deserialize)]
enum BuildStatus {
    SUCCESS,
    FAILURE,
    ERROR,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum BuildState {
    Queued,
    Running,
    Finished,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Build {
    number: String,
    status: BuildStatus,
    state: BuildState,
    web_url: String,
    #[serde(rename = "snapshot-dependencies")]
    snapshot_dependencies: SnapshotDependencies,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BuildType {
    name: String,
}

#[derive(Debug, Deserialize)]
struct SnapshotDependencies {
    build: Vec<BuildDependency>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BuildDependency {
    number: String,
    status: BuildStatus,
    state: BuildState,
    percentage_complete: Option<i32>,
    web_url: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn can_poke_tc_api() {
        let tc_token = env::var("TCUI_TC_TOKEN").expect("TCUI_TC_TOKEN is required");
        let url = format!("{tcRoot}/app/rest/builds/buildType:{buildType},defaultFilter:false,branch:name:{branchName}", tcRoot="https://buildserver.red-gate.com", buildType="RedgateChangeControl_OverallBuild", branchName="add-beta-tag");
        // println!("Requesting url {}", url);
        let response = ureq::get(&url)
            .query("fields", "number,status,state,percentageComplete,webUrl,buildType(name),snapshot-dependencies(build(webUrl,number,status,state,percentageComplete,buildType(name)))")
            .set(
                "Authorization",
                &format!("Bearer {tc_token}", tc_token = tc_token),
            )
            .set("Accept", "application/json")
            .call();
        // println!("{}", response.status_line());
        // println!("{:?}", response.into_string());
        let json = response.into_json().unwrap();
        // println!("{:#?}", json);
        let latest_build = serde_json::from_value::<Build>(json).unwrap();
        println!("{:#?}", latest_build);
    }
}
