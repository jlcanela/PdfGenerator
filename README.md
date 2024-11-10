# Sample project to generate PDF file in C# server side

## Setup for Windows

In windows setup you need rust and dotnet tooling to build. 
The pdf_generator.dll is prebuilt for windows. 

To run the PdfApp: 
```
cd PdfApp
dotnet run
```

To build the pdf_generator.dll
```
.\build.ps1
```

## Setup for Linux (currently broken, need cross compile config)

In linux setup you rust and dotnet tooling as usual. 

Build the rust shared library
```
./build.sh
```

Run the C# .Net console app:
```
cd PdfApp
dotnet run
```

And check the generated output.pdf file. 

# Typst

The library is just repackaging [typst](https://typst.app/) an awesome tooling to generate various documents. 
