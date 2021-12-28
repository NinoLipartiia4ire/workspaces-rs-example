#![cfg(test)]

use workspaces::DevNetwork;

#[tokio::test]
async fn first_test() -> anyhow::Result<()> {
    let worker = workspaces::sandbox();
    println!("Running the first test");
    Ok(())
}

#[tokio::test]
async fn second_test() -> anyhow::Result<()> {
    let worker = workspaces::sandbox();
    println!("Running the second test");
    Ok(())
}
