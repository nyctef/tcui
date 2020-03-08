use failure::Fallible;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum BuildStatus {
    SUCCESS,
    FAILURE,
    ERROR,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BuildState {
    Queued,
    Running,
    Finished,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Build {
    number: String,
    status: BuildStatus,
    state: BuildState,
    web_url: String,
    #[serde(rename = "snapshot-dependencies")]
    snapshot_dependencies: SnapshotDependencies,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildType {
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct SnapshotDependencies {
    build: Vec<BuildDependency>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildDependency {
    number: String,
    status: BuildStatus,
    state: BuildState,
    percentage_complete: Option<i32>,
    web_url: String,
}

pub fn download_build(
    api_token: &str,
    api_root: &str,
    build_type: &str,
    branch: &str,
) -> Fallible<Build> {
    let url = format!("{api_root}/app/rest/builds/buildType:{build_type},defaultFilter:false,branch:name:{branch_name}", api_root = api_root, build_type = build_type, branch_name = branch);
    // println!("Requesting url {}", url);
    let response = ureq::get(&url)
            .query("fields", "number,status,state,percentageComplete,webUrl,buildType(name),snapshot-dependencies(build(webUrl,number,status,state,percentageComplete,buildType(name)))")
            .set(
                "Authorization",
                &format!("Bearer {tc_token}", tc_token = api_token),
            )
            .set("Accept", "application/json")
            .call();
    // println!("{}", response.status_line());
    // println!("{:?}", response.into_string());
    let json = response.into_json()?;
    // println!("{:#?}", json);
    Ok(serde_json::from_value::<Build>(json)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn can_poke_tc_api() {
        let tc_token = env::var("TCUI_TC_TOKEN").expect("TCUI_TC_TOKEN is required");
        let latest_build = download_build(
            &tc_token,
            "https://buildserver.red-gate.com",
            "RedgateChangeControl_OverallBuild",
            "add-beta-tag",
        );
        println!("{:#?}", latest_build);
    }
}
