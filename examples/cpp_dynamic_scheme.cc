#include <cstdint>
#include <iostream>

#include "material_color/material_color.hpp"

int main() {
  const uint32_t source = 0xff4285f4;
  const uint32_t primary = material_color::DynamicSchemeColor(
      source, MATERIAL_COLOR_VARIANT_TONAL_SPOT, false, 0.0,
      MATERIAL_COLOR_ROLE_PRIMARY);

  std::cout << std::hex << primary << '\n';
  return 0;
}
