use rust_ethernet_ip::{BatchOperation, EipClient, PlcValue};
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Rust EtherNet/IP Batch Operations Benchmark");
    println!("================================================");

    // Connect to PLC
    let mut client = EipClient::new("192.168.0.1:44818").await?;
    println!("✅ Connected to PLC successfully!");

    // Define test tags
    let test_tags = vec![
        "ProductionCount",
        "Temperature_1",
        "Temperature_2",
        "Pressure_1",
        "FlowRate",
        "SetPoint_1",
        "SetPoint_2",
        "EnableFlag",
        "ProductionMode",
        "RecipeNumber",
    ];

    println!("\\n📊 Benchmark: Reading {} tags", test_tags.len());
    println!("===================================================");

    // Benchmark individual operations
    println!("\\n🐌 Individual Operations:");
    println!("-------------------------");
    let start = Instant::now();
    let mut individual_results = Vec::new();

    for tag in &test_tags {
        let tag_start = Instant::now();
        match client.read_tag(tag).await {
            Ok(value) => {
                let duration = tag_start.elapsed();
                individual_results.push((tag.to_string(), Ok(value), duration));
                println!(
                    "  📊 {}: {:?} ({:.2}ms)",
                    tag,
                    individual_results.last().unwrap().1.as_ref().unwrap(),
                    duration.as_secs_f64() * 1000.0
                );
            }
            Err(e) => {
                let duration = tag_start.elapsed();
                individual_results.push((tag.to_string(), Err(e.to_string()), duration));
                println!(
                    "  ❌ {}: Error ({:.2}ms)",
                    tag,
                    duration.as_secs_f64() * 1000.0
                );
            }
        }
    }
    let individual_total = start.elapsed();

    // Benchmark batch operations
    println!("\\n🚀 Batch Operations:");
    println!("--------------------");
    let operations: Vec<BatchOperation> = test_tags
        .iter()
        .map(|tag| BatchOperation::Read {
            tag_name: tag.to_string(),
        })
        .collect();

    let batch_start = Instant::now();
    let batch_results = client.execute_batch(&operations).await?;
    let batch_total = batch_start.elapsed();

    for batch_result in &batch_results {
        let tag_name = match &batch_result.operation {
            BatchOperation::Read { tag_name } => tag_name,
            BatchOperation::Write { tag_name, .. } => tag_name,
        };
        match &batch_result.result {
            Ok(Some(value)) => println!("  📊 {}: {:?}", tag_name, value),
            Ok(None) => println!("  📊 {}: Success (write)", tag_name),
            Err(e) => println!("  ❌ {}: {}", tag_name, e),
        }
    }

    // Performance comparison
    println!("\\n⚡ Performance Comparison:");
    println!("==========================");
    println!(
        "  Individual operations: {:.2}ms",
        individual_total.as_secs_f64() * 1000.0
    );
    println!(
        "  Batch operations:      {:.2}ms",
        batch_total.as_secs_f64() * 1000.0
    );

    let speedup = individual_total.as_secs_f64() / batch_total.as_secs_f64();
    println!("  📈 Speedup:             {:.1}x faster", speedup);

    let saved_time = individual_total.saturating_sub(batch_total);
    println!(
        "  ⏱️  Time saved:          {:.2}ms",
        saved_time.as_secs_f64() * 1000.0
    );

    // Extended benchmark with different batch sizes
    println!("\\n🔬 Extended Benchmark: Batch Size Impact");
    println!("=========================================");

    let extended_tags: Vec<String> = (1..=100).map(|i| format!("TestTag_{:03}", i)).collect();

    for &batch_size in &[1, 5, 10, 20, 50] {
        println!("\\n📊 Testing batch size: {}", batch_size);

        let num_operations = batch_size.min(extended_tags.len());
        let test_operations: Vec<BatchOperation> = extended_tags
            .iter()
            .take(num_operations)
            .map(|tag| BatchOperation::Read {
                tag_name: tag.clone(),
            })
            .collect();

        // Measure batch performance
        let start = Instant::now();
        let _results = client.execute_batch(&test_operations).await;
        let batch_time = start.elapsed();

        println!(
            "  ⏱️  {} operations in {:.2}ms ({:.2}ms per operation)",
            num_operations,
            batch_time.as_secs_f64() * 1000.0,
            batch_time.as_secs_f64() * 1000.0 / num_operations as f64
        );
    }

    // Write benchmark
    println!("\\n✏️  Write Operations Benchmark");
    println!("===============================");

    let write_operations = vec![
        BatchOperation::Write {
            tag_name: "TestWrite_1".to_string(),
            value: PlcValue::Dint(42),
        },
        BatchOperation::Write {
            tag_name: "TestWrite_2".to_string(),
            value: PlcValue::Real(3.14),
        },
        BatchOperation::Write {
            tag_name: "TestWrite_3".to_string(),
            value: PlcValue::Bool(true),
        },
    ];

    println!("\\n🐌 Individual writes:");
    let start = Instant::now();
    for op in &write_operations {
        let tag_start = Instant::now();
        match op {
            BatchOperation::Write { tag_name, value } => {
                match client.write_tag(tag_name, value.clone()).await {
                    Ok(_) => println!(
                        "  ✅ {}: Success ({:.2}ms)",
                        tag_name,
                        tag_start.elapsed().as_secs_f64() * 1000.0
                    ),
                    Err(e) => println!(
                        "  ❌ {}: {} ({:.2}ms)",
                        tag_name,
                        e,
                        tag_start.elapsed().as_secs_f64() * 1000.0
                    ),
                }
            }
            _ => unreachable!("Should only have write operations"),
        }
    }
    let individual_write_time = start.elapsed();

    println!("\\n🚀 Batch writes:");
    let batch_start = Instant::now();
    let batch_write_results = client.execute_batch(&write_operations).await?;
    let batch_write_time = batch_start.elapsed();

    for batch_result in batch_write_results {
        let tag_name = match &batch_result.operation {
            BatchOperation::Write { tag_name, .. } => tag_name,
            _ => unreachable!("Should only have write operations"),
        };
        match batch_result.result {
            Ok(None) => println!("  ✅ {}: Success", tag_name),
            Ok(Some(_)) => println!("  ⚠️  {}: Unexpected value result", tag_name),
            Err(e) => println!("  ❌ {}: {}", tag_name, e),
        }
    }

    println!("\\n⚡ Write Performance:");
    println!(
        "  Individual writes: {:.2}ms",
        individual_write_time.as_secs_f64() * 1000.0
    );
    println!(
        "  Batch writes:      {:.2}ms",
        batch_write_time.as_secs_f64() * 1000.0
    );
    let write_speedup = individual_write_time.as_secs_f64() / batch_write_time.as_secs_f64();
    println!("  📈 Write speedup:   {:.1}x faster", write_speedup);

    // Mixed operations benchmark
    println!("\\n🔄 Mixed Operations Benchmark");
    println!("==============================");

    let mixed_operations = vec![
        BatchOperation::Read {
            tag_name: "ReadTest_1".to_string(),
        },
        BatchOperation::Write {
            tag_name: "WriteTest_1".to_string(),
            value: PlcValue::Dint(100),
        },
        BatchOperation::Read {
            tag_name: "ReadTest_2".to_string(),
        },
        BatchOperation::Write {
            tag_name: "WriteTest_2".to_string(),
            value: PlcValue::Real(25.5),
        },
    ];

    let mixed_start = Instant::now();
    let mixed_results = client.execute_batch(&mixed_operations).await?;
    let mixed_time = mixed_start.elapsed();

    println!("\\n🔄 Mixed batch results:");
    for batch_result in mixed_results {
        let tag_name = match &batch_result.operation {
            BatchOperation::Read { tag_name } => tag_name,
            BatchOperation::Write { tag_name, .. } => tag_name,
        };
        match batch_result.result {
            Ok(Some(value)) => println!("  📊 {}: {:?}", tag_name, value),
            Ok(None) => println!("  ✅ {}: Success (write)", tag_name),
            Err(e) => println!("  ❌ {}: {}", tag_name, e),
        }
    }
    println!(
        "  ⏱️  Mixed operations completed in {:.2}ms",
        mixed_time.as_secs_f64() * 1000.0
    );

    println!("\\n🎉 Benchmark completed successfully!");
    Ok(())
}
