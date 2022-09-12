namespace Cast
{
    public class Program
    {
        public static void Main(string[] args)
        {   
            TestClass testObj1 = (TestClass) GetTestClass(1); // will throw exception if can't cast
            Console.WriteLine(testObj1.Count);

            TestClass? testObj2 = GetTestClass(2) as TestClass; // will return null if can't cast - can only be used for nullable types
            Console.WriteLine(testObj2.Count);

            if (GetTestClass(3) is TestClass testObj3)
            {
                Console.WriteLine(testObj3.Count);
            }

            TestClass testObj4 = GetTestClass2(4);
            Console.WriteLine(testObj4.Count);
        }

        public static object GetTestClass(int i)
        {
            return new TestClass()
            {
                Count = i,
            };
        }

        public static TestClass2 GetTestClass2(int i)
        {
            return new TestClass2()
            {
                Count = i
            };
        }
    }

    public class TestClass
    {
        public int Count { get; set; }

        public static implicit operator TestClass(TestClass2 test)
        {
            return new TestClass()
            {
                Count = test.Count
            };
        }
    }

    public class TestClass2
    {
        public int Count { get; set; }
    }
}