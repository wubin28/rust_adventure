#include "lib.hpp"

auto main() -> int
{
  auto const lib = library {};

  return lib.name == "hello_world_cpp" ? 0 : 1;
}
