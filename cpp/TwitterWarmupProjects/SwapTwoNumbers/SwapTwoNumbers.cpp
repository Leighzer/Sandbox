#include <iostream>

int main()
{
    int x = 10;
    int y = -10;
    int tmp = 0;

    std::cout << "x: " << x << " y: " << y << "\n";

    tmp = x;
    x = y;
    y = tmp;

    std::cout << "x: " << x << " y: " << y << "\n";
}