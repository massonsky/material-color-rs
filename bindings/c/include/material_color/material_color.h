#ifndef MATERIAL_COLOR_MATERIAL_COLOR_H_
#define MATERIAL_COLOR_MATERIAL_COLOR_H_

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#define MATERIAL_COLOR_VERSION_MAJOR 1
#define MATERIAL_COLOR_VERSION_MINOR 0
#define MATERIAL_COLOR_VERSION_PATCH 0
#define MATERIAL_COLOR_VERSION "1.0.0"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum MaterialColorStatus {
  MATERIAL_COLOR_STATUS_OK = 0,
  MATERIAL_COLOR_STATUS_INVALID_ARGUMENT = 1,
  MATERIAL_COLOR_STATUS_PANIC = 255,
} MaterialColorStatus;

typedef enum MaterialColorVariant {
  MATERIAL_COLOR_VARIANT_MONOCHROME = 0,
  MATERIAL_COLOR_VARIANT_NEUTRAL = 1,
  MATERIAL_COLOR_VARIANT_TONAL_SPOT = 2,
  MATERIAL_COLOR_VARIANT_VIBRANT = 3,
  MATERIAL_COLOR_VARIANT_EXPRESSIVE = 4,
  MATERIAL_COLOR_VARIANT_FIDELITY = 5,
  MATERIAL_COLOR_VARIANT_CONTENT = 6,
  MATERIAL_COLOR_VARIANT_RAINBOW = 7,
  MATERIAL_COLOR_VARIANT_FRUIT_SALAD = 8,
} MaterialColorVariant;

typedef enum MaterialColorRole {
  MATERIAL_COLOR_ROLE_PRIMARY_PALETTE_KEY_COLOR = 0,
  MATERIAL_COLOR_ROLE_SECONDARY_PALETTE_KEY_COLOR = 1,
  MATERIAL_COLOR_ROLE_TERTIARY_PALETTE_KEY_COLOR = 2,
  MATERIAL_COLOR_ROLE_NEUTRAL_PALETTE_KEY_COLOR = 3,
  MATERIAL_COLOR_ROLE_NEUTRAL_VARIANT_PALETTE_KEY_COLOR = 4,
  MATERIAL_COLOR_ROLE_BACKGROUND = 5,
  MATERIAL_COLOR_ROLE_ON_BACKGROUND = 6,
  MATERIAL_COLOR_ROLE_SURFACE = 7,
  MATERIAL_COLOR_ROLE_SURFACE_DIM = 8,
  MATERIAL_COLOR_ROLE_SURFACE_BRIGHT = 9,
  MATERIAL_COLOR_ROLE_SURFACE_CONTAINER_LOWEST = 10,
  MATERIAL_COLOR_ROLE_SURFACE_CONTAINER_LOW = 11,
  MATERIAL_COLOR_ROLE_SURFACE_CONTAINER = 12,
  MATERIAL_COLOR_ROLE_SURFACE_CONTAINER_HIGH = 13,
  MATERIAL_COLOR_ROLE_SURFACE_CONTAINER_HIGHEST = 14,
  MATERIAL_COLOR_ROLE_ON_SURFACE = 15,
  MATERIAL_COLOR_ROLE_SURFACE_VARIANT = 16,
  MATERIAL_COLOR_ROLE_ON_SURFACE_VARIANT = 17,
  MATERIAL_COLOR_ROLE_INVERSE_SURFACE = 18,
  MATERIAL_COLOR_ROLE_INVERSE_ON_SURFACE = 19,
  MATERIAL_COLOR_ROLE_OUTLINE = 20,
  MATERIAL_COLOR_ROLE_OUTLINE_VARIANT = 21,
  MATERIAL_COLOR_ROLE_SHADOW = 22,
  MATERIAL_COLOR_ROLE_SCRIM = 23,
  MATERIAL_COLOR_ROLE_SURFACE_TINT = 24,
  MATERIAL_COLOR_ROLE_PRIMARY = 25,
  MATERIAL_COLOR_ROLE_ON_PRIMARY = 26,
  MATERIAL_COLOR_ROLE_PRIMARY_CONTAINER = 27,
  MATERIAL_COLOR_ROLE_ON_PRIMARY_CONTAINER = 28,
  MATERIAL_COLOR_ROLE_INVERSE_PRIMARY = 29,
  MATERIAL_COLOR_ROLE_SECONDARY = 30,
  MATERIAL_COLOR_ROLE_ON_SECONDARY = 31,
  MATERIAL_COLOR_ROLE_SECONDARY_CONTAINER = 32,
  MATERIAL_COLOR_ROLE_ON_SECONDARY_CONTAINER = 33,
  MATERIAL_COLOR_ROLE_TERTIARY = 34,
  MATERIAL_COLOR_ROLE_ON_TERTIARY = 35,
  MATERIAL_COLOR_ROLE_TERTIARY_CONTAINER = 36,
  MATERIAL_COLOR_ROLE_ON_TERTIARY_CONTAINER = 37,
  MATERIAL_COLOR_ROLE_ERROR = 38,
  MATERIAL_COLOR_ROLE_ON_ERROR = 39,
  MATERIAL_COLOR_ROLE_ERROR_CONTAINER = 40,
  MATERIAL_COLOR_ROLE_ON_ERROR_CONTAINER = 41,
  MATERIAL_COLOR_ROLE_PRIMARY_FIXED = 42,
  MATERIAL_COLOR_ROLE_PRIMARY_FIXED_DIM = 43,
  MATERIAL_COLOR_ROLE_ON_PRIMARY_FIXED = 44,
  MATERIAL_COLOR_ROLE_ON_PRIMARY_FIXED_VARIANT = 45,
  MATERIAL_COLOR_ROLE_SECONDARY_FIXED = 46,
  MATERIAL_COLOR_ROLE_SECONDARY_FIXED_DIM = 47,
  MATERIAL_COLOR_ROLE_ON_SECONDARY_FIXED = 48,
  MATERIAL_COLOR_ROLE_ON_SECONDARY_FIXED_VARIANT = 49,
  MATERIAL_COLOR_ROLE_TERTIARY_FIXED = 50,
  MATERIAL_COLOR_ROLE_TERTIARY_FIXED_DIM = 51,
  MATERIAL_COLOR_ROLE_ON_TERTIARY_FIXED = 52,
  MATERIAL_COLOR_ROLE_ON_TERTIARY_FIXED_VARIANT = 53,
} MaterialColorRole;

typedef struct MaterialColorHct {
  double hue;
  double chroma;
  double tone;
  uint32_t argb;
} MaterialColorHct;

typedef struct MaterialColorPopulation {
  uint32_t argb;
  uint32_t population;
} MaterialColorPopulation;

typedef struct MaterialColorArgbArray {
  uint32_t* data;
  size_t len;
} MaterialColorArgbArray;

typedef struct MaterialColorPopulationArray {
  MaterialColorPopulation* data;
  size_t len;
} MaterialColorPopulationArray;

uint32_t material_color_version_major(void);
uint32_t material_color_version_minor(void);
uint32_t material_color_version_patch(void);

uint32_t material_color_argb_from_rgb(uint8_t red, uint8_t green, uint8_t blue);
double material_color_lstar_from_argb(uint32_t argb);
uint32_t material_color_int_from_lstar(double lstar);

MaterialColorStatus material_color_hct_from_argb(uint32_t argb,
                                                 MaterialColorHct* out_hct);
MaterialColorStatus material_color_hct_to_argb(double hue, double chroma,
                                               double tone,
                                               uint32_t* out_argb);
MaterialColorStatus material_color_tonal_palette_get(double hue, double chroma,
                                                     double tone,
                                                     uint32_t* out_argb);

MaterialColorStatus material_color_blend_harmonize(uint32_t design_color,
                                                   uint32_t key_color,
                                                   uint32_t* out_argb);
MaterialColorStatus material_color_blend_hct_hue(uint32_t from, uint32_t to,
                                                 double amount,
                                                 uint32_t* out_argb);
MaterialColorStatus material_color_blend_cam16_ucs(uint32_t from, uint32_t to,
                                                   double amount,
                                                   uint32_t* out_argb);

double material_color_contrast_ratio_of_tones(double tone_a, double tone_b);

MaterialColorStatus material_color_dynamic_scheme_color(
    uint32_t source_argb, int32_t variant, bool is_dark, double contrast_level,
    int32_t role, uint32_t* out_argb);

MaterialColorStatus material_color_quantize_celebi(
    const uint32_t* pixels, size_t pixel_count, uint16_t max_colors,
    MaterialColorPopulationArray* out_entries);

MaterialColorStatus material_color_score_ranked(
    const MaterialColorPopulation* entries, size_t entry_count, size_t desired,
    uint32_t fallback_color_argb, bool filter, MaterialColorArgbArray* out_colors);

MaterialColorStatus material_color_temperature_complement(uint32_t argb,
                                                          uint32_t* out_argb);
MaterialColorStatus material_color_temperature_analogous(
    uint32_t argb, size_t count, size_t divisions,
    MaterialColorArgbArray* out_colors);

void material_color_argb_array_free(MaterialColorArgbArray array);
void material_color_population_array_free(MaterialColorPopulationArray array);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // MATERIAL_COLOR_MATERIAL_COLOR_H_
