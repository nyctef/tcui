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
struct BuildJson {
    number: String,
    status: BuildStatus,
    state: BuildState,
    web_url: String,
    #[serde(rename = "snapshot-dependencies")]
    snapshot_dependencies: SnapshotDependencies
}

#[derive(Debug, Deserialize)]
struct SnapshotDependencies {
    build: Vec<BuildDependency>
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
        let response = ureq::get(&url)
            .query("fields", "number,status,state,percentageComplete,webUrl,snapshot-dependencies(build(number,status,state,percentageComplete,webUrl))")
            .set(
                "Authorization",
                &format!("Bearer {tc_token}", tc_token = tc_token),
            )
            .set("Accept", "application/json")
            .call()
            .into_json()
            .unwrap();
        let latest_build = serde_json::from_value::<BuildJson>(response).unwrap();
        println!("{:#?}", latest_build);
    }
}
