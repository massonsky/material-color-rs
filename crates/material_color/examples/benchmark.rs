use std::hint::black_box;
use std::time::{Duration, Instant};

use material_color::argb::Argb;
use material_color::cam::Hct;
use material_color::dynamic::{DynamicColorRole, DynamicScheme, Variant};
use material_color::quantize;
use material_color::score::{ScoreOptions, ranked_suggestions};

fn measure(label: &str, mut work: impl FnMut()) -> Duration {
    let started = Instant::now();
    work();
    let elapsed = started.elapsed();
    println!("{label}: {elapsed:?}");
    elapsed
}

fn main() {
    let source = Argb::new(0xff42_85f4);

    measure("hct_roundtrip_10k", || {
        for index in 0..10_000 {
            let hue = f64::from(index % 360);
            black_box(Hct::new(hue, 48.0, 52.0).to_argb());
        }
    });

    measure("dynamic_roles_10k", || {
        for _ in 0..10_000 {
            let scheme = DynamicScheme::from_argb(source, Variant::TonalSpot, false, 0.0);
            for role in DynamicColorRole::ALL {
                black_box(scheme.color(role));
            }
        }
    });

    let pixels = (0..4096)
        .map(|index| {
            let red = u8::try_from((index * 37) & 0xff).unwrap_or(0);
            let green = u8::try_from((index * 73) & 0xff).unwrap_or(0);
            let blue = u8::try_from((index * 109) & 0xff).unwrap_or(0);
            Argb::from_rgb(red, green, blue)
        })
        .collect::<Vec<_>>();

    measure("quantize_score_4k", || {
        let quantized = quantize::quantize_celebi(&pixels, 16);
        black_box(ranked_suggestions(
            &quantized.color_to_count,
            ScoreOptions {
                desired: 8,
                fallback_color_argb: source,
                filter: true,
            },
        ));
    });
}
