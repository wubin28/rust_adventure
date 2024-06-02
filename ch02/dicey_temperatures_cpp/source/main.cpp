#include <iostream>
#include <random>
#include <stdexcept>
#include <string>

auto main() -> int
{
  std::cout << "Guess the sum of two dice!" << std::endl;

  // 使用随机数引擎生成2到12之间的随机数
  std::random_device rd;
  std::mt19937 gen(rd());
  std::uniform_int_distribution<> dis(2, 12);

  int sum_of_two_dice = dis(gen);

  std::cout << "The sum of two dice is: " << sum_of_two_dice << std::endl;

  std::cout << "Please input your guess (between 2 and 12): " << std::endl;

  std::string guess;
  std::getline(std::cin, guess);

  int guess_number;
  try {
    guess_number = std::stoi(guess);
  } catch (const std::invalid_argument&) {
    std::cerr << "Please type a number!" << std::endl;
    return 1;
  }

  std::cout << "You guessed: " << guess_number << std::endl;

  if (guess_number < sum_of_two_dice || guess_number > sum_of_two_dice) {
    std::cout << "You guessed it wrong on the first try" << std::endl;
  } else {
    std::cout << "You win!" << std::endl;
  }

  return 0;
}
