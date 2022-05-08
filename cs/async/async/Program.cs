using System;
using System.Diagnostics;
using System.Threading.Tasks;

namespace async
{
    // code to highlight advantage of async vs sync code
    public class Program
    {
        public const string sync_arg = "sync";
        public const string async_arg = "async";
        public const int delay = 1000000;

        public static void Main(string[] args)
        {
            if (args.Length != 1)
            {
                Console.WriteLine($"You must supply 1 argument {sync_arg} or {async_arg}");
                return;
            }
            else
            {
                Stopwatch watch = new Stopwatch();
                string arg = args[0];
                if (arg != sync_arg && arg != async_arg)
                {
                    Console.WriteLine($"You must supply 1 argument {sync_arg} or {async_arg}");
                    return;
                }
                else if (arg == sync_arg)
                {
                    watch.Restart();
                    ulong sum = Sync(watch);
                    watch.Stop();
                    Console.WriteLine(sum + $" {watch.ElapsedTicks}");
                }
                else
                {
                    watch.Restart();
                    ulong sum = Async(watch).GetAwaiter().GetResult();
                    watch.Stop();
                    Console.WriteLine(sum + $" {watch.ElapsedTicks}");
                }
            }
        }

        public static ulong Sync(Stopwatch watch)
        {
            Console.WriteLine($"Begin delay {watch.ElapsedTicks}");
            Task.Delay(10000).GetAwaiter().GetResult();
            Console.WriteLine($"end delay {watch.ElapsedTicks}");

            Console.WriteLine($"Begin sum {watch.ElapsedTicks}");
            ulong sum = 0;
            for (ulong i = 0; i < 1000000000; i++)
            {
                sum = sum + i;
            }
            Console.WriteLine($"End sum {watch.ElapsedTicks}");

            return sum;
        }

        public static async Task<ulong> Async(Stopwatch watch)
        {
            Console.WriteLine($"Begin delay {watch.ElapsedTicks}");
            Task.Delay(10000);
            Console.WriteLine($"End delay {watch.ElapsedTicks}");

            Console.WriteLine($"Begin sum {watch.ElapsedTicks}");
            ulong sum = 0;
            for (ulong i = 0; i < 1000000000; i++)
            {
                sum = sum + i;
            }
            Console.WriteLine($"End sum {watch.ElapsedTicks}");

            return sum;
        }
    }
}
