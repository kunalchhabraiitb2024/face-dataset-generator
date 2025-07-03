#!/bin/bash

# Face Dataset Generator - Focused Edge Case Test Images
# This script downloads a curated set of test images that validate
# the face extraction system across different challenging scenarios.

set -e

echo "🎯 Downloading focused edge case test images..."

mkdir -p images
cd images

# Test scenarios included:
echo "📸 Downloading 7 focused test cases..."

# 1. Baseline - Good quality frontal face
echo "  1/7 Baseline case..."
curl -s "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=400&h=400&fit=crop&crop=face" -o good_001.jpg

# 2. Multiple faces - Tests selection logic  
echo "  2/7 Multiple faces..."
curl -s "https://images.unsplash.com/photo-1529626455594-4ff0802cfb7e?w=400&h=400&fit=crop" -o multi_001.jpg

# 3. Profile view - Side angle detection
echo "  3/7 Profile view..."
curl -s "https://images.unsplash.com/photo-1463453091185-61582044d556?w=400&h=400&fit=crop&crop=face" -o profile_001.jpg

# 4. Accessories - Face with glasses
echo "  4/7 Accessories (glasses)..."
curl -s "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=400&h=400&fit=crop&crop=face" -o glasses_001.jpg

# 5. Poor lighting - Shadows and contrast issues
echo "  5/7 Poor lighting..."
curl -s "https://images.unsplash.com/photo-1506794778202-cad84cf45f1d?w=400&h=400&fit=crop&crop=face" -o shadow_001.jpg

# 6. Age diversity - Elderly person
echo "  6/7 Age diversity..."
curl -s "https://images.unsplash.com/photo-1582750433449-648ed127bb54?w=400&h=400&fit=crop&crop=face" -o elderly_001.jpg

# 7. Error handling - Corrupted file
echo "  7/7 Error handling..."
echo "corrupted_image_data" > corrupted_001.jpg

cd ..

echo "✅ Downloaded 7 focused edge case test images"
echo ""
echo "📋 Test Coverage:"
echo "  ✅ Baseline detection (good_001.jpg)"
echo "  ✅ Multiple face selection (multi_001.jpg)"
echo "  ✅ Profile view handling (profile_001.jpg)"
echo "  ✅ Accessories/occlusion (glasses_001.jpg)"
echo "  ✅ Poor lighting conditions (shadow_001.jpg)"
echo "  ✅ Age diversity (elderly_001.jpg)"
echo "  ✅ Error handling (corrupted_001.jpg)"
echo ""
echo "🚀 Run test: ./target/release/face_dataset_generator --input images --output faces"
