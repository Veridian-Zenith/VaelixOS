// vxp_installer.rs

// Package manager core logic module
pub mod vxp_installer {
    use reqwest::Error;
    use serde::Deserialize;
    use std::collections::HashMap;

    #[derive(Deserialize)]
    struct Package {
        name: String,
        version: String,
        description: String,
    }

    pub async fn fetch_packages() -> Result<HashMap<String, Package>, Error> {
        let response = reqwest::get("https://api.github.com/orgs/your-org/repos").await?;
        let repos: Vec<Package> = response.json().await?;
        let mut packages = HashMap::new();
        for repo in repos {
            packages.insert(repo.name, repo);
        }
        Ok(packages)
    }
}
