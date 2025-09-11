use rust_ethernet_ip::EipClient;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🔧 Testing Tag Discovery Fix");
    println!("=============================");

    // Replace with your PLC IP address
    let plc_ip = "10.50.126.145:44818";

    println!("🔌 Connecting to PLC at {}...", plc_ip);
    let mut client = EipClient::connect(plc_ip).await?;
    println!("✅ Connected successfully!");

    // Test health check
    println!("\n🏥 Testing health check...");
    let health = client.check_health().await;
    println!("Health status: {:?}", health);

    // Test tag discovery
    println!("\n🔍 Testing tag discovery...");
    match client.discover_tags().await {
        Ok(()) => {
            println!("✅ Tag discovery completed successfully!");

            // Try to get some tag metadata
            if let Some(metadata) = client.get_tag_metadata("Controller").await {
                println!("📊 Found Controller tag metadata: {:?}", metadata);
            }
        }
        Err(e) => {
            println!("❌ Tag discovery failed: {}", e);
        }
    }

    // Test reading a specific tag
    println!("\n📖 Testing tag reading...");
    let test_tags = [
        "Controller.Type",
        "Controller.MajorRev",
        "Controller.MinorRev",
        "Program:LS18_Rewind.CoreDiamMin", // This was the failing tag from the user
    ];

    for tag_name in &test_tags {
        println!("  Reading tag: {}", tag_name);
        match client.read_tag(tag_name).await {
            Ok(value) => {
                println!("    ✅ {} = {:?}", tag_name, value);
            }
            Err(e) => {
                println!("    ❌ {} failed: {}", tag_name, e);
            }
        }
    }

    println!("\n🏁 Test completed!");
    Ok(())
}
