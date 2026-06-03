#include <cstdint>
#include <iomanip>
#include <iostream>
#include <vector>

#include "material_color/material_color.hpp"

int main() {
  const std::vector<uint32_t> pixels = {
      0xffff0000, 0xffff0000, 0xffff0000, 0xff00ff00,
      0xff00ff00, 0xff0000ff, 0xffffffff, 0xffffffff,
      0xffffffff, 0xffffffff, 0xff000000, 0xff000000,
  };

  auto populations = material_color::QuantizeCelebi(pixels, 4);
  std::vector<material_color::Population> entries;
  entries.reserve(populations.size());

  for (const auto& [argb, population] : populations.ToMap()) {
    entries.push_back({argb, population});
  }

  auto suggestions =
      material_color::RankedSuggestions(entries, 4, 0xff4285f4, true);
  const std::vector<uint32_t> colors = suggestions.ToVector();

  if (colors.empty()) {
    return 1;
  }

  for (uint32_t color : colors) {
    std::cout << "0x" << std::hex << std::setw(8) << std::setfill('0') << color
              << '\n';
  }

  return 0;
}
