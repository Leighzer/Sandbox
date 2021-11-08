#include <iostream>
#include <string>
#include <vector>

bool IsArmstrong(int number);

int main()
{   
    const int maxIntToSearch = 10000000;
    for (int i = 0; i < maxIntToSearch; i++)
    {
        bool isArmstrong = IsArmstrong(i);
        if (isArmstrong) {
            std::cout << i << ": " << (isArmstrong ? "Yes" : "No") << "\n";
        }
    }
    std::cout << "DONE SEARCHING TO " << maxIntToSearch << "\n";
}

bool IsArmstrong(int number) {
    int originalNumber = number;
    std::vector<int> digits;

    while (number > 0)
    {
        digits.push_back(number % 10);

        number /= 10;
    }

    int sum = 0;
    for (int i = 0; i < digits.size(); i++) {
        sum += digits[i] * digits[i] * digits[i];
    }

    bool result = sum == originalNumber;

    return result;
}