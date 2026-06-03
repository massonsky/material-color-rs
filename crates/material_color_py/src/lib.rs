#![allow(clippy::trivially_copy_pass_by_ref)]

use std::collections::BTreeMap;

use material_color::TemperatureCache;
use material_color::argb::Argb;
use material_color::blend;
use material_color::cam::Hct;
use material_color::dynamic::{DynamicColorRole, DynamicScheme, Variant};
use material_color::palettes::TonalPalette;
use material_color::quantize;
use material_color::score::{ScoreOptions, ranked_suggestions as core_ranked_suggestions};
use pyo3::prelude::*;
use pyo3::{exceptions::PyValueError, types::PyAny};

#[pyclass(name = "Variant", frozen, skip_from_py_object)]
#[derive(Clone, Copy, Debug)]
struct PyVariant {
    value: i32,
}

#[pymethods]
impl PyVariant {
    #[new]
    const fn new(value: i32) -> Self {
        Self { value }
    }

    #[getter]
    const fn value(&self) -> i32 {
        self.value
    }

    #[getter]
    fn name(&self) -> PyResult<&'static str> {
        Ok(self.to_core()?.name())
    }

    #[classattr]
    const MONOCHROME: Self = Self { value: 0 };
    #[classattr]
    const NEUTRAL: Self = Self { value: 1 };
    #[classattr]
    const TONAL_SPOT: Self = Self { value: 2 };
    #[classattr]
    const VIBRANT: Self = Self { value: 3 };
    #[classattr]
    const EXPRESSIVE: Self = Self { value: 4 };
    #[classattr]
    const FIDELITY: Self = Self { value: 5 };
    #[classattr]
    const CONTENT: Self = Self { value: 6 };
    #[classattr]
    const RAINBOW: Self = Self { value: 7 };
    #[classattr]
    const FRUIT_SALAD: Self = Self { value: 8 };

    fn __repr__(&self) -> String {
        format!("Variant({})", self.value)
    }
}

impl PyVariant {
    fn to_core(self) -> PyResult<Variant> {
        variant_from_i32(self.value)
    }
}

fn variant_from_i32(value: i32) -> PyResult<Variant> {
    match value {
        0 => Ok(Variant::Monochrome),
        1 => Ok(Variant::Neutral),
        2 => Ok(Variant::TonalSpot),
        3 => Ok(Variant::Vibrant),
        4 => Ok(Variant::Expressive),
        5 => Ok(Variant::Fidelity),
        6 => Ok(Variant::Content),
        7 => Ok(Variant::Rainbow),
        8 => Ok(Variant::FruitSalad),
        _ => Err(PyValueError::new_err("invalid Variant value")),
    }
}

fn variant_from_py(value: &Bound<'_, PyAny>) -> PyResult<Variant> {
    if let Ok(variant) = value.extract::<PyRef<'_, PyVariant>>() {
        return variant.to_core();
    }

    variant_from_i32(value.extract::<i32>()?)
}

#[pyclass(name = "DynamicColorRole", frozen, skip_from_py_object)]
#[derive(Clone, Copy, Debug)]
struct PyDynamicColorRole {
    value: i32,
}

#[pymethods]
impl PyDynamicColorRole {
    #[new]
    const fn new(value: i32) -> Self {
        Self { value }
    }

    #[getter]
    const fn value(&self) -> i32 {
        self.value
    }

    #[getter]
    fn name(&self) -> PyResult<&'static str> {
        Ok(self.to_core()?.name())
    }

    #[classattr]
    const PRIMARY_PALETTE_KEY_COLOR: Self = Self { value: 0 };
    #[classattr]
    const SECONDARY_PALETTE_KEY_COLOR: Self = Self { value: 1 };
    #[classattr]
    const TERTIARY_PALETTE_KEY_COLOR: Self = Self { value: 2 };
    #[classattr]
    const NEUTRAL_PALETTE_KEY_COLOR: Self = Self { value: 3 };
    #[classattr]
    const NEUTRAL_VARIANT_PALETTE_KEY_COLOR: Self = Self { value: 4 };
    #[classattr]
    const BACKGROUND: Self = Self { value: 5 };
    #[classattr]
    const ON_BACKGROUND: Self = Self { value: 6 };
    #[classattr]
    const SURFACE: Self = Self { value: 7 };
    #[classattr]
    const SURFACE_DIM: Self = Self { value: 8 };
    #[classattr]
    const SURFACE_BRIGHT: Self = Self { value: 9 };
    #[classattr]
    const SURFACE_CONTAINER_LOWEST: Self = Self { value: 10 };
    #[classattr]
    const SURFACE_CONTAINER_LOW: Self = Self { value: 11 };
    #[classattr]
    const SURFACE_CONTAINER: Self = Self { value: 12 };
    #[classattr]
    const SURFACE_CONTAINER_HIGH: Self = Self { value: 13 };
    #[classattr]
    const SURFACE_CONTAINER_HIGHEST: Self = Self { value: 14 };
    #[classattr]
    const ON_SURFACE: Self = Self { value: 15 };
    #[classattr]
    const SURFACE_VARIANT: Self = Self { value: 16 };
    #[classattr]
    const ON_SURFACE_VARIANT: Self = Self { value: 17 };
    #[classattr]
    const INVERSE_SURFACE: Self = Self { value: 18 };
    #[classattr]
    const INVERSE_ON_SURFACE: Self = Self { value: 19 };
    #[classattr]
    const OUTLINE: Self = Self { value: 20 };
    #[classattr]
    const OUTLINE_VARIANT: Self = Self { value: 21 };
    #[classattr]
    const SHADOW: Self = Self { value: 22 };
    #[classattr]
    const SCRIM: Self = Self { value: 23 };
    #[classattr]
    const SURFACE_TINT: Self = Self { value: 24 };
    #[classattr]
    const PRIMARY: Self = Self { value: 25 };
    #[classattr]
    const ON_PRIMARY: Self = Self { value: 26 };
    #[classattr]
    const PRIMARY_CONTAINER: Self = Self { value: 27 };
    #[classattr]
    const ON_PRIMARY_CONTAINER: Self = Self { value: 28 };
    #[classattr]
    const INVERSE_PRIMARY: Self = Self { value: 29 };
    #[classattr]
    const SECONDARY: Self = Self { value: 30 };
    #[classattr]
    const ON_SECONDARY: Self = Self { value: 31 };
    #[classattr]
    const SECONDARY_CONTAINER: Self = Self { value: 32 };
    #[classattr]
    const ON_SECONDARY_CONTAINER: Self = Self { value: 33 };
    #[classattr]
    const TERTIARY: Self = Self { value: 34 };
    #[classattr]
    const ON_TERTIARY: Self = Self { value: 35 };
    #[classattr]
    const TERTIARY_CONTAINER: Self = Self { value: 36 };
    #[classattr]
    const ON_TERTIARY_CONTAINER: Self = Self { value: 37 };
    #[classattr]
    const ERROR: Self = Self { value: 38 };
    #[classattr]
    const ON_ERROR: Self = Self { value: 39 };
    #[classattr]
    const ERROR_CONTAINER: Self = Self { value: 40 };
    #[classattr]
    const ON_ERROR_CONTAINER: Self = Self { value: 41 };
    #[classattr]
    const PRIMARY_FIXED: Self = Self { value: 42 };
    #[classattr]
    const PRIMARY_FIXED_DIM: Self = Self { value: 43 };
    #[classattr]
    const ON_PRIMARY_FIXED: Self = Self { value: 44 };
    #[classattr]
    const ON_PRIMARY_FIXED_VARIANT: Self = Self { value: 45 };
    #[classattr]
    const SECONDARY_FIXED: Self = Self { value: 46 };
    #[classattr]
    const SECONDARY_FIXED_DIM: Self = Self { value: 47 };
    #[classattr]
    const ON_SECONDARY_FIXED: Self = Self { value: 48 };
    #[classattr]
    const ON_SECONDARY_FIXED_VARIANT: Self = Self { value: 49 };
    #[classattr]
    const TERTIARY_FIXED: Self = Self { value: 50 };
    #[classattr]
    const TERTIARY_FIXED_DIM: Self = Self { value: 51 };
    #[classattr]
    const ON_TERTIARY_FIXED: Self = Self { value: 52 };
    #[classattr]
    const ON_TERTIARY_FIXED_VARIANT: Self = Self { value: 53 };

    fn __repr__(&self) -> String {
        format!("DynamicColorRole({})", self.value)
    }
}

impl PyDynamicColorRole {
    fn to_core(self) -> PyResult<DynamicColorRole> {
        dynamic_role_from_i32(self.value)
    }
}

fn dynamic_role_from_i32(value: i32) -> PyResult<DynamicColorRole> {
    usize::try_from(value)
        .ok()
        .and_then(|index| DynamicColorRole::ALL.get(index).copied())
        .ok_or_else(|| PyValueError::new_err("invalid DynamicColorRole value"))
}

fn dynamic_role_from_py(value: &Bound<'_, PyAny>) -> PyResult<DynamicColorRole> {
    if let Ok(role) = value.extract::<PyRef<'_, PyDynamicColorRole>>() {
        return role.to_core();
    }

    dynamic_role_from_i32(value.extract::<i32>()?)
}

#[pyclass(name = "Argb", frozen, skip_from_py_object)]
#[derive(Clone, Copy, Debug)]
struct PyArgb {
    value: u32,
}

#[pymethods]
impl PyArgb {
    #[new]
    const fn new(value: u32) -> Self {
        Self { value }
    }

    #[staticmethod]
    const fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self {
            value: Argb::from_rgb(red, green, blue).to_u32(),
        }
    }

    #[getter]
    const fn value(&self) -> u32 {
        self.value
    }

    #[getter]
    const fn alpha(&self) -> u8 {
        Argb::new(self.value).alpha()
    }

    #[getter]
    const fn red(&self) -> u8 {
        Argb::new(self.value).red()
    }

    #[getter]
    const fn green(&self) -> u8 {
        Argb::new(self.value).green()
    }

    #[getter]
    const fn blue(&self) -> u8 {
        Argb::new(self.value).blue()
    }

    #[getter]
    const fn is_opaque(&self) -> bool {
        Argb::new(self.value).is_opaque()
    }

    fn __int__(&self) -> u32 {
        self.value
    }

    fn __repr__(&self) -> String {
        format!("Argb(0x{:08x})", self.value)
    }
}

#[pyclass(name = "Hct", skip_from_py_object)]
#[derive(Clone, Copy, Debug)]
struct PyHct {
    inner: Hct,
}

#[pymethods]
impl PyHct {
    #[new]
    fn new(hue: f64, chroma: f64, tone: f64) -> Self {
        Self {
            inner: Hct::new(hue, chroma, tone),
        }
    }

    #[staticmethod]
    fn from_argb(argb: u32) -> Self {
        Self {
            inner: Hct::from_argb(Argb::new(argb)),
        }
    }

    #[getter]
    fn hue(&self) -> f64 {
        self.inner.hue()
    }

    #[getter]
    fn chroma(&self) -> f64 {
        self.inner.chroma()
    }

    #[getter]
    fn tone(&self) -> f64 {
        self.inner.tone()
    }

    #[getter]
    fn argb(&self) -> u32 {
        self.inner.to_argb().to_u32()
    }

    fn set_hue(&mut self, hue: f64) {
        self.inner.set_hue(hue);
    }

    fn set_chroma(&mut self, chroma: f64) {
        self.inner.set_chroma(chroma);
    }

    fn set_tone(&mut self, tone: f64) {
        self.inner.set_tone(tone);
    }

    fn __repr__(&self) -> String {
        format!(
            "Hct(hue={}, chroma={}, tone={}, argb=0x{:08x})",
            self.hue(),
            self.chroma(),
            self.tone(),
            self.argb()
        )
    }
}

#[pyclass(name = "TonalPalette", frozen, skip_from_py_object)]
#[derive(Clone, Copy, Debug)]
struct PyTonalPalette {
    inner: TonalPalette,
}

#[pymethods]
impl PyTonalPalette {
    #[new]
    fn new(hue: f64, chroma: f64) -> Self {
        Self {
            inner: TonalPalette::from_hue_and_chroma(hue, chroma),
        }
    }

    #[staticmethod]
    fn from_argb(argb: u32) -> Self {
        Self {
            inner: TonalPalette::from_argb(Argb::new(argb)),
        }
    }

    #[getter]
    fn hue(&self) -> f64 {
        self.inner.hue()
    }

    #[getter]
    fn chroma(&self) -> f64 {
        self.inner.chroma()
    }

    fn key_color(&self) -> PyHct {
        PyHct {
            inner: self.inner.key_color(),
        }
    }

    fn get(&self, tone: f64) -> u32 {
        self.inner.get(tone).to_u32()
    }
}

#[pyclass(name = "DynamicScheme", frozen, skip_from_py_object)]
#[derive(Clone, Copy, Debug)]
struct PyDynamicScheme {
    inner: DynamicScheme,
}

#[pymethods]
impl PyDynamicScheme {
    #[new]
    fn new(
        source_argb: u32,
        variant: &Bound<'_, PyAny>,
        is_dark: bool,
        contrast_level: f64,
    ) -> PyResult<Self> {
        Ok(Self {
            inner: DynamicScheme::from_argb(
                Argb::new(source_argb),
                variant_from_py(variant)?,
                is_dark,
                contrast_level,
            ),
        })
    }

    fn color(&self, role: &Bound<'_, PyAny>) -> PyResult<u32> {
        Ok(self.inner.color(dynamic_role_from_py(role)?).to_u32())
    }

    fn hct(&self, role: &Bound<'_, PyAny>) -> PyResult<PyHct> {
        Ok(PyHct {
            inner: self.inner.hct(dynamic_role_from_py(role)?),
        })
    }

    fn tone(&self, role: &Bound<'_, PyAny>) -> PyResult<f64> {
        Ok(self.inner.tone(dynamic_role_from_py(role)?))
    }
}

#[pyfunction]
fn blend_harmonize(design_color: u32, key_color: u32) -> u32 {
    blend::harmonize(Argb::new(design_color), Argb::new(key_color)).to_u32()
}

#[pyfunction]
fn blend_hct_hue(from: u32, to: u32, amount: f64) -> u32 {
    blend::hct_hue(Argb::new(from), Argb::new(to), amount).to_u32()
}

#[pyfunction]
#[pyo3(signature = (pixels, max_colors = 256))]
fn quantize_celebi(pixels: Vec<u32>, max_colors: u16) -> BTreeMap<u32, u32> {
    let pixels = pixels.into_iter().map(Argb::new).collect::<Vec<_>>();
    quantize::quantize_celebi(&pixels, max_colors)
        .color_to_count
        .into_iter()
        .map(|(argb, population)| (argb.to_u32(), population))
        .collect()
}

#[pyfunction]
#[pyo3(signature = (populations, desired = 4, fallback_color_argb = 0xff42_85f4, filter = true))]
fn ranked_suggestions(
    populations: BTreeMap<u32, u32>,
    desired: usize,
    fallback_color_argb: u32,
    filter: bool,
) -> Vec<u32> {
    let populations = populations
        .into_iter()
        .map(|(argb, population)| (Argb::new(argb), population))
        .collect::<BTreeMap<_, _>>();
    core_ranked_suggestions(
        &populations,
        ScoreOptions {
            desired,
            fallback_color_argb: Argb::new(fallback_color_argb),
            filter,
        },
    )
    .into_iter()
    .map(Argb::to_u32)
    .collect()
}

#[pyfunction]
fn temperature_complement(argb: u32) -> u32 {
    let mut cache = TemperatureCache::new(Hct::from_argb(Argb::new(argb)));
    cache.complement().to_argb().to_u32()
}

#[pyfunction]
#[pyo3(signature = (argb, count = 5, divisions = 12))]
fn temperature_analogous(argb: u32, count: usize, divisions: usize) -> PyResult<Vec<u32>> {
    if count == 0 || divisions == 0 {
        return Err(PyValueError::new_err(
            "count and divisions must be non-zero",
        ));
    }

    let mut cache = TemperatureCache::new(Hct::from_argb(Argb::new(argb)));
    Ok(cache
        .analogous_colors(count, divisions)
        .into_iter()
        .map(|hct| hct.to_argb().to_u32())
        .collect())
}

#[pymodule]
fn material_color_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyArgb>()?;
    m.add_class::<PyHct>()?;
    m.add_class::<PyTonalPalette>()?;
    m.add_class::<PyDynamicScheme>()?;
    m.add_class::<PyVariant>()?;
    m.add_class::<PyDynamicColorRole>()?;
    m.add_function(wrap_pyfunction!(blend_harmonize, m)?)?;
    m.add_function(wrap_pyfunction!(blend_hct_hue, m)?)?;
    m.add_function(wrap_pyfunction!(quantize_celebi, m)?)?;
    m.add_function(wrap_pyfunction!(ranked_suggestions, m)?)?;
    m.add_function(wrap_pyfunction!(temperature_complement, m)?)?;
    m.add_function(wrap_pyfunction!(temperature_analogous, m)?)?;
    Ok(())
}
