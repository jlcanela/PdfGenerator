using Wasmtime;
using PdfGenerator;
class Program
{
    static void Main(string[] args)
    {

        // Usage:
        try
        {
            string typstCode = @"#set page(width: 210mm, height: 297mm)
                        Hello, World!";
            string jsonConfig = "{}";

            byte[] pdfBytes = PdfGeneratorWrapper.GeneratePdf(typstCode, jsonConfig);
            File.WriteAllBytes("output.pdf", pdfBytes);
        }
        catch (Exception ex)
        {
            Console.WriteLine($"Error generating PDF: {ex.Message}");
        }

    }
}