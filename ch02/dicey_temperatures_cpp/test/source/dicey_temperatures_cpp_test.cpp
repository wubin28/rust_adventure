#include "lib.hpp"

auto main() -> int
{
  auto const lib = library {};

  return lib.name == "dicey_temperatures_cpp" ? 0 : 1;
}
