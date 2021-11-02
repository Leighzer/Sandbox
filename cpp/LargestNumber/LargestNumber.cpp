#include <iostream>
#include <stdlib.h> 
#include <time.h>
#include <vector>
#include <iostream>
#include <string>
#include <vector>

int main()
{
    srand(time(NULL));

    std::vector<int> numbers;
    for (int i = 0; i < 1000; i++) {
        numbers.push_back(rand());
    }

    int largestNumber = -1;
    for (int i = 0; i < numbers.size(); i++) {
        std::cout << numbers[i] << "\n";
        if (numbers[i] > largestNumber) {
            largestNumber = numbers[i];
        }
    }

    std::cout << "Largest number: " << largestNumber;
}