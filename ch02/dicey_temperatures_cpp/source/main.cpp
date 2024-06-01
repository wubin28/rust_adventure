#include <iostream>
#include <random>
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

  std::cout << "You guessed: " << guess << std::endl;

  return 0;
}
