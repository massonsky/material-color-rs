import material_color_py as mc


def main() -> None:
    assert mc.__version__ == "1.0.0"
    assert mc.VERSION == "1.0.0"
    assert mc.VERSION_MAJOR == 1
    assert mc.VERSION_MINOR == 0
    assert mc.VERSION_PATCH == 0
    assert mc.version() == "1.0.0"

    blue = mc.Argb.from_rgb(66, 133, 244)
    assert int(blue) == 0xFF4285F4
    assert blue.alpha == 0xFF
    assert blue.red == 66
    assert blue.green == 133
    assert blue.blue == 244
    assert blue.is_opaque

    hct = mc.Hct.from_argb(blue.value)
    assert 250.0 < hct.hue < 280.0
    assert 50.0 < hct.chroma < 70.0
    assert 50.0 < hct.tone < 60.0

    palette = mc.TonalPalette.from_argb(blue.value)
    assert palette.get(40.0) == 0xFF005AC1
    assert palette.key_color().argb == 0xFF2B74E2

    scheme = mc.DynamicScheme(blue.value, mc.Variant.TONAL_SPOT, False, 0.0)
    assert scheme.color(mc.DynamicColorRole.PRIMARY) == 0xFF445E91
    assert scheme.color(mc.DynamicColorRole.ON_PRIMARY) == 0xFFFFFFFF
    assert scheme.tone(mc.DynamicColorRole.PRIMARY) == 40.0
    assert scheme.hct(mc.DynamicColorRole.PRIMARY).argb == 0xFF445E91

    populations = mc.quantize_celebi(
        [0xFFFF0000, 0xFFFF0000, 0xFF00FF00, 0xFF0000FF],
        3,
    )
    assert populations[0xFFFF0000] == 2
    assert sum(populations.values()) == 4

    suggestions = mc.ranked_suggestions(populations, 3)
    assert len(suggestions) >= 1
    assert all(isinstance(color, int) for color in suggestions)

    assert mc.blend_harmonize(0xFFFF0000, blue.value) == 0xFFFB0057
    assert mc.blend_hct_hue(0xFFFF0000, blue.value, 0.5) == 0xFFEB00BA
    assert mc.temperature_complement(blue.value) == 0xFFD06E00
    assert len(mc.temperature_analogous(blue.value, 5, 12)) == 5

    try:
        mc.temperature_analogous(blue.value, 0, 12)
    except ValueError:
        pass
    else:
        raise AssertionError("temperature_analogous must reject zero count")


if __name__ == "__main__":
    main()
