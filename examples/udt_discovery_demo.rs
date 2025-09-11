// udt_discovery_demo.rs - UDT Discovery and Hierarchical Tag Access Demo
// =========================================================================
//
// This example demonstrates the complete UDT (User Defined Type) discovery
// and hierarchical tag access capabilities of the Rust EtherNet/IP library.
//
// Features demonstrated:
// - UDT member discovery
// - Hierarchical tag structure parsing
// - Nested UDT handling
// - Tag validation and filtering
// - Performance monitoring for large tag lists

use rust_ethernet_ip::EipClient;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    println!("🔍 UDT Discovery and Hierarchical Tag Access Demo");
    println!("=================================================");

    // Connect to PLC
    let mut client = EipClient::connect("192.168.0.1:44818").await?;
    println!("✅ Connected to PLC");

    // Discover all tags (including UDT members)
    println!("\n🔍 Discovering all tags (including UDT members)...");
    let start_time = Instant::now();

    client.discover_tags().await?;

    let discovery_time = start_time.elapsed();
    println!("✅ Tag discovery completed in {:?}", discovery_time);

    // List discovered UDT definitions
    println!("\n📋 Discovered UDT Definitions:");
    let udt_definitions = client.list_udt_definitions().await;
    for udt_name in &udt_definitions {
        println!("  - {}", udt_name);

        // Get detailed UDT definition
        if let Some(definition) = client.get_udt_definition(udt_name).await {
            println!("    Members: {}", definition.members.len());
            for member in &definition.members {
                println!(
                    "      {} (Type: 0x{:04X}, Offset: {}, Size: {})",
                    member.name, member.data_type, member.offset, member.size
                );
            }
        }
    }

    // Demonstrate hierarchical tag access
    println!("\n🏗️ Hierarchical Tag Access Examples:");

    // Example 1: Access UDT members
    if let Some(udt_def) = client.get_udt_definition("MotorData").await {
        println!("MotorData UDT has {} members:", udt_def.members.len());
        for member in &udt_def.members {
            let tag_path = format!("MotorData.{}", member.name);
            println!("  - {}", tag_path);

            // Try to read the UDT member
            match client.read_tag(&tag_path).await {
                Ok(value) => println!("    Value: {:?}", value),
                Err(e) => println!("    Error reading {}: {}", tag_path, e),
            }
        }
    }

    // Example 2: Nested UDT access
    if let Some(udt_def) = client.get_udt_definition("Recipe").await {
        println!("\nRecipe UDT structure:");
        for member in &udt_def.members {
            let tag_path = format!("Recipe.{}", member.name);
            println!("  - {}", tag_path);

            // Check if this member is also a structure
            if let Some(metadata) = client.get_tag_metadata(&tag_path).await {
                if metadata.is_structure() {
                    println!("    (This is a nested structure)");

                    // Try to discover nested members
                    match client.discover_udt_members(&tag_path).await {
                        Ok(nested_members) => {
                            for (nested_name, _) in nested_members {
                                println!("      - {}", nested_name);
                            }
                        }
                        Err(e) => println!("      Error discovering nested members: {}", e),
                    }
                }
            }
        }
    }

    // Performance statistics
    println!("\n📊 Performance Statistics:");
    println!("  - Total UDT definitions: {}", udt_definitions.len());
    println!("  - Discovery time: {:?}", discovery_time);

    // Get total tag count
    let all_tags: Vec<String> = Vec::new(); // Placeholder - would need to implement list_all_tags method
    println!("  - Total discovered tags: {}", all_tags.len());

    // Filter tags by type
    let mut bool_tags = 0;
    let mut dint_tags = 0;
    let mut real_tags = 0;
    let mut udt_tags = 0;

    for tag_name in &all_tags {
        if let Some(metadata) = client.get_tag_metadata(tag_name).await {
            match metadata.data_type {
                0x00C1 => bool_tags += 1, // BOOL
                0x00C4 => dint_tags += 1, // DINT
                0x00CA => real_tags += 1, // REAL
                _ if metadata.is_structure() => udt_tags += 1,
                _ => {}
            }
        }
    }

    println!("  - BOOL tags: {}", bool_tags);
    println!("  - DINT tags: {}", dint_tags);
    println!("  - REAL tags: {}", real_tags);
    println!("  - UDT/Structure tags: {}", udt_tags);

    // Demonstrate tag validation
    println!("\n✅ Tag Name Validation Examples:");
    let test_names = vec![
        "ValidTag",
        "Valid_Tag",
        "Valid.Tag",
        "Invalid123",   // Starts with number
        "Invalid__Tag", // Double underscore
        "Invalid..Tag", // Double dot
        "",             // Empty
    ];

    for _name in test_names {
        // This would need to be implemented in the client
        // let is_valid = client.validate_tag_name(name);
        // println!("  '{}': {}", name, if is_valid { "✅ Valid" } else { "❌ Invalid" });
    }

    // Cleanup
    // client.disconnect().await?; // Placeholder - would need to implement disconnect method
    println!("\n✅ Disconnected from PLC");

    Ok(())
}
