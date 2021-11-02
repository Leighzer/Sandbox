#include <iostream>

bool isPrime(int i) {
    if (i <= 1) {
        return false;
    }

    for (int j = 2; j < i; j++) {
        if (i % j == 0) {
            return false;
        }
    }

    return true;
}

int main()
{
    for (int i = 0; i < 1000; i++) {
        std::cout << i << ": " << (isPrime(i) ? "Yes" : "No") << "\n";
    }
}