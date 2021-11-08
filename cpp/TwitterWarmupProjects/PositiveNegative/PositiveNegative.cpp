#include <iostream>
#include <stdlib.h> 
#include <time.h>

int main()
{
    srand(time(NULL));

    int number = rand() % 50 - 25; 

    if (number < 0) {
        std::cout << number << " is negative!";
    }
    else if (number == 0) {
        std::cout << number << " is zero!";
    }
    else {
        std::cout << number << " is positive!";
    }

    return 0;
}