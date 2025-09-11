use rust_ethernet_ip::EipClient;

#[tokio::main]
async fn main() {
    env_logger::init();

    println!("Testing improved tag discovery...");

    // Create client and connect
    let addr = "192.168.1.100:44818";
    let mut client = match EipClient::connect(addr).await {
        Ok(c) => {
            println!("Connected to PLC at {}", addr);
            c
        }
        Err(e) => {
            eprintln!("Failed to connect: {e}");
            return;
        }
    };

    // Discover tags with improved parsing
    println!("Discovering tags...");
    match client.discover_tags().await {
        Ok(_) => println!("Tag discovery completed successfully!"),
        Err(e) => {
            eprintln!("Tag discovery failed: {e}");
            return;
        }
    }

    // Test reading some common tags
    println!("\nTesting tag reads...");
    let test_tags = [
        "TestDINT",
        "TestREAL",
        "TestSTRING",
        "TestBOOL",
        "Program:MainProgram.TestTag",
    ];

    for tag in &test_tags {
        match client.read_tag(tag).await {
            Ok(value) => println!("✓ {} = {:?}", tag, value),
            Err(e) => println!("✗ {} - Error: {}", tag, e),
        }
    }

    println!("Test completed successfully!");
}
