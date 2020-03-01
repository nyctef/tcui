use serde::Deserialize;

#[derive(Debug, Deserialize)]
enum BuildStatus {
    SUCCESS,
    FAILURE,
    ERROR,
}

#[derive(Debug, Deserialize)]
struct BuildJson {
    id: i32,
    status: BuildStatus,
    composite: bool,
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
            .set(
                "Authorization",
                &format!("Bearer {tc_token}", tc_token = tc_token),
            )
            .set("Accept", "application/json")
            .call()
            .into_json()
            .unwrap();
        let latest_build = serde_json::from_value::<BuildJson>(response);
        println!("{:#?}", latest_build);
    }
}
