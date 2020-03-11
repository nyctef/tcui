use failure::{bail, Fallible};
use serde::Deserialize;
use serde_json::value::Value;

#[derive(Debug, Deserialize)]
pub enum BuildStatus {
    SUCCESS,
    FAILURE,
    ERROR,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", tag = "state")]
pub enum Build {
    #[serde(rename_all = "camelCase")]
    Queued {
        build_type: BuildType,
        web_url: String,
        #[serde(rename = "snapshot-dependencies")]
        snapshot_dependencies: Option<SnapshotDependencies>,
    },
    #[serde(rename_all = "camelCase")]
    Running {
        build_type: BuildType,
        number: String,
        status: BuildStatus,
        status_text: String,
        web_url: String,
        #[serde(rename = "running-info")]
        running_info: RunningInfo,
        #[serde(rename = "snapshot-dependencies")]
        snapshot_dependencies: Option<SnapshotDependencies>,
    },
    #[serde(rename_all = "camelCase")]
    Finished {
        build_type: BuildType,
        number: String,
        status: BuildStatus,
        status_text: String,
        web_url: String,
        #[serde(rename = "snapshot-dependencies")]
        snapshot_dependencies: Option<SnapshotDependencies>,
    },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunningInfo {
    pub percentage_complete: u16,
    pub elapsed_seconds: i32,
    pub estimated_total_seconds: i32,
    pub outdated: bool,
    pub probably_hanging: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildType {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct SnapshotDependencies {
    pub build: Vec<Build>,
}

pub fn download_build(
    api_token: &str,
    api_root: &str,
    build_type: &str,
    branch: &str,
) -> Fallible<Build> {
    let url = format!("{api_root}/app/rest/builds/buildType:{build_type},defaultFilter:false,branch:name:{branch_name}", api_root = api_root, build_type = build_type, branch_name = branch);
    // println!("Requesting url {}", url);
    let build_fields = "number,status,state,statusText,webUrl,buildType(name),running-info(percentageComplete,elapsedSeconds,estimatedTotalSeconds,outdated,probablyHanging)";
    let response = ureq::get(&url)
        .query(
            "fields",
            &format!(
                "{},snapshot-dependencies(build({}))",
                build_fields, build_fields
            ),
        )
        .set(
            "Authorization",
            &format!("Bearer {tc_token}", tc_token = api_token),
        )
        .set("Accept", "application/json")
        .call();
    // println!("{}", response.status_line());
    // println!("{:?}", response.into_string());
    let json = response.into_json()?;

    if let Value::Object(obj) = &json {
        if obj.is_empty() {
            bail!("teamcity returned an empty json object (build not found?)");
        }
    }
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
            "master",
        );
        dbg!(latest_build.unwrap());
    }
}
