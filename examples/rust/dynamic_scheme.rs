use material_color::VERSION;
use material_color::argb::Argb;
use material_color::dynamic::{DynamicColorRole, DynamicScheme, Variant};

fn main() {
    let source = Argb::from_rgb(66, 133, 244);
    let scheme = DynamicScheme::from_argb(source, Variant::TonalSpot, false, 0.0);

    let primary = scheme.color(DynamicColorRole::Primary);
    let on_primary = scheme.color(DynamicColorRole::OnPrimary);
    let container = scheme.color(DynamicColorRole::PrimaryContainer);

    println!("material_color {VERSION}");
    println!("source=0x{:08x}", source.to_u32());
    println!("primary=0x{:08x}", primary.to_u32());
    println!("on_primary=0x{:08x}", on_primary.to_u32());
    println!("primary_container=0x{:08x}", container.to_u32());

    assert_eq!(VERSION, "1.0.0");
    assert_eq!(primary.to_u32(), 0xff44_5e91);
    assert_eq!(on_primary.to_u32(), 0xffff_ffff);
    assert_eq!(container.to_u32(), 0xffd8_e2ff);
}
