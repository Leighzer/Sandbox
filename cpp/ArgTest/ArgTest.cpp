#include <iostream>
#include <vector>

int main(int argc, char* argv[])
{
    std::vector<std::string> args(argv, argv + argc);

    for (int i = 0; i < args.size(); i++) {
        std::cout << args[i] << "\n";
    }
}