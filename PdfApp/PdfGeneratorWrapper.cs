using System.Runtime.InteropServices;
using PdfGenerator;

public static class PdfGeneratorWrapper
{
    public static byte[] GeneratePdf(string input, string jsonConfig)
    {
        // Call the native function
        var buffer = new ByteBuffer();

        var result = Pdf.generate_pdf(input, jsonConfig, out buffer);
        
        try
        {
            // Check for null pointer
            if (result != FFIError.Ok)
            {
                throw new InvalidOperationException("PDF generation failed - null pointer returned");
            }

            // Convert to managed byte array
              var bytes = new byte[buffer.len];
            Marshal.Copy(buffer.ptr, bytes, 0, (int)buffer.len);
            return bytes;
        }
        finally
        {
            // Always free the unmanaged memory
            Pdf.free_binary_data(buffer);
        }
    }
}
