import json
from pathlib import Path

import material_color_py as mc


ROOT = Path(__file__).resolve().parents[1]
GOLDEN = ROOT / "golden" / "material_color_golden.json"


def parse_argb(value: str) -> int:
    return int(value, 16)


def format_argb(value: int) -> str:
    return f"0x{value:08x}"


def assert_near(actual: float, expected: float, tolerance: float = 0.001) -> None:
    if abs(actual - expected) > tolerance:
        raise AssertionError(f"actual {actual} expected {expected} tolerance {tolerance}")


def load_pixels(relative_path: str) -> list[int]:
    path = ROOT / relative_path
    return [
        parse_argb(line.strip())
        for line in path.read_text().splitlines()
        if line.strip()
    ]


def check_source_colors(golden: dict) -> None:
    for source in golden["source_colors"]:
        argb = parse_argb(source["argb"])
        hct = mc.Hct.from_argb(argb)
        assert_near(hct.hue, source["hct"]["hue"])
        assert_near(hct.chroma, source["hct"]["chroma"])
        assert_near(hct.tone, source["hct"]["tone"])

        palette = mc.TonalPalette.from_argb(argb)
        assert format_argb(palette.key_color().argb) == source["palette"]["key_color"]
        assert format_argb(palette.get(40.0)) == source["palette"]["tone_40"]

        for scheme_golden in source["schemes"]:
            variant = getattr(mc.Variant, scheme_golden["variant"])
            scheme = mc.DynamicScheme(
                argb,
                variant,
                scheme_golden["dark"],
                scheme_golden["contrast"],
            )
            for role_name, expected in scheme_golden["roles"].items():
                role = getattr(mc.DynamicColorRole, role_name)
                assert format_argb(scheme.color(role)) == expected


def check_extraction(golden: dict) -> None:
    extraction = golden["extraction"]
    populations = mc.quantize_celebi(
        load_pixels(extraction["fixture"]),
        extraction["max_colors"],
    )
    actual_populations = {
        format_argb(argb): population
        for argb, population in populations.items()
    }
    assert actual_populations == extraction["populations"]

    ranked = extraction["ranked"]
    actual_suggestions = [
        format_argb(argb)
        for argb in mc.ranked_suggestions(
            populations,
            ranked["desired"],
            parse_argb(ranked["fallback"]),
            ranked["filter"],
        )
    ]
    assert actual_suggestions == ranked["suggestions"]


def check_blend_temperature(golden: dict) -> None:
    values = golden["blend_temperature"]
    source = parse_argb(values["source"])
    design = parse_argb(values["design"])

    assert format_argb(mc.blend_harmonize(design, source)) == values["harmonize"]
    assert format_argb(mc.blend_hct_hue(design, source, 0.5)) == values["hct_hue_50"]
    assert format_argb(mc.temperature_complement(source)) == values["complement"]
    assert [
        format_argb(argb)
        for argb in mc.temperature_analogous(source, 5, 12)
    ] == values["analogous"]


def main() -> None:
    golden = json.loads(GOLDEN.read_text())

    assert mc.__version__ == golden["version"]
    assert mc.version() == golden["version"]
    check_source_colors(golden)
    check_extraction(golden)
    check_blend_temperature(golden)


if __name__ == "__main__":
    main()
