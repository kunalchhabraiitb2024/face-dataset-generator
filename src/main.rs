use anyhow::{Context, Result};
use clap::Parser;
use image::{DynamicImage, GenericImageView, GrayImage, RgbImage};
use rustface::{Detector, FaceInfo, ImageData};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "face_extractor")]
#[command(about = "Extract faces from images using RustFace detector")]
struct Args {
    /// Input directory containing images
    #[arg(short, long, default_value = "./images")]
    input: PathBuf,

    /// Output directory for extracted faces
    #[arg(short, long, default_value = "./faces")]
    output: PathBuf,

    /// Path to the face detection model
    #[arg(short, long, default_value = "./model.bin")]
    model: PathBuf,

    /// Minimum face size (pixels)
    #[arg(long, default_value = "40")]
    min_face_size: u32,

    /// Confidence threshold (0.0-5.0)
    #[arg(long, default_value = "2.0")]
    threshold: f64,

    /// Target number of faces to extract
    #[arg(long, default_value = "5000")]
    target_faces: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("🚀 Face Dataset Generator");
    println!("Target: {} faces", args.target_faces);

    // Create output directory
    fs::create_dir_all(&args.output)
        .context("Failed to create output directory")?;

    // Load face detection model
    let mut detector = rustface::create_detector(args.model.to_str().unwrap())
        .context("Failed to load face detection model")?;

    // Configure detector
    detector.set_min_face_size(args.min_face_size);
    detector.set_score_thresh(args.threshold);
    detector.set_pyramid_scale_factor(0.8);
    detector.set_slide_window_step(4, 4);

    println!("✅ Model loaded and configured");

    // Find all image files
    let image_paths: Vec<PathBuf> = WalkDir::new(&args.input)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| {
            let path = e.path();
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_str()?.to_lowercase();
                if matches!(ext_str.as_str(), "jpg" | "jpeg" | "png" | "bmp") {
                    Some(path.to_path_buf())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    println!("📁 Found {} images to process", image_paths.len());

    if image_paths.is_empty() {
        println!("❌ No images found in {}", args.input.display());
        return Ok(());
    }

    let face_counter = AtomicUsize::new(0);
    let mut processed = 0;
    let mut errors = 0;

    // Process images sequentially
    for (i, path) in image_paths.iter().enumerate() {
        let current_count = face_counter.load(Ordering::Relaxed);
        if current_count >= args.target_faces {
            println!("🎯 Target reached! Extracted {} faces", current_count);
            break;
        }

        println!("[{}/{}] Processing: {}", i + 1, image_paths.len(), path.display());
        
        match process_image(path, &args.output, &mut *detector, &face_counter, args.target_faces) {
            Ok(extracted) => {
                processed += 1;
                if extracted > 0 {
                    println!("  ✅ Extracted {} faces", extracted);
                }
            }
            Err(e) => {
                errors += 1;
                eprintln!("  ❌ Error: {}", e);
            }
        }
    }

    let final_count = face_counter.load(Ordering::Relaxed);
    println!("\n🎉 Processing complete!");
    println!("📊 Results:");
    println!("  - Images processed: {}", processed);
    println!("  - Errors: {}", errors);
    println!("  - Faces extracted: {}", final_count);
    println!("  - Output directory: {}", args.output.display());

    Ok(())
}

fn process_image(
    image_path: &Path,
    output_dir: &Path,
    detector: &mut dyn Detector,
    face_counter: &AtomicUsize,
    target: usize,
) -> Result<usize> {
    // Check if we've already reached our target
    let current_count = face_counter.load(Ordering::Relaxed);
    if current_count >= target {
        return Ok(0);
    }

    // Load image
    let image = image::open(image_path)
        .context("Failed to open image")?;

    // Detect faces
    let faces = detect_faces(detector, &image.to_luma8())?;
    
    if faces.is_empty() {
        return Ok(0);
    }

    // Filter valid faces (good size, confidence)
    let valid_faces = filter_valid_faces(&faces, &image);
    
    if valid_faces.is_empty() {
        return Ok(0);
    }

    // Extract and save faces
    let mut extracted = 0;
    let filename_stem = image_path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");

    for (i, face) in valid_faces.iter().enumerate() {
        let current = face_counter.load(Ordering::Relaxed);
        if current >= target {
            break;
        }

        let bbox = face.bbox();
        
        // Crop face from original image with padding
        let padding = ((bbox.width() + bbox.height()) / 8) as i32; // 12.5% padding
        let x = (bbox.x() - padding).max(0) as u32;
        let y = (bbox.y() - padding).max(0) as u32;
        let width = ((bbox.width() as i32 + 2 * padding) as u32).min(image.width() - x);
        let height = ((bbox.height() as i32 + 2 * padding) as u32).min(image.height() - y);
        
        let face_img = image.crop_imm(x, y, width, height);

        // Generate unique filename
        let face_filename = format!("{}_{:04}_{:.0}.jpg", 
            filename_stem, 
            current + 1,
            face.score() * 100.0
        );
        let face_path = output_dir.join(face_filename);

        // Save face
        face_img.save(&face_path)
            .context("Failed to save face image")?;

        face_counter.fetch_add(1, Ordering::Relaxed);
        extracted += 1;
    }

    Ok(extracted)
}

fn detect_faces(detector: &mut dyn Detector, gray: &GrayImage) -> Result<Vec<FaceInfo>> {
    let (width, height) = gray.dimensions();
    let mut image_data = ImageData::new(gray, width, height);
    let faces = detector.detect(&mut image_data);
    Ok(faces)
}

fn filter_valid_faces<'a>(faces: &'a [FaceInfo], image: &DynamicImage) -> Vec<&'a FaceInfo> {
    let (img_width, img_height) = image.dimensions();
    let img_area = (img_width * img_height) as f64;
    
    faces
        .iter()
        .filter(|face| {
            let bbox = face.bbox();
            let face_area = (bbox.width() * bbox.height()) as f64;
            let face_ratio = face_area / img_area;
            
            // Face should be 2-40% of image area (removes tiny and huge faces)
            let size_ok = face_ratio > 0.02 && face_ratio < 0.4;
            
            // Good confidence score (RustFace uses different scale)
            let confidence_ok = face.score() > 2.0;
            
            // Face should be reasonably rectangular (not too thin/wide)
            let aspect_ratio = bbox.width() as f64 / bbox.height() as f64;
            let ratio_ok = aspect_ratio > 0.5 && aspect_ratio < 2.0;
            
            // Minimum size check
            let min_size_ok = bbox.width() >= 40 && bbox.height() >= 40;
            
            size_ok && confidence_ok && ratio_ok && min_size_ok
        })
        .collect()
}
