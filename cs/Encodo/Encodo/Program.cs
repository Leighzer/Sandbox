using System;
using System.Security.Cryptography;
using System.Text;
using System.Linq;
using System.IO;

namespace Encodo
{
    public class Program
    {
        public static void Main(string[] args)
        {
            File.WriteAllText("text.txt", "");

            RNGCryptoServiceProvider rng = new RNGCryptoServiceProvider();
            byte[] randomBytes = new byte[16];

            rng.GetBytes(randomBytes);
            
            StringBuilder bin = new StringBuilder();
            StringBuilder tern = new StringBuilder();
            StringBuilder quat = new StringBuilder();
            StringBuilder dec = new StringBuilder();
            StringBuilder hex = new StringBuilder();            

            StringBuilder ascii = new StringBuilder();
            StringBuilder iso88951 =  new StringBuilder();//ISO-8895-1

            StringBuilder utf7 = new StringBuilder();
            StringBuilder utf8 = new StringBuilder();
            StringBuilder utf16 = new StringBuilder();
            StringBuilder utf32 = new StringBuilder();

            StringBuilder base64 = new StringBuilder();

            for (int i = 0; i < randomBytes.Length; i++)
            {
                byte b = randomBytes[i];
                byte[] barr = new byte[] { b };
                
                bin.Append(Convert.ToString(b, 2).PadLeft(8, '0'));
                //tern.Append(Convert.ToString(b, 3).PadLeft(6, '0')); // todo
                //quat.Append(Convert.ToString(b, 4).PadLeft(4, '0'));
                dec.Append(Convert.ToString(b, 10).PadLeft(3, '0'));
                hex.Append(Convert.ToString(b, 16).PadLeft(2, '0'));                                
            }
            ascii.Append(Encoding.ASCII.GetString(randomBytes));
            iso88951.Append(Encoding.GetEncoding("ISO-8859-1").GetString(randomBytes));

            utf7.Append(Encoding.UTF7.GetString(randomBytes));
            utf8.Append(Encoding.UTF8.GetString(randomBytes));
            utf16.Append(Encoding.Unicode.GetString(randomBytes));
            utf32.Append(Encoding.UTF32.GetString(randomBytes));

            base64.Append(Convert.ToBase64String(randomBytes));

            File.WriteAllBytes("text.txt",randomBytes);
            
            Print("Binary");
            Print(bin.ToString());
            Print("Ternary");
            Print(tern.ToString());
            Print("Quaternary");
            Print(quat.ToString());
            Print("Decimal");
            Print(dec.ToString());
            Print("Hex");
            Print(hex.ToString());
            
            Print("ASCII");
            Print(ascii.ToString());
            Print("ISO-8859-1");
            Print(iso88951.ToString());

            Print("UTF7");
            Print(utf7.ToString());
            Print("UTF8");
            Print(utf8.ToString());
            Print("Unicode");
            Print(utf16.ToString());
            Print("UTF32");
            Print(utf32.ToString());

            Print("Base64");
            Print(base64.ToString());
            Console.ReadLine();
        }

        private static void Print(string s)
        {
            Console.Write(s);
            File.AppendAllText("text.txt", s);
        }
    }
}
