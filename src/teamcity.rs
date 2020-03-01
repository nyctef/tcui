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
    use failure::Fallible;
    use std::env;
    use tokio;

    #[tokio::test]
    async fn can_poke_tc_api() -> Fallible<()> {
        let tc_token = env::var("TCUI_TC_TOKEN").expect("TCUI_TC_TOKEN is required");
        let url = format!("{tcRoot}/app/rest/builds/buildType:{buildType},defaultFilter:false,branch:name:{branchName}", tcRoot="https://buildserver.red-gate.com", buildType="RedgateChangeControl_OverallBuild", branchName="add-beta-tag");
        let latest_build: BuildJson = reqwest::Client::new()
            .get(&url)
            .header(
                "Authorization",
                format!("Bearer {tc_token}", tc_token = tc_token),
            )
            .header("Accept", "application/json")
            .send()
            .await?
            .json()
            .await?;
        println!("{:#?}", latest_build);
        Ok(())
    }
}
