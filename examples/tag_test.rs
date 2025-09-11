use rust_ethernet_ip::{EipClient, PlcValue};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("🔍 EtherNet/IP Tag Test");
    println!("======================");

    // Connect to the PLC
    let mut client = EipClient::connect("192.168.0.1:44818").await?;
    println!("✅ Connected to PLC");

    // Test reading tags
    println!("\n📖 Reading tags...");
    println!("-----------------");

    // Read TestInt
    match client.read_tag("TestInt").await {
        Ok(value) => println!("TestInt: {:?}", value),
        Err(e) => println!("❌ Failed to read TestInt: {}", e),
    }

    // Read TestReal
    match client.read_tag("TestReal").await {
        Ok(value) => println!("TestReal: {:?}", value),
        Err(e) => println!("❌ Failed to read TestReal: {}", e),
    }

    // Read TestBool
    match client.read_tag("TestBool").await {
        Ok(value) => println!("TestBool: {:?}", value),
        Err(e) => println!("❌ Failed to read TestBool: {}", e),
    }

    // Read TestString
    match client.read_tag("TestString").await {
        Ok(value) => println!("TestString: {:?}", value),
        Err(e) => println!("❌ Failed to read TestString: {}", e),
    }

    // Test writing tags
    println!("\n✏️ Writing tags...");
    println!("-----------------");

    // Write TestInt
    match client.write_tag("TestInt", PlcValue::Dint(42)).await {
        Ok(_) => println!("✅ Wrote TestInt: 42"),
        Err(e) => println!("❌ Failed to write TestInt: {}", e),
    }

    // Write TestReal
    match client.write_tag("TestReal", PlcValue::Real(3.14159)).await {
        Ok(_) => println!("✅ Wrote TestReal: 3.14159"),
        Err(e) => println!("❌ Failed to write TestReal: {}", e),
    }

    // Write TestBool
    match client.write_tag("TestBool", PlcValue::Bool(true)).await {
        Ok(_) => println!("✅ Wrote TestBool: true"),
        Err(e) => println!("❌ Failed to write TestBool: {}", e),
    }

    // Write TestString
    match client
        .write_tag(
            "TestString",
            PlcValue::String("Hello from Rust!".to_string()),
        )
        .await
    {
        Ok(_) => println!("✅ Wrote TestString: Hello from Rust!"),
        Err(e) => println!("❌ Failed to write TestString: {}", e),
    }

    // Read back the values to verify
    println!("\n📖 Reading tags after writing...");
    println!("----------------------------");

    // Read TestInt
    match client.read_tag("TestInt").await {
        Ok(value) => println!("TestInt: {:?}", value),
        Err(e) => println!("❌ Failed to read TestInt: {}", e),
    }

    // Read TestReal
    match client.read_tag("TestReal").await {
        Ok(value) => println!("TestReal: {:?}", value),
        Err(e) => println!("❌ Failed to read TestReal: {}", e),
    }

    // Read TestBool
    match client.read_tag("TestBool").await {
        Ok(value) => println!("TestBool: {:?}", value),
        Err(e) => println!("❌ Failed to read TestBool: {}", e),
    }

    // Read TestString
    match client.read_tag("TestString").await {
        Ok(value) => println!("TestString: {:?}", value),
        Err(e) => println!("❌ Failed to read TestString: {}", e),
    }

    Ok(())
}
