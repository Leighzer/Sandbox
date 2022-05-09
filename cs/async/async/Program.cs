using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Text;
using System.Threading;
using System.Threading.Tasks;

namespace async
{
    // code to highlight advantage of async vs sync code
    public class Program
    {
        public const int Delay = 1000;
        public static void Main()
        {
            MainAsync().GetAwaiter().GetResult();
        }

        public static async Task MainAsync()
        {
            Stopwatch watch = new Stopwatch();
            watch.Restart();
            for (int i = 0; i < 10; i++)
            {
                DoWork(i, watch);
            }
            watch.Stop();
            long syncMilliseconds = watch.ElapsedMilliseconds;
            Console.WriteLine($"Sync milliseconds for {syncMilliseconds}\n");
                
            watch.Restart();
            List<Task> asyncTasks = new List<Task>();
            for (int i = 10; i < 20; i++)
            {
                // start each task and track of them in a list
                asyncTasks.Add(DowWorkAsync(i, watch));
            }
            // wait for when all of our tasks have completed
            await Task.WhenAll(asyncTasks);
            watch.Stop();
            long asyncMilliseconds = watch.ElapsedMilliseconds;
            Console.WriteLine($"Async milliseconds for {asyncMilliseconds}\n");

            Console.WriteLine($"Sync is {syncMilliseconds - asyncMilliseconds} milliseconds slower than async in this contrived example");
        }

        public static int DoWork(int taskId, Stopwatch watch)
        {
            Console.WriteLine($"Task {taskId} start {watch.ElapsedMilliseconds}");
            Thread.Sleep(Delay);
            Console.WriteLine($"Task {taskId} end {watch.ElapsedMilliseconds}");
            return 0;
        }

        public static async Task<int> DowWorkAsync(int taskId, Stopwatch watch)
        {
            // Schedule work to run on thread pool
            // Task.Run will return a task we can await and keep track of the work with
            var test = await Task.Run(() => DoWork(taskId, watch));
            return test;
        }
    }

    
}
