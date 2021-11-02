#include <iostream>

bool isPalindrome(std::string possiblePalindrome) {
    bool isOddLength = possiblePalindrome.length() % 2 == 1;
    int iterations = possiblePalindrome.length() / 2;
    if (isOddLength) {
        iterations++;
    }

    bool isNotPalindrome = false;
    for (int i = 0; i < iterations; i++) {
        char first = possiblePalindrome[i];
        char mirror = possiblePalindrome[possiblePalindrome.length() - i - 1];

        if (first != mirror) {
            isNotPalindrome = true;
            break;
        }
    }

    return !isNotPalindrome;
}

int main(int argc, char** argv)
{
    if (argc >= 2) {
        std::string possiblePalindrome = argv[1];

        std::cout << (isPalindrome(possiblePalindrome) ? "Yes" : "No") << "\n";
    }
    else {
        std::cout << "Not enough arguments supplied.";
    }

    return 0;
}