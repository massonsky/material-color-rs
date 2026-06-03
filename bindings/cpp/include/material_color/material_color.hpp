#ifndef MATERIAL_COLOR_MATERIAL_COLOR_HPP_
#define MATERIAL_COLOR_MATERIAL_COLOR_HPP_

#include <cstdint>
#include <map>
#include <stdexcept>
#include <utility>
#include <vector>

#include "material_color/material_color.h"

namespace material_color {

using Status = MaterialColorStatus;
using Variant = MaterialColorVariant;
using Role = MaterialColorRole;
using Hct = MaterialColorHct;
using Population = MaterialColorPopulation;

inline void Check(Status status) {
  if (status != MATERIAL_COLOR_STATUS_OK) {
    throw std::runtime_error("material_color C ABI call failed");
  }
}

class OwnedArgbArray {
 public:
  OwnedArgbArray() = default;
  explicit OwnedArgbArray(MaterialColorArgbArray array) : array_(array) {}

  OwnedArgbArray(const OwnedArgbArray&) = delete;
  OwnedArgbArray& operator=(const OwnedArgbArray&) = delete;

  OwnedArgbArray(OwnedArgbArray&& other) noexcept : array_(other.array_) {
    other.array_ = {};
  }

  OwnedArgbArray& operator=(OwnedArgbArray&& other) noexcept {
    if (this != &other) {
      Reset();
      array_ = other.array_;
      other.array_ = {};
    }
    return *this;
  }

  ~OwnedArgbArray() { Reset(); }

  [[nodiscard]] const uint32_t* data() const { return array_.data; }
  [[nodiscard]] size_t size() const { return array_.len; }

  [[nodiscard]] std::vector<uint32_t> ToVector() const {
    if (array_.data == nullptr || array_.len == 0) {
      return {};
    }
    return {array_.data, array_.data + array_.len};
  }

 private:
  void Reset() {
    material_color_argb_array_free(array_);
    array_ = {};
  }

  MaterialColorArgbArray array_{};
};

class OwnedPopulationArray {
 public:
  OwnedPopulationArray() = default;
  explicit OwnedPopulationArray(MaterialColorPopulationArray array)
      : array_(array) {}

  OwnedPopulationArray(const OwnedPopulationArray&) = delete;
  OwnedPopulationArray& operator=(const OwnedPopulationArray&) = delete;

  OwnedPopulationArray(OwnedPopulationArray&& other) noexcept
      : array_(other.array_) {
    other.array_ = {};
  }

  OwnedPopulationArray& operator=(OwnedPopulationArray&& other) noexcept {
    if (this != &other) {
      Reset();
      array_ = other.array_;
      other.array_ = {};
    }
    return *this;
  }

  ~OwnedPopulationArray() { Reset(); }

  [[nodiscard]] const Population* data() const { return array_.data; }
  [[nodiscard]] size_t size() const { return array_.len; }

  [[nodiscard]] std::map<uint32_t, uint32_t> ToMap() const {
    std::map<uint32_t, uint32_t> result;
    for (size_t index = 0; index < array_.len; ++index) {
      result[array_.data[index].argb] = array_.data[index].population;
    }
    return result;
  }

 private:
  void Reset() {
    material_color_population_array_free(array_);
    array_ = {};
  }

  MaterialColorPopulationArray array_{};
};

[[nodiscard]] inline uint32_t ArgbFromRgb(uint8_t red, uint8_t green,
                                          uint8_t blue) {
  return material_color_argb_from_rgb(red, green, blue);
}

[[nodiscard]] inline double LstarFromArgb(uint32_t argb) {
  return material_color_lstar_from_argb(argb);
}

[[nodiscard]] inline Hct HctFromArgb(uint32_t argb) {
  Hct hct{};
  Check(material_color_hct_from_argb(argb, &hct));
  return hct;
}

[[nodiscard]] inline uint32_t HctToArgb(double hue, double chroma,
                                        double tone) {
  uint32_t argb = 0;
  Check(material_color_hct_to_argb(hue, chroma, tone, &argb));
  return argb;
}

[[nodiscard]] inline uint32_t TonalPaletteGet(double hue, double chroma,
                                              double tone) {
  uint32_t argb = 0;
  Check(material_color_tonal_palette_get(hue, chroma, tone, &argb));
  return argb;
}

[[nodiscard]] inline uint32_t Harmonize(uint32_t design_color,
                                        uint32_t key_color) {
  uint32_t argb = 0;
  Check(material_color_blend_harmonize(design_color, key_color, &argb));
  return argb;
}

[[nodiscard]] inline uint32_t DynamicSchemeColor(uint32_t source_argb,
                                                 Variant variant, bool is_dark,
                                                 double contrast_level,
                                                 Role role) {
  uint32_t argb = 0;
  Check(material_color_dynamic_scheme_color(
      source_argb, static_cast<int32_t>(variant), is_dark, contrast_level,
      static_cast<int32_t>(role), &argb));
  return argb;
}

[[nodiscard]] inline OwnedPopulationArray QuantizeCelebi(
    const std::vector<uint32_t>& pixels, uint16_t max_colors) {
  MaterialColorPopulationArray entries{};
  Check(material_color_quantize_celebi(pixels.data(), pixels.size(), max_colors,
                                      &entries));
  return OwnedPopulationArray(entries);
}

[[nodiscard]] inline OwnedArgbArray RankedSuggestions(
    const std::vector<Population>& entries, size_t desired = 4,
    uint32_t fallback_color_argb = 0xff4285f4, bool filter = true) {
  MaterialColorArgbArray colors{};
  Check(material_color_score_ranked(entries.data(), entries.size(), desired,
                                    fallback_color_argb, filter, &colors));
  return OwnedArgbArray(colors);
}

[[nodiscard]] inline uint32_t TemperatureComplement(uint32_t argb) {
  uint32_t complement = 0;
  Check(material_color_temperature_complement(argb, &complement));
  return complement;
}

[[nodiscard]] inline OwnedArgbArray TemperatureAnalogous(uint32_t argb,
                                                        size_t count = 5,
                                                        size_t divisions = 12) {
  MaterialColorArgbArray colors{};
  Check(material_color_temperature_analogous(argb, count, divisions, &colors));
  return OwnedArgbArray(colors);
}

}  // namespace material_color

#endif  // MATERIAL_COLOR_MATERIAL_COLOR_HPP_
