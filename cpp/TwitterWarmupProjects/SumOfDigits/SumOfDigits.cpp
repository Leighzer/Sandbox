#include <string>
#include <iostream>

unsigned long long SumOfDigits(std::string numbers) {
    unsigned long long sum = 0;
    for (int i = 0; i < numbers.length(); i++) {
        std::string c = numbers.substr(i,1);
        int parsedNum = stoi(c);
        sum += parsedNum;
    }

    return sum;
}

int main()
{   
    std::cout << SumOfDigits("561961324941234165461321341896516165189616351316549876513215648974613") << "\n";
}