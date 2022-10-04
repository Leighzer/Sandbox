namespace Record
{
    // records are basically syntax sugar for classes
    // with immutable fields and some other nice features
    // equality comparison is override to compare fields
    public class Program
    {
        public static void Main(string[] args)
        {
            var person1 = new Person()
            {
                FirstName = "Jane",
                LastName = "Doe",
                BirthDate = DateTime.Today,
                EmailAddress = "test@leighzer.com"
            };

            var person2 = new Person()
            {
                FirstName = "Bob",
                LastName = "Doe",
                BirthDate = DateTime.Today.AddYears(-1),
                EmailAddress = "test2@leighzer.com"
            };

            var person3 = person1;

            Console.WriteLine(person1);
            Console.WriteLine(person2);
            Console.WriteLine(person3);
            Console.WriteLine($"person1 == person2: {person1 == person2}");
            Console.WriteLine($"person1 == person3: {person1 == person3}");

            var individual1 = new Individual("Charlie", "Doe", DateTime.Today.AddDays(-1), "test3@leighzer.com");
            var individual2 = new Individual("Charlie", "Doe", DateTime.Today.AddDays(-1), "test3@leighzer.com");
            var individual3 = new Individual("Dan", "Doe", DateTime.Today.AddDays(-30), "test4@leighzer.com");
            var individual4 = individual1 with { LastName = "Dill" };

            Console.WriteLine(individual1);
            Console.WriteLine(individual2);
            Console.WriteLine($"individual1 == individual2: {individual1 == individual2}");
            Console.WriteLine($"individual1 ReferenceEquals individual2: {ReferenceEquals(individual1, individual2)}");
            Console.WriteLine(individual3);
            Console.WriteLine(individual4);

            // deconstruct
            var (firstName, lastName) = individual1.GetNames();
        }
    }

    public class Person
    {
        public string FirstName { get; set; }
        public string LastName { get; set; }
        public DateTime BirthDate { get; set; }
        public string EmailAddress { get; set; }
    }

    public record Individual(string FirstName, string LastName, DateTime BirthDate, string EmailAddress)
    {
        public (string, string) GetNames()
        {
            return (FirstName, LastName);
        }
    };
}