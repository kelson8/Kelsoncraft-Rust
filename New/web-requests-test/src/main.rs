use wreq::Client;
use wreq_util::Emulation;

// https://github.com/0x676e67/wreq

#[tokio::main]
async fn main() -> wreq::Result<()> {
    // Build a client
    let client = Client::builder()
        // .emulation(Emulation::)
        .emulation(Emulation::Safari26)
        .build()?;

    // Use the API you're already familiar with
    // let resp = client.get("https://pingly.us.kg/api/all").send().await?;
    let git_url = "https://git.kelsoncraft.net/kelson8/KelsonCraft-Website/raw/branch/master/src/json/videos.json";
    let resp = client.get(git_url).send().await?;
    println!("{}", resp.text().await?);
    Ok(())
}