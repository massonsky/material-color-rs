import material_color_py as mc


def main() -> None:
    source = mc.Argb.from_rgb(66, 133, 244)
    scheme = mc.DynamicScheme(source.value, mc.Variant.TONAL_SPOT, False, 0.0)

    primary = scheme.color(mc.DynamicColorRole.PRIMARY)
    on_primary = scheme.color(mc.DynamicColorRole.ON_PRIMARY)
    container = scheme.color(mc.DynamicColorRole.PRIMARY_CONTAINER)

    print(f"material_color_py {mc.__version__}")
    print(f"source=0x{source.value:08x}")
    print(f"primary=0x{primary:08x}")
    print(f"on_primary=0x{on_primary:08x}")
    print(f"primary_container=0x{container:08x}")

    assert mc.__version__ == "1.0.0"
    assert primary == 0xFF445E91
    assert on_primary == 0xFFFFFFFF
    assert container == 0xFFD8E2FF


if __name__ == "__main__":
    main()
