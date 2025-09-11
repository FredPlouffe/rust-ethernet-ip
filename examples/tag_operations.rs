use rust_ethernet_ip::{EipClient, PlcValue};

#[tokio::main]
async fn main() {
    // Create client and connect
    let addr = "192.168.1.120:44818";
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

    // Tags to test
    let tags = ["TestDINT", "TestREAL", "TestSTRING", "TestBOOL"];

    // Read tags
    println!("\nReading tags...");
    for tag in &tags {
        match client.read_tag(tag).await {
            Ok(val) => println!("Read {tag}: {val:?}"),
            Err(e) => eprintln!("Failed to read {tag}: {e}"),
        }
    }

    // Write DINT
    match client.write_tag("TestDINT", PlcValue::Dint(42)).await {
        Ok(_) => println!("Wrote DINT value"),
        Err(e) => eprintln!("Failed to write DINT: {e}"),
    }
    // Write REAL
    match client.write_tag("TestREAL", PlcValue::Real(3.14159)).await {
        Ok(_) => println!("Wrote REAL value"),
        Err(e) => eprintln!("Failed to write REAL: {e}"),
    }
    // Write STRING
    match client
        .write_tag("TestSTRING", PlcValue::String("Hello PLC!".to_string()))
        .await
    {
        Ok(_) => println!("Wrote STRING value"),
        Err(e) => eprintln!("Failed to write STRING: {e}"),
    }
    // Write BOOL
    match client.write_tag("TestBOOL", PlcValue::Bool(true)).await {
        Ok(_) => println!("Wrote BOOL value"),
        Err(e) => eprintln!("Failed to write BOOL: {e}"),
    }

    // Verify written values
    println!("\nVerifying written values...");
    for tag in &tags {
        match client.read_tag(tag).await {
            Ok(val) => println!("Current value of {tag}: {val:?}"),
            Err(e) => eprintln!("Failed to verify {tag}: {e}"),
        }
    }

    // Error handling: invalid tag
    println!("\nTesting error handling with invalid tag...");
    match client.read_tag("NonExistentTag").await {
        Ok(_) => println!("Unexpected success reading invalid tag"),
        Err(e) => println!("Expected error reading invalid tag: {e}"),
    }

    // Error handling: invalid data type
    println!("\nTesting error handling with invalid data type...");
    match client
        .write_tag("TestDINT", PlcValue::String("Invalid".to_string()))
        .await
    {
        Ok(_) => println!("Unexpected success writing invalid data type"),
        Err(e) => println!("Expected error writing invalid data type: {e}"),
    }

    // Disconnect (optional, handled by Drop)
    println!("\nDemo complete.");
}
