#include <iostream>

void fib(int max) {
    int x = 0;
    int y = 1;
    int temp = 0;

    while (x < max && x >= 0) {
        std::cout << x << "\n";
        temp = x + y;
        x = y;
        y = temp;
    }
}

int main()
{
    fib(INT_MAX);
}

