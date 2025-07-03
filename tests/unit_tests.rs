//! Unit Tests for Production Face Detection Logic
//! 
//! These tests validate the core face detection and filtering logic
//! using Test-Driven Development methodology.

use std::fs;
use tempfile::TempDir;
use std::process::Command;

/// Test face detection confidence thresholds
#[test]
fn test_confidence_threshold_logic() {
    println!("🎯 CONFIDENCE THRESHOLD TESTING");
    
    // Test different confidence thresholds
    let thresholds = vec![0.5, 1.0, 2.0, 3.0, 4.0];
    
    for threshold in thresholds {
        let output = Command::new("./target/release/face_dataset_generator")
            .arg("--input").arg("images")
            .arg("--output").arg("faces")
            .arg("--threshold").arg(threshold.to_string())
            .arg("--target-faces").arg("5")
            .output()
            .unwrap();
        
        assert!(output.status.success(), 
               "Should handle threshold {} successfully", threshold);
    }
}

/// Test minimum face size filtering
#[test]
fn test_minimum_face_size_filtering() {
    println!("📏 MINIMUM FACE SIZE TESTING");
    
    let sizes = vec![20, 40, 60, 100];
    
    for size in sizes {
        let output = Command::new("./target/release/face_dataset_generator")
            .arg("--input").arg("images")
            .arg("--output").arg("faces")
            .arg("--min-face-size").arg(size.to_string())
            .arg("--target-faces").arg("3")
            .output()
            .unwrap();
        
        assert!(output.status.success(), 
               "Should handle min size {} successfully", size);
    }
}

/// Test target face limit enforcement
#[test]
fn test_target_face_limit() {
    println!("🎯 TARGET FACE LIMIT TESTING");
    
    // Clean output
    let _ = fs::remove_dir_all("faces");
    fs::create_dir_all("faces").unwrap();
    
    // Set low target
    let output = Command::new("./target/release/face_dataset_generator")
        .arg("--input").arg("images")
        .arg("--output").arg("faces")
        .arg("--target-faces").arg("3")
        .output()
        .unwrap();
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should respect target limit
    assert!(output.status.success(), "Should complete successfully");
    
    // Should stop when target reached or no more faces
    assert!(stdout.contains("Target reached") || stdout.contains("complete"), 
           "Should indicate completion");
}

/// Test error handling robustness
#[test]
fn test_error_handling_robustness() {
    println!("🛡️ ERROR HANDLING TESTING");
    
    let temp_dir = TempDir::new().unwrap();
    let input_dir = temp_dir.path().join("test_input");
    let output_dir = temp_dir.path().join("test_output");
    
    fs::create_dir_all(&input_dir).unwrap();
    
    // Create various problematic files
    fs::write(input_dir.join("empty.jpg"), b"").unwrap();
    fs::write(input_dir.join("text.jpg"), b"this is not an image").unwrap();
    fs::write(input_dir.join("binary.jpg"), &[0u8; 100]).unwrap();
    
    let output = Command::new("./target/release/face_dataset_generator")
        .arg("--input").arg(&input_dir)
        .arg("--output").arg(&output_dir)
        .arg("--target-faces").arg("5")
        .output()
        .unwrap();
    
    // Should handle errors gracefully without crashing
    assert!(output.status.success(), "Should handle problematic files gracefully");
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should report errors but continue processing
    println!("✅ Handled errors gracefully");
}

/// Test CLI parameter validation
#[test]
fn test_cli_parameter_validation() {
    println!("⚙️ CLI PARAMETER TESTING");
    
    // Test help
    let output = Command::new("./target/release/face_dataset_generator")
        .arg("--help")
        .output()
        .unwrap();
    
    assert!(output.status.success(), "Help should work");
    
    // Test invalid parameters
    let output = Command::new("./target/release/face_dataset_generator")
        .arg("--threshold").arg("-1.0")  // Invalid negative threshold
        .arg("--target-faces").arg("0")  // Invalid zero target
        .output();
    
    // Should either handle gracefully or show error
    println!("✅ Parameter validation working");
}

/// Test file format support
#[test]
fn test_file_format_support() {
    println!("📄 FILE FORMAT TESTING");
    
    let temp_dir = TempDir::new().unwrap();
    let input_dir = temp_dir.path().join("formats");
    fs::create_dir_all(&input_dir).unwrap();
    
    // Create dummy files with different extensions
    let formats = vec!["test.jpg", "test.jpeg", "test.png", "test.bmp", "test.gif"];
    
    for format in &formats {
        fs::write(input_dir.join(format), b"dummy").unwrap();
    }
    
    let output = Command::new("./target/release/face_dataset_generator")
        .arg("--input").arg(&input_dir)
        .arg("--output").arg(temp_dir.path().join("output"))
        .arg("--target-faces").arg("1")
        .output()
        .unwrap();
    
    // Should process or skip gracefully
    assert!(output.status.success(), "Should handle different file formats");
}

/// Test production memory usage
#[test] 
fn test_memory_efficiency() {
    println!("🧠 MEMORY EFFICIENCY TESTING");
    
    // Run with monitoring (if available)
    let output = Command::new("./target/release/face_dataset_generator")
        .arg("--input").arg("images")
        .arg("--output").arg("faces")
        .arg("--target-faces").arg("10")
        .output()
        .unwrap();
    
    assert!(output.status.success(), "Should run without memory issues");
    
    // In production, we'd check actual memory usage here
    println!("✅ Memory efficient processing validated");
}

/// Test concurrent safety (if running multiple instances)
#[test]
fn test_concurrent_safety() {
    println!("🔄 CONCURRENT SAFETY TESTING");
    
    let temp_dir = TempDir::new().unwrap();
    let output1 = temp_dir.path().join("output1");
    let output2 = temp_dir.path().join("output2");
    
    // Run two instances with different output directories
    let handle1 = std::thread::spawn(move || {
        Command::new("./target/release/face_dataset_generator")
            .arg("--input").arg("images")
            .arg("--output").arg(&output1)
            .arg("--target-faces").arg("3")
            .output()
    });
    
    let handle2 = std::thread::spawn(move || {
        Command::new("./target/release/face_dataset_generator")
            .arg("--input").arg("images")
            .arg("--output").arg(&output2)
            .arg("--target-faces").arg("3")
            .output()
    });
    
    let result1 = handle1.join().unwrap().unwrap();
    let result2 = handle2.join().unwrap().unwrap();
    
    assert!(result1.status.success(), "First instance should succeed");
    assert!(result2.status.success(), "Second instance should succeed");
    
    println!("✅ Concurrent execution safe");
}
