#include <cstdint>
#include <vector>

#include "material_color/material_color.hpp"

int main() {
  static_assert(MATERIAL_COLOR_VERSION_MAJOR == 1);
  static_assert(MATERIAL_COLOR_VERSION_MINOR == 0);
  static_assert(MATERIAL_COLOR_VERSION_PATCH == 0);
  static_assert(material_color::kHeaderVersion.major == 1);

  const material_color::Version runtime_version = material_color::RuntimeVersion();
  if (runtime_version.major != 1 || runtime_version.minor != 0 ||
      runtime_version.patch != 0) {
    return 10;
  }

  const uint32_t blue = material_color::ArgbFromRgb(0, 0, 255);
  if (blue != 0xff0000ff) {
    return 1;
  }

  const material_color::Hct hct = material_color::HctFromArgb(blue);
  if (hct.tone < 32.0 || hct.tone > 33.0) {
    return 2;
  }

  const uint32_t primary = material_color::DynamicSchemeColor(
      blue, MATERIAL_COLOR_VARIANT_TONAL_SPOT, false, 0.0,
      MATERIAL_COLOR_ROLE_PRIMARY);
  if (primary == 0) {
    return 3;
  }

  const std::vector<uint32_t> pixels = {
      0xffff0000, 0xffff0000, 0xff00ff00, 0xff00ff00, 0xff00ff00};
  auto populations = material_color::QuantizeCelebi(pixels, 256);
  if (populations.size() != 2) {
    return 4;
  }

  const uint32_t complement = material_color::TemperatureComplement(blue);
  if (complement != 0xff9d0002) {
    return 5;
  }

  return 0;
}
