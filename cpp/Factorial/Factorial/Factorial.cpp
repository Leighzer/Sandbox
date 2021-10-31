#include <iostream>

int main()
{   
    // not accurate beyond 20
    for (int i = 0; i < 20; i++) {
        unsigned long long num = (unsigned long long) i;
        unsigned long long sum = 1l;
        while (num > 0) {
            sum *= num--;
        }
        std::cout << i << ": " << sum << "\n";
    }
}