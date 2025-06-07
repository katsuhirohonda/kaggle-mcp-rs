#!/bin/bash

# Script to run kaggle-mcp-rs examples

echo "🚀 Running kaggle-mcp-rs example: competitions_list"
echo "=================================================="
echo ""
echo "Make sure you have set up your Kaggle credentials:"
echo "  - Either set KAGGLE_USERNAME and KAGGLE_KEY environment variables"
echo "  - Or create ~/.kaggle/kaggle.json with your credentials"
echo ""
echo "Press Enter to continue or Ctrl+C to exit..."
read

cargo run --example competitions_list