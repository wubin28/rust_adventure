#include "lib.hpp"

auto main() -> int
{
  auto const lib = library {};

  return lib.name == "type_inference_cpp" ? 0 : 1;
}
