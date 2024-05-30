#include <iostream>
#include <string>

auto main() -> int
{
  std::cout << "Guess the sum of two dice!" << std::endl;

  std::cout << "Please input your guess (between 2 and 12): " << std::endl;

  std::string guess;
  std::getline(std::cin, guess);

  std::cout << "You guessed: " << guess << std::endl;

  return 0;
}
