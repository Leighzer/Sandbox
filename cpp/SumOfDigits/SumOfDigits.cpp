#include <vector>
#include <string>
#include <sstream>
#include <iostream>

// c++ you janky
unsigned long long SumOfDigits(std::string numbers) {
    std::stringstream ststr;
    unsigned long long sum = 0;
    for (int i = 0; i < numbers.length(); i++) {
        ststr << numbers[i];
        int parsedNum;
        ststr >> parsedNum;
        sum += parsedNum;
        ststr.clear();
        // sum += numbers[i] - '0';
    }

    return sum;
}

int main()
{
    std::cout << SumOfDigits("561961324941234165461321341896516165189616351316549876513215648974613") << "\n";
}