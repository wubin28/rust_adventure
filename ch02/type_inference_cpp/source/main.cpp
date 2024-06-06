#include <iostream>
#include <typeinfo>

template<typename T>
std::string type_name(T&&)
{
  return typeid(T).name();
}

auto main() -> int
{
  auto x = 5;
  auto y = 3.0;
  auto z = "Hello, World!";

  std::cout << "The type of x is: " << type_name(x) << std::endl; // The type of x is: i (int)
  std::cout << "The type of y is: " << type_name(y) << std::endl; // The type of y is: d (double)
  std::cout << "The type of z is: " << type_name(z) << std::endl; // The type of z is: PKc (const char*)

  return 0;
}
