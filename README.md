# Face Dataset Generator

A production-ready Rust application for extracting faces from images using the RustFace detection library. This tool processes batches of images and outputs cropped face images suitable for machine learning datasets.

---

## Features

- **Efficient Face Detection**: Uses RustFace (SeetaFace) for accurate face detection
- **Quality Filtering**: Filters faces based on size, confidence, and aspect ratio
- **Batch Processing**: Processes multiple images sequentially with progress tracking
- **Smart Cropping**: Adds padding around detected faces for better context
- **Production Ready**: Clean error handling, logging, and configurable parameters
- **Tested & Benchmarked**: Comprehensive test suite and performance benchmarks

---

## Quick Start

### 1. Build the project
```bash
cargo build --release
```

### 2. Download sample images and model
```bash
./download_samples.sh
./download_model.sh
```

### 3. Extract faces from images
```bash
./target/release/face_dataset_generator --input ./images --output ./faces --target-faces 100
```

### 4. (Optional) Download the WIDER FACE dataset
```bash
./download_wider_face.sh
```

### 5. (Optional) Run on WIDER FACE dataset
```bash
./target/release/face_dataset_generator --wider-face-dir ./wider_face --output-dir ./output_faces
```

---

## Data Management & .gitignore

- The following folders are **ignored by git** (see `.gitignore`):
  - `images/` (input images)
  - `wider_face/` (WIDER FACE dataset)
  - `faces/` (output faces)
- **Do not commit large datasets or generated images.**
- If you add new data folders, update `.gitignore` accordingly.

---

## Building from Source

1. Clone the repository:
```bash
git clone https://github.com/kunalchhabraiitb2024/face-dataset-generator.git
cd face-dataset-generator
```

2. Build the project:
```bash
cargo build --release
```

The executable will be available at `target/release/face_dataset_generator`.

---

## Usage

```bash
face_dataset_generator [OPTIONS]
```

**OPTIONS:**
- `-i, --input <PATH>`          Input directory containing images [default: ./images]
- `-o, --output <PATH>`         Output directory for extracted faces [default: ./faces]
- `-m, --model <PATH>`          Path to face detection model [default: ./model.bin]
- `--min-face-size <PIXELS>`    Minimum face size in pixels [default: 40]
- `--threshold <FLOAT>`         Confidence threshold (0.0-5.0) [default: 2.0]
- `--target-faces <COUNT>`      Target number of faces to extract [default: 5000]
- `-h, --help`                  Print help information

---

## Architecture & Design Decisions

### Model Choice: RustFace (SeetaFace)
- **Why**: Lightweight, fast, and doesn't require GPU
- **Production Benefits**: Easy deployment, consistent performance across environments
- **Trade-offs**: Slightly lower accuracy than deep learning models, but 10x faster

### Face Quality Filtering
- **Size filtering**: Face must be 2-40% of image area
- **Confidence threshold**: RustFace score > 2.0
- **Aspect ratio**: Width/height between 0.5-2.0
- **Minimum dimensions**: At least 40x40 pixels

### Edge Cases Handled
- Invalid/corrupted images
- No faces detected
- Multiple faces per image
- Profile views, accessories, poor lighting, age diversity
- Target face count limit
- Memory efficiency

---

## Testing & Validation

### Test Suite
- **Production TDD Tests**: 9
- **Unit Tests**: 8
- **Benchmarks**: 4
- **Edge Case Scenarios**: 7

**Run all tests:**
```bash
cargo test
```

**Run specific tests:**
```bash
cargo test --test production_tdd
cargo test --test unit_tests
cargo test --test benchmarks
```

**Edge case validation:**
```bash
./download_focused_tests.sh
./target/release/face_dataset_generator --input images --output faces --target-faces 100
```

---

## Performance

- **Speed**: ~100-200ms per image on modern hardware
- **Memory**: Constant usage regardless of dataset size
- **Scalability**: Handles large datasets
- **Throughput**: ~5,000 faces/hour (single thread)

---

## File Structure

```
face_dataset_generator/
├── src/main.rs                 # Main application logic
├── Cargo.toml                  # Dependencies and build config
├── model.bin                   # Face detection model (SeetaFace)
├── download_samples.sh         # Download sample images
├── download_model.sh           # Download face detection model
├── download_wider_face.sh      # Download WIDER FACE dataset
├── download_focused_tests.sh   # Download edge case test images
├── benchmark.sh                # Performance validation script
├── tests/                      # Test suite
│   ├── production_tdd.rs
│   ├── unit_tests.rs
│   └── benchmarks.rs
├── images/                     # Input images (ignored by git)
├── faces/                      # Output faces (ignored by git)
├── wider_face/                 # WIDER FACE dataset (ignored by git)
├── target/release/             # Compiled binary
└── README.md                   # Documentation
```

---

## Production Deployment

- **Containerization**: Use Docker for deployment
- **Parallel Processing**: Run multiple instances for speed
- **Quality Assurance**: Add face verification for critical use
- **Monitoring**: Track processing rate, error rate, quality
- **Storage**: Use object storage (e.g., S3) for large datasets

---

## Dependencies

- `rustface`: Face detection
- `image`: Image processing
- `clap`: CLI argument parsing
- `anyhow`: Error handling
- `walkdir`: Directory traversal

---

## License

MIT License - See LICENSE file for details.
