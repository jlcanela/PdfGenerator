echo "run build"

# Navigate to PdfGenerator directory
Set-Location -Path PdfGenerator
cargo build --release
cargo test
Set-Location -Path ..

# Create directory if it doesn't exist (PowerShell equivalent of mkdir -p)
New-Item -ItemType Directory -Path "PdfApp\bin\Debug\net6.0" -Force

# Copy the DLL (Windows equivalent of .so file)
Copy-Item -Path "PdfGenerator\target\release\pdf_generator.dll" -Destination "PdfApp\bin\Debug\net6.0\" -Force
