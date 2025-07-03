#!/bin/bash
mkdir -p wider_face
cd wider_face

# Download WIDER FACE training images using curl
echo "Downloading WIDER FACE dataset..."
curl -L -o WIDER_train.zip https://huggingface.co/datasets/wider_face/resolve/main/data/WIDER_train.zip

# Check if download was successful
if [ -f "WIDER_train.zip" ]; then
    echo "Unzipping dataset..."
    unzip WIDER_train.zip
    rm WIDER_train.zip
    echo "Dataset downloaded to ./wider_face/WIDER_train/"
else
    echo "Download failed. Please download manually from http://shuoyang1213.me/WIDERFACE/"
fi
