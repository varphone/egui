//! A collection of easing functions for use in animations and transitions.

use std::f64::consts::PI;

const FRAC_2_PI_3: f64 = (2.0 * PI) / 3.0;
const FRAC_2_PI_4_5: f64 = (2.0 * PI) / 4.5;

/// An enum representing the different easing functions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(usize)]
pub enum Easing {
    // Sine easing functions
    /// Easing function for sine easing in
    SineIn,
    /// Easing function for sine easing out
    SineOut,
    /// Easing function for sine easing in and out
    SineInOut,

    // Quad easing functions
    /// Easing function for quadratic easing in
    QuadIn,
    /// Easing function for quadratic easing out
    QuadOut,
    /// Easing function for quadratic easing in and out
    QuadInOut,

    // Cubic easing functions
    /// Easing function for cubic easing in
    CubicIn,
    /// Easing function for cubic easing out
    CubicOut,
    /// Easing function for cubic easing in and out
    CubicInOut,

    // Quartic easing functions
    /// Easing function for quartic easing in
    QuartIn,
    /// Easing function for quartic easing out
    QuartOut,
    /// Easing function for quartic easing in and out
    QuartInOut,

    // Quintic easing functions
    /// Easing function for quintic easing in
    QuintIn,
    /// Easing function for quintic easing out
    QuintOut,
    /// Easing function for quintic easing in and out
    QuintInOut,

    // Exponential easing functions
    /// Easing function for exponential easing in
    ExpoIn,
    /// Easing function for exponential easing out
    ExpoOut,
    /// Easing function for exponential easing in and out
    ExpoInOut,

    // Circular easing functions
    CircIn,
    /// Easing function for circular easing out
    CircOut,
    /// Easing function for circular easing in and out
    CircInOut,

    // Back easing functions
    /// Easing function for back easing in
    BackIn,
    /// Easing function for back easing out
    BackOut,
    /// Easing function for back easing in and out
    BackInOut,

    // Elastic easing functions
    /// Easing function for elastic easing in
    ElasticIn,
    /// Easing function for elastic easing out
    ElasticOut,
    /// Easing function for elastic easing in and out
    ElasticInOut,

    // Bounce easing functions
    /// Easing function for bounce easing in
    BounceIn,
    /// Easing function for bounce easing out
    BounceOut,
    /// Easing function for bounce easing in and out
    BounceInOut,

    Max,
}

impl Easing {
    /// Returns the easing function for the given easing type.
    #[inline]
    pub fn apply_function(self) -> fn(f64) -> f64 {
        match self {
            Easing::SineIn => sine_in,
            Easing::SineOut => sine_out,
            Easing::SineInOut => sine_in_out,
            Easing::QuadIn => quad_in,
            Easing::QuadOut => quad_out,
            Easing::QuadInOut => quad_in_out,
            Easing::CubicIn => cubic_in,
            Easing::CubicOut => cubic_out,
            Easing::CubicInOut => cubic_in_out,
            Easing::QuartIn => quart_in,
            Easing::QuartOut => quart_out,
            Easing::QuartInOut => quart_in_out,
            Easing::QuintIn => quint_in,
            Easing::QuintOut => quint_out,
            Easing::QuintInOut => quint_in_out,
            Easing::ExpoIn => expo_in,
            Easing::ExpoOut => expo_out,
            Easing::ExpoInOut => expo_in_out,
            Easing::CircIn => circ_in,
            Easing::CircOut => circ_out,
            Easing::CircInOut => circ_in_out,
            Easing::BackIn => back_in,
            Easing::BackOut => back_out,
            Easing::BackInOut => back_in_out,
            Easing::ElasticIn => elastic_in,
            Easing::ElasticOut => elastic_out,
            Easing::ElasticInOut => elastic_in_out,
            Easing::BounceIn => bounce_in,
            Easing::BounceOut => bounce_out,
            Easing::BounceInOut => bounce_in_out,
            Easing::Max => unreachable!(),
        }
    }

    /// Returns the inverse easing function for the given easing type.
    #[inline]
    pub fn inverse_function(self) -> fn(f64) -> f64 {
        match self {
            Easing::SineIn => inverse_sine_in,
            Easing::SineOut => inverse_sine_out,
            Easing::SineInOut => inverse_sine_in_out,
            Easing::QuadIn => inverse_quad_in,
            Easing::QuadOut => inverse_quad_out,
            Easing::QuadInOut => inverse_quad_in_out,
            Easing::CubicIn => inverse_cubic_in,
            Easing::CubicOut => inverse_cubic_out,
            Easing::CubicInOut => inverse_cubic_in_out,
            Easing::QuartIn => inverse_quart_in,
            Easing::QuartOut => inverse_quart_out,
            Easing::QuartInOut => inverse_quart_in_out,
            Easing::QuintIn => inverse_quint_in,
            Easing::QuintOut => inverse_quint_out,
            Easing::QuintInOut => inverse_quint_in_out,
            Easing::ExpoIn => inverse_expo_in,
            Easing::ExpoOut => inverse_expo_out,
            Easing::ExpoInOut => inverse_expo_in_out,
            Easing::CircIn => inverse_circ_in,
            Easing::CircOut => inverse_circ_out,
            Easing::CircInOut => inverse_circ_in_out,
            Easing::BackIn => inverse_back_in,
            Easing::BackOut => inverse_back_out,
            Easing::BackInOut => inverse_back_in_out,
            Easing::ElasticIn => inverse_elastic_in,
            Easing::ElasticOut => inverse_elastic_out,
            Easing::ElasticInOut => inverse_elastic_in_out,
            Easing::BounceIn => inverse_bounce_in,
            Easing::BounceOut => inverse_bounce_out,
            Easing::BounceInOut => inverse_bounce_in_out,
            Easing::Max => unreachable!(),
        }
    }

    /// Returns the easing function for the given easing type.
    ///
    /// # Arguments
    ///
    /// * `t` - The time value to apply the easing function to. This should be a value between 0.0 and 1.0.
    ///
    /// # Example
    ///
    /// ```
    /// use emath::Easing;
    ///
    /// let easing = Easing::SineIn;
    /// let t = 0.5;
    /// let result = easing.apply(t);
    /// ```
    pub fn apply(self, t: f64) -> f64 {
        (self.apply_function())(t)
    }

    /// Returns the inverse easing function for the given easing type.
    ///
    /// # Arguments
    ///
    /// * `t` - The time value to apply the inverse easing function to. This should be a value between 0.0 and 1.0.
    ///
    /// # Example
    ///
    /// ```
    /// use emath::Easing;
    ///
    /// let easing = Easing::SineIn;
    /// let t = 0.5;
    /// let result = easing.inverse(t);
    /// ```
    pub fn inverse(self, t: f64) -> f64 {
        (self.inverse_function())(t)
    }

    /// Returns whether the given easing function is reversible.
    pub fn reversible(self) -> bool {
        match self {
            Easing::SineIn
            | Easing::SineOut
            | Easing::SineInOut
            | Easing::QuadIn
            | Easing::QuadOut
            | Easing::QuadInOut
            | Easing::CubicIn
            | Easing::CubicOut
            | Easing::CubicInOut
            | Easing::QuartIn
            | Easing::QuartOut
            | Easing::QuartInOut
            | Easing::QuintIn
            | Easing::QuintOut
            | Easing::QuintInOut
            | Easing::ExpoIn
            | Easing::ExpoOut
            | Easing::ExpoInOut
            | Easing::CircIn
            | Easing::CircOut
            | Easing::CircInOut => true,
            _ => false,
        }
    }

    /// Returns an iterator over all the easing functions.
    pub fn all() -> Easings {
        Easings::new()
    }
}

impl From<usize> for Easing {
    fn from(value: usize) -> Self {
        match value {
            0 => Easing::SineIn,
            1 => Easing::SineOut,
            2 => Easing::SineInOut,
            3 => Easing::QuadIn,
            4 => Easing::QuadOut,
            5 => Easing::QuadInOut,
            6 => Easing::CubicIn,
            7 => Easing::CubicOut,
            8 => Easing::CubicInOut,
            9 => Easing::QuartIn,
            10 => Easing::QuartOut,
            11 => Easing::QuartInOut,
            12 => Easing::QuintIn,
            13 => Easing::QuintOut,
            14 => Easing::QuintInOut,
            15 => Easing::ExpoIn,
            16 => Easing::ExpoOut,
            17 => Easing::ExpoInOut,
            18 => Easing::CircIn,
            19 => Easing::CircOut,
            20 => Easing::CircInOut,
            21 => Easing::BackIn,
            22 => Easing::BackOut,
            23 => Easing::BackInOut,
            24 => Easing::ElasticIn,
            25 => Easing::ElasticOut,
            26 => Easing::ElasticInOut,
            27 => Easing::BounceIn,
            28 => Easing::BounceOut,
            29 => Easing::BounceInOut,
            _ => Easing::Max,
        }
    }
}

pub struct Easings {
    current: Easing,
}

impl Easings {
    pub fn new() -> Self {
        Self {
            current: Easing::SineIn,
        }
    }
}

impl Iterator for Easings {
    type Item = Easing;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current != Easing::Max {
            let tmp = self.current;
            self.current = Easing::from(tmp as usize + 1);
            Some(tmp)
        } else {
            None
        }
    }
}

/// Returns the value of the "ease in" sine easing function.
///
/// This function models a motion that starts slow and then speeds up.
///
/// For a visual representation of this easing function, refer to: <https://easings.net/#easeInSine>
#[inline]
pub fn sine_in(x: f64) -> f64 {
    if x == 0.0 || x == 1.0 {
        x
    } else {
        1.0 - (x * PI / 2.0).cos()
    }
}

/// Returns the inverse value of the "ease in" sine easing function.
#[inline]
pub fn inverse_sine_in(x: f64) -> f64 {
    if x == 0.0 || x == 1.0 {
        x
    } else {
        2.0 * (1.0 - x).acos() / PI
    }
}

/// Returns the value of the "ease out" sine easing function.
///
/// This function models a motion that starts fast and then slows down.
///
/// For a visual representation of this easing function, refer to: <https://easings.net/#easeOutSine>
#[inline]
pub fn sine_out(x: f64) -> f64 {
    if x == 0.0 || x == 1.0 {
        x
    } else {
        (x * PI / 2.0).sin()
    }
}

/// Returns the inverse value of the "ease out" sine easing function.
#[inline]
pub fn inverse_sine_out(x: f64) -> f64 {
    if x == 0.0 || x == 1.0 {
        x
    } else {
        2.0 * x.asin() / PI
    }
}

/// Returns the value of the "ease in out" sine easing function.
///
/// This function models a motion that starts slow, speeds up, and then slows down again towards the end.
///
/// For a visual representation of this easing function, refer to: <https://easings.net/#easeInOutSine>
#[inline]
pub fn sine_in_out(x: f64) -> f64 {
    if x == 0.0 || x == 1.0 {
        x
    } else {
        -((x * PI).cos() - 1.0) / 2.0
    }
}

/// Returns the inverse value of the "ease in out" sine easing function.
#[inline]
pub fn inverse_sine_in_out(x: f64) -> f64 {
    if x == 0.0 || x == 1.0 {
        x
    } else {
        (1.0 - 2.0 * x).acos() / PI
    }
}

/// Returns the value of the "ease in" quadratic easing function.
///
/// This function models a motion that starts slow and then speeds up.
///
/// For a visual representation of this easing function, refer to: <https://easings.net/#easeInQuad>
#[inline]
pub fn quad_in(x: f64) -> f64 {
    x * x
}

/// Returns the inverse value of the "ease in" quadratic easing function.
#[inline]
pub fn inverse_quad_in(x: f64) -> f64 {
    x.sqrt()
}

/// Returns the value of the "ease out" quadratic easing function.
///
/// This function models a motion that starts fast and then slows down.
///
/// For a visual representation of this easing function, refer to: <https://easings.net/#easeOutQuad>
#[inline]
pub fn quad_out(x: f64) -> f64 {
    1.0 - (1.0 - x) * (1.0 - x)
}

/// Returns the inverse value of the "ease out" quadratic easing function.
#[inline]
pub fn inverse_quad_out(x: f64) -> f64 {
    1.0 - (1.0 - x).sqrt()
}

/// Returns the value of the "ease in out" quadratic easing function.
///
/// This function models a motion that starts slow, speeds up, and then slows down again towards the end.
///
/// For a visual representation of this easing function, refer to: <https://easings.net/#easeInOutQuad>
#[inline]
pub fn quad_in_out(x: f64) -> f64 {
    if x < 0.5 {
        2.0 * x * x
    } else {
        let v = -2.0 * x + 2.0;
        1.0 - v * v / 2.0
    }
}

/// Returns the inverse value of the "ease in out" quadratic easing function.
#[inline]
pub fn inverse_quad_in_out(x: f64) -> f64 {
    if x < 0.5 {
        (x / 2.0).sqrt()
    } else {
        1.0 - ((1.0 - x) / 2.0).sqrt()
    }
}

/// Returns the value of the "ease in" cubic easing function.
///
/// This function models a motion that starts slow and then speeds up.
///
/// For a visual representation of this easing function, refer to: <https://easings.net/#easeInCubic>
#[inline]
pub fn cubic_in(x: f64) -> f64 {
    x * x * x
}

/// Returns the inverse value of the "ease in" cubic easing function.
#[inline]
pub fn inverse_cubic_in(x: f64) -> f64 {
    x.cbrt()
}

/// Returns the value of the "ease out" cubic easing function.
///
/// This function models a motion that starts fast and then slows down.
///
/// For a visual representation of this easing function, refer to: <https://easings.net/#easeOutCubic>
#[inline]
pub fn cubic_out(x: f64) -> f64 {
    1.0 - (1.0 - x) * (1.0 - x) * (1.0 - x)
}

/// Returns the inverse value of the "ease out" cubic easing function.
#[inline]
pub fn inverse_cubic_out(x: f64) -> f64 {
    1.0 - (1.0 - x).cbrt()
}

/// Returns the value of the "ease in out" cubic easing function.
///
/// This function models a motion that starts slow, speeds up, and then slows down again towards the end.
///
/// For a visual representation of this easing function, refer to: <https://easings.net/#easeInOutCubic>
#[inline]
pub fn cubic_in_out(x: f64) -> f64 {
    if x == 0.0 || x == 1.0 {
        x
    } else if x < 0.5 {
        4.0 * x * x * x
    } else {
        1.0 - (-2.0 * x + 2.0).powi(3) / 2.0
    }
}

/// Returns the inverse value of the "ease in out" cubic easing function.
#[inline]
pub fn inverse_cubic_in_out(x: f64) -> f64 {
    if x == 0.0 || x == 1.0 {
        x
    } else if x < 0.5 {
        (x / 4.0).cbrt()
    } else {
        1.0 - ((2.0 * (1.0 - x)).cbrt() / 2.0)
    }
}

/// Returns the value of the "ease in" quartic easing function.
///
/// This function models a motion that starts slow and then speeds up.
///
/// For a visual representation of this easing function, refer to: <https://easings.net/#easeInQuart>
#[inline]
pub fn quart_in(x: f64) -> f64 {
    x * x * x * x
}

/// Returns the inverse value of the "ease in" quartic easing function.
#[inline]
pub fn inverse_quart_in(x: f64) -> f64 {
    x.powf(1.0 / 4.0)
}

/// Returns the value of the "ease out" quartic easing function.
///
/// This function models a motion that starts fast and then slows down.
///
/// For a visual representation of this easing function, refer to: <https://easings.net/#easeOutQuart>
#[inline]
pub fn quart_out(x: f64) -> f64 {
    1.0 - (1.0 - x).powi(4)
}

/// Returns the inverse value of the "ease out" quartic easing function.
#[inline]
pub fn inverse_quart_out(x: f64) -> f64 {
    1.0 - (1.0 - x).powf(1.0 / 4.0)
}

/// Returns the value of the "ease in out" quartic easing function.
///
/// This function models a motion that starts slow, speeds up, and then slows down again towards the end.
///
/// For a visual representation of this easing function, refer to: <https://easings.net/#easeInOutQuart>
#[inline]
pub fn quart_in_out(x: f64) -> f64 {
    if x < 0.5 {
        8.0 * x * x * x * x
    } else {
        1.0 - (-2.0 * x + 2.0).powi(4) / 2.0
    }
}

/// Returns the inverse value of the "ease in out" quartic easing function.
#[inline]
pub fn inverse_quart_in_out(x: f64) -> f64 {
    if x == 0.0 || x == 1.0 {
        x
    } else if x < 0.5 {
        (x / 8.0).powf(1.0 / 4.0)
    } else {
        1.0 - ((2.0 * (1.0 - x)).powf(1.0 / 4.0) / 2.0)
    }
}

/// Returns the value of the "ease in" quintic easing function.
///
/// This function models a motion that starts slow and then speeds up.
///
/// For a visual representation of this easing function, refer to: <https://easings.net/#easeInQuint>
#[inline]
pub fn quint_in(x: f64) -> f64 {
    x * x * x * x * x
}

/// Returns the inverse value of the "ease in" quintic easing function.
#[inline]
pub fn inverse_quint_in(x: f64) -> f64 {
    x.powf(1.0 / 5.0)
}

/// Returns the value of the "ease out" quintic easing function.
///
/// This function models a motion that starts fast and then slows down.
///
/// For a visual representation of this easing function, refer to: <https://easings.net/#easeOutQuint>
#[inline]
pub fn quint_out(x: f64) -> f64 {
    1.0 - (1.0 - x).powi(5)
}

/// Returns the inverse value of the "ease out" quintic easing function.
#[inline]
pub fn inverse_quint_out(x: f64) -> f64 {
    1.0 - (1.0 - x).powf(1.0 / 5.0)
}

/// Returns the value of the "ease in out" quintic easing function.
///
/// This function models a motion that starts slow, speeds up, and then slows down again towards the end.
///
/// For a visual representation of this easing function, refer to: <https://easings.net/#easeInOutQuint>
#[inline]
pub fn quint_in_out(x: f64) -> f64 {
    if x < 0.5 {
        16.0 * x * x * x * x * x
    } else {
        1.0 - (-2.0 * x + 2.0).powi(5) / 2.0
    }
}

/// Returns the inverse value of the "ease in out" quintic easing function.
#[inline]
pub fn inverse_quint_in_out(x: f64) -> f64 {
    if x == 0.0 || x == 1.0 {
        x
    } else if x < 0.5 {
        (x / 16.0).powf(1.0 / 5.0)
    } else {
        1.0 - ((2.0 * (1.0 - x)).powf(1.0 / 5.0) / 2.0)
    }
}

/// Returns the value of the "ease in" exponential easing function.
///
/// This function models a motion that starts slow and then speeds up.
///
/// For a visual representation of this easing function, refer to: <https://easings.net/#easeInExpo>
#[inline]
pub fn expo_in(x: f64) -> f64 {
    if x == 0.0 || x == 1.0 {
        x
    } else {
        2.0f64.powf(10.0 * x - 10.0)
    }
}

/// Returns the inverse value of the "ease in" exponential easing function.
#[inline]
pub fn inverse_expo_in(x: f64) -> f64 {
    if x == 0.0 || x == 1.0 {
        x
    } else {
        (x.log2() / 10.0) + 1.0
    }
}

/// Returns the value of the "ease out" exponential easing function.
///
/// This function models a motion that starts fast and then slows down.
///
/// For a visual representation of this easing function, refer to: <https://easings.net/#easeOutExpo>
#[inline]
pub fn expo_out(x: f64) -> f64 {
    if x == 0.0 || x == 1.0 {
        x
    } else {
        1.0 - 2.0f64.powf(-10.0 * x)
    }
}

/// Returns the inverse value of the "ease out" exponential easing function.
#[inline]
pub fn inverse_expo_out(x: f64) -> f64 {
    if x == 0.0 || x == 1.0 {
        x
    } else {
        -((1.0 - x).log2() / 10.0)
    }
}

/// Returns the value of the "ease in out" exponential easing function.
///
/// This function models a motion that starts slow, speeds up, and then slows down again towards the end.
///
/// For a visual representation of this easing function, refer to: <https://easings.net/#easeInOutExpo>
#[inline]
pub fn expo_in_out(x: f64) -> f64 {
    if x == 0.0 || x == 1.0 {
        x
    } else if x < 0.5 {
        2.0f64.powf(20.0 * x - 10.0) / 2.0
    } else {
        (2.0 - 2.0f64.powf(-20.0 * x + 10.0)) / 2.0
    }
}

/// Returns the inverse value of the "ease in out" exponential easing function.
#[inline]
pub fn inverse_expo_in_out(x: f64) -> f64 {
    if x == 0.0 || x == 1.0 {
        x
    } else if x < 0.5 {
        ((2.0 * x).log2() + 10.0) / 20.0
    } else {
        1.0 - ((2.0 - 2.0 * x).log2() + 10.0) / 20.0
    }
}

/// Returns the value of the "ease in" circular easing function.
///
/// This function models a motion that starts slow and then speeds up.
///
/// For a visual representation of this easing function, refer to: <https://easings.net/#easeInCirc>
#[inline]
pub fn circ_in(x: f64) -> f64 {
    1.0 - (1.0 - x.powi(2)).sqrt()
}

/// Returns the inverse value of the "ease in" circular easing function.
#[inline]
pub fn inverse_circ_in(x: f64) -> f64 {
    (1.0 - (1.0 - x).powi(2)).sqrt()
}

/// Returns the value of the "ease out" circular easing function.
///
/// This function models a motion that starts fast and then slows down.
///
/// For a visual representation of this easing function, refer to: <https://easings.net/#easeOutCirc>
#[inline]
pub fn circ_out(x: f64) -> f64 {
    (1.0 - (x - 1.0).powi(2)).sqrt()
}

/// Returns the inverse value of the "ease out" circular easing function.
#[inline]
pub fn inverse_circ_out(y: f64) -> f64 {
    1.0 - (1.0 - y.powi(2)).sqrt()
}

/// Returns the value of the "ease in out" circular easing function.
///
/// This function models a motion that starts slow, speeds up, and then slows down again towards the end.
///
/// For a visual representation of this easing function, refer to: <https://easings.net/#easeInOutCirc>
#[inline]
pub fn circ_in_out(x: f64) -> f64 {
    if x < 0.5 {
        (1.0 - (1.0 - (2.0 * x).powi(2)).sqrt()) / 2.0
    } else {
        ((1.0 - (-2.0 * x + 2.0).powi(2)).sqrt() + 1.0) / 2.0
    }
}

/// Returns the inverse value of the "ease in out" circular easing function.
#[inline]
pub fn inverse_circ_in_out(y: f64) -> f64 {
    if y < 0.5 {
        (1.0 - (1.0 - 2.0 * y).powi(2)).sqrt() / 2.0
    } else {
        1.0 - (1.0 - (2.0 * y - 1.0).powi(2)).sqrt() / 2.0
    }
}

/// Returns the value of the "ease in" back easing function.
///
/// This function models a motion that starts slow and then speeds up with an added back effect at the start of the motion. It's similar to a spring that is being stretched.
///
/// For a visual representation of this easing function, refer to: <https://easings.net/#easeInBack>
#[inline]
pub fn back_in(x: f64) -> f64 {
    const C1: f64 = 1.70158;
    const C3: f64 = C1 + 1.0;

    if x == 0.0 || x == 1.0 {
        x
    } else {
        C3 * x * x * x - C1 * x * x
    }
}

/// Returns the inverse value of the "ease in" back easing function.
///
/// # Note
///
/// This function is not actually the inverse of the back easing function.
/// Because the back easing function is not a bijection, it is not possible to define an inverse function.
/// It is simply the identity function.
#[inline]
pub fn inverse_back_in(y: f64) -> f64 {
    unimplemented!("back is irreversible")
}

/// Returns the value of the "ease out" back easing function.
///
/// This function models a motion that starts fast and then slows down towards the end, with an added back effect at the end of the motion. It's similar to a spring that is being compressed and then released.
///
/// For a visual representation of this easing function, refer to: <https://easings.net/#easeOutBack>
pub fn back_out(x: f64) -> f64 {
    const C1: f64 = 1.70158;
    const C3: f64 = C1 + 1.0;

    if x == 0.0 || x == 1.0 {
        x
    } else {
        1.0 + C3 * ((x - 1.0).powi(3)) + C1 * ((x - 1.0).powi(2))
    }
}

/// Returns the inverse value of the "ease out" back easing function.
///
/// # Note
///
/// This function is not actually the inverse of the back easing function.
/// Because the back easing function is not a bijection, it is not possible to define an inverse function.
/// It is simply the identity function.
#[inline]
pub fn inverse_back_out(x: f64) -> f64 {
    unimplemented!("back is irreversible")
}

/// Returns the value of the "ease in out" back easing function.
///
/// This function models a motion that starts slow, speeds up, and then slows down again towards the end, with an added back effect both at the start and the end of the motion. It's similar to a spring that is being stretched and then released.
///
/// For a visual representation of this easing function, refer to: <https://easings.net/#easeInOutBack>
#[inline]
pub fn back_in_out(x: f64) -> f64 {
    const C1: f64 = 1.70158;
    const C2: f64 = C1 * 1.525;

    if x < 0.5 {
        ((2.0 * x).powi(2) * ((C2 + 1.0) * 2.0 * x - C2)) / 2.0
    } else {
        ((2.0 * x - 2.0).powi(2) * ((C2 + 1.0) * (x * 2.0 - 2.0) + C2) + 2.0) / 2.0
    }
}

/// Returns the inverse value of the "ease in out" back easing function.
///
/// # Note
///
/// This function is not actually the inverse of the back easing function.
/// Because the back easing function is not a bijection, it is not possible to define an inverse function.
/// It is simply the identity function.
#[inline]
pub fn inverse_back_in_out(x: f64) -> f64 {
    unimplemented!("back is irreversible")
}

/// Returns the value of the "ease in" elastic easing function.
///
/// This function models a motion that starts slow and then speeds up with an added elastic effect at the start of the motion. It's similar to a spring that is being stretched.
///
/// For a visual representation of this easing function, refer to: <https://easings.net/#easeInElastic>
#[inline]
fn elastic_in(x: f64) -> f64 {
    let x2 = x * x;
    x2 * x2 * (x * PI * 4.5).sin()
}

/// Returns the inverse of the elastic easing in function.
///
/// # Note
///
/// This function is not actually the inverse of the elastic easing function.
/// Because the elastic easing function is not a bijection, it is not possible to define an inverse function.
/// It is simply the identity function.
#[inline]
pub fn inverse_elastic_in(x: f64) -> f64 {
    unimplemented!("elastic is irreversible")
}

/// Returns the value of the "ease out" elastic easing function.
///
/// This function models a motion that starts fast and then slows down towards the end, with an added elastic effect at the end of the motion. It's similar to a spring that is being compressed and then released.
///
/// For a visual representation of this easing function, refer to: <https://easings.net/#easeOutElastic>
#[inline]
pub fn elastic_out(x: f64) -> f64 {
    let x2 = (x - 1.0) * (x - 1.0);
    1.0 - x2 * x2 * (x * PI * 4.5).cos()
}

/// Returns the inverse of the elastic easing out function.
///
/// # Note
///
/// This function is not actually the inverse of the elastic easing function.
/// Because the elastic easing function is not a bijection, it is not possible to define an inverse function.
/// It is simply the identity function.
#[inline]
pub fn inverse_elastic_out(x: f64) -> f64 {
    unimplemented!("elastic is irreversible")
}

/// Returns the value of the "ease in out" elastic easing function.
///
/// This function models a motion that starts slow, speeds up, and then slows down again towards the end, with an added elastic effect both at the start and the end of the motion. It's similar to a spring that is being stretched and then released.
///
/// For a visual representation of this easing function, refer to: <https://easings.net/#easeInOutElastic>
#[inline]
pub fn elastic_in_out(x: f64) -> f64 {
    if x < 0.45 {
        let x2 = x * x;
        8.0 * x2 * x2 * (x * PI * 9.0).sin()
    } else if x < 0.55 {
        0.5 + 0.75 * (x * PI * 4.0).sin()
    } else {
        let x2 = (x - 1.0) * (x - 1.0);
        1.0 - 8.0 * x2 * x2 * (x * PI * 9.0).sin()
    }
}

/// Returns the inverse value of the "ease in out" elastic easing function.
///
/// # Note
///
/// This function is not actually the inverse of the elastic easing function.
/// Because the elastic easing function is not a bijection, it is not possible to define an inverse function.
/// It is simply the identity function.
#[inline]
pub fn inverse_elastic_in_out(x: f64) -> f64 {
    unimplemented!("elastic is irreversible")
}

/// Returns the value of the "ease in" bounce easing function.
///
/// This function models a motion that starts slow and then speeds up with an added bounce effect at the start of the motion. It's similar to a ball that is being dropped.
///
/// For a visual representation of this easing function, refer to: <https://easings.net/#easeInBounce>
#[inline]
pub fn bounce_in(x: f64) -> f64 {
    2.0f64.powf(6.0 * (x - 1.0)) * (x * PI * 3.5).sin().abs()
}

/// Returns the inverse value of the "ease in" bounce easing function.
///
/// # Note
///
/// This function is not actually the inverse of the bounce easing function.
/// Because the bounce easing function is not a bijection, it is not possible to define an inverse function.
/// It is simply the identity function.
#[inline]
pub fn inverse_bounce_in(x: f64) -> f64 {
    unimplemented!("bounce is irreversible")
}

/// Returns the value of the "ease out" bounce easing function.
///
/// This function models a motion that starts fast and then slows down towards the end, with an added bounce effect at the end of the motion. It's similar to a ball that is being thrown at the ground.
///
/// For a visual representation of this easing function, refer to: <https://easings.net/#easeOutBounce>
#[inline]
pub fn bounce_out(x: f64) -> f64 {
    1.0 - 2.0f64.powf(-6.0 * x) * (x * PI * 3.5).cos().abs()
}

/// Returns the inverse value of the "ease out" bounce easing function.
///
/// # Note
///
/// This function is not actually the inverse of the bounce easing function.
/// Because the bounce easing function is not a bijection, it is not possible to define an inverse function.
/// It is simply the identity function.
#[inline]
pub fn inverse_bounce_out(x: f64) -> f64 {
    unimplemented!("bounce is irreversible")
}

/// Returns the value of the "ease in out" bounce easing function.
///
/// This function models a motion that starts slow, speeds up, and then slows down again towards the end, with an added bounce effect both at the start and the end of the motion. It's similar to a ball that is being dropped and then bounces back up.
///
/// For a visual representation of this easing function, refer to: <https://easings.net/#easeInOutBounce>
#[inline]
pub fn bounce_in_out(x: f64) -> f64 {
    if x < 0.5 {
        8.0 * 2.0f64.powf(8.0 * (x - 1.0)) * (x * PI * 7.0).sin().abs()
    } else {
        1.0 - 8.0 * 2.0f64.powf(-8.0 * x) * (x * PI * 7.0).sin().abs()
    }
}

/// Returns the inverse value of the "ease in out" bounce easing function.
///
/// # Note
///
/// This function is not actually the inverse of the bounce easing function.
/// Because the bounce easing function is not a bijection, it is not possible to define an inverse function.
/// It is simply the identity function.
#[inline]
pub fn inverse_bounce_in_out(x: f64) -> f64 {
    unimplemented!("bounce is irreversible")
}

macro_rules! assert_approx_eq {
    ($a:expr, $b:expr) => {{
        let eps = 1.0e-6;
        let (a, b) = (&$a, &$b);
        assert!(
            (*a - *b).abs() < eps,
            "assertion failed: `(left !== right)` \n  left: `{:?}`\n right: `{:?}`\n expect diff: `{:?}`, real diff: `{:?}`",
            *a,
            *b,
            eps,
            (*a - *b).abs()
        );
    }};
    ($a:expr, $b:expr, $eps:expr) => {{
        let (a, b) = (&$a, &$b);
        let eps = $eps;
        assert!(
            (*a - *b).abs() < eps,
            "assertion failed: `(left !== right)` \n  left: `{:?}`\n right: `{:?}`\n expect diff: `{:?}`, real diff: `{:?}`",
            *a,
            *b,
            eps,
            (*a - *b).abs()
        );
    }};
}

#[test]
fn test_sine_in() {
    assert_eq!(sine_in(0.0), 0.0);
    assert_approx_eq!(sine_in(0.2), 0.048943);
    assert_approx_eq!(sine_in(0.4), 0.190983);
    assert_approx_eq!(sine_in(0.5), 0.292893);
    assert_approx_eq!(sine_in(0.6), 0.412214);
    assert_approx_eq!(sine_in(0.8), 0.690983);
    assert_eq!(sine_in(1.0), 1.0);
}

#[test]
fn test_inverse_sine_in() {
    assert_eq!(inverse_sine_in(0.0), 0.0);
    assert_approx_eq!(inverse_sine_in(0.048943), 0.2);
    assert_approx_eq!(inverse_sine_in(0.190983), 0.4);
    assert_approx_eq!(inverse_sine_in(0.292893), 0.5);
    assert_approx_eq!(inverse_sine_in(0.412214), 0.6);
    assert_approx_eq!(inverse_sine_in(0.690983), 0.8);
    assert_eq!(inverse_sine_in(1.0), 1.0);
}

#[test]
fn test_sine_out() {
    assert_eq!(sine_out(0.0), 0.0);
    assert_approx_eq!(sine_out(0.2), 0.309016);
    assert_approx_eq!(sine_out(0.4), 0.587785);
    assert_approx_eq!(sine_out(0.5), 0.707106);
    assert_approx_eq!(sine_out(0.6), 0.809016);
    assert_approx_eq!(sine_out(0.8), 0.951056);
    assert_eq!(sine_out(1.0), 1.0);
}

#[test]
fn test_inverse_sine_out() {
    assert_eq!(inverse_sine_out(0.0), 0.0);
    assert_approx_eq!(inverse_sine_out(0.309016), 0.2);
    assert_approx_eq!(inverse_sine_out(0.587785), 0.4);
    assert_approx_eq!(inverse_sine_out(0.707106), 0.5);
    assert_approx_eq!(inverse_sine_out(0.809016), 0.599998);
    assert_approx_eq!(inverse_sine_out(0.951056), 0.799998);
    assert_eq!(inverse_sine_out(1.0), 1.0);
}

#[test]
fn test_sine_in_out() {
    assert_eq!(sine_in_out(0.0), 0.0);
    assert_approx_eq!(sine_in_out(0.2), 0.095491);
    assert_approx_eq!(sine_in_out(0.4), 0.345491);
    assert_approx_eq!(sine_in_out(0.5), 0.5);
    assert_approx_eq!(sine_in_out(0.6), 0.654508);
    assert_approx_eq!(sine_in_out(0.8), 0.904508);
    assert_eq!(sine_in_out(1.0), 1.0);
}

#[test]
fn test_inverse_sine_in_out() {
    assert_eq!(inverse_sine_in_out(0.0), 0.0);
    assert_approx_eq!(inverse_sine_in_out(0.095491), 0.2);
    assert_approx_eq!(inverse_sine_in_out(0.345491), 0.4);
    assert_approx_eq!(inverse_sine_in_out(0.5), 0.5);
    assert_approx_eq!(inverse_sine_in_out(0.654508), 0.6);
    assert_approx_eq!(inverse_sine_in_out(0.904508), 0.8);
    assert_eq!(inverse_sine_in_out(1.0), 1.0);
}

#[test]
fn test_quad_in() {
    assert_eq!(quad_in(0.0), 0.0);
    assert_approx_eq!(quad_in(0.2), 0.04);
    assert_approx_eq!(quad_in(0.4), 0.16);
    assert_approx_eq!(quad_in(0.5), 0.25);
    assert_approx_eq!(quad_in(0.6), 0.36);
    assert_approx_eq!(quad_in(0.8), 0.64);
    assert_eq!(quad_in(1.0), 1.0);
}

#[test]
fn test_inverse_quad_in() {
    assert_eq!(inverse_quad_in(0.0), 0.0);
    assert_approx_eq!(inverse_quad_in(0.04), 0.2);
    assert_approx_eq!(inverse_quad_in(0.16), 0.4);
    assert_approx_eq!(inverse_quad_in(0.25), 0.5);
    assert_approx_eq!(inverse_quad_in(0.36), 0.6);
    assert_approx_eq!(inverse_quad_in(0.64), 0.8);
    assert_eq!(inverse_quad_in(1.0), 1.0);
}

#[test]
fn test_quad_out() {
    assert_eq!(quad_out(0.0), 0.0);
    assert_approx_eq!(quad_out(0.2), 0.36);
    assert_approx_eq!(quad_out(0.4), 0.64);
    assert_approx_eq!(quad_out(0.5), 0.75);
    assert_approx_eq!(quad_out(0.6), 0.84);
    assert_approx_eq!(quad_out(0.8), 0.96);
    assert_eq!(quad_out(1.0), 1.0);
}

#[test]
fn test_inverse_quad_out() {
    assert_eq!(inverse_quad_out(0.0), 0.0);
    assert_approx_eq!(inverse_quad_out(0.36), 0.2);
    assert_approx_eq!(inverse_quad_out(0.64), 0.4);
    assert_approx_eq!(inverse_quad_out(0.75), 0.5);
    assert_approx_eq!(inverse_quad_out(0.84), 0.6);
    assert_approx_eq!(inverse_quad_out(0.96), 0.8);
    assert_eq!(inverse_quad_out(1.0), 1.0);
}

#[test]
fn test_quad_in_out() {
    assert_eq!(quad_in_out(0.0), 0.0);
    assert_approx_eq!(quad_in_out(0.2), 0.08);
    assert_approx_eq!(quad_in_out(0.4), 0.32);
    assert_approx_eq!(quad_in_out(0.5), 0.5);
    assert_approx_eq!(quad_in_out(0.6), 0.68);
    assert_approx_eq!(quad_in_out(0.8), 0.92);
    assert_eq!(quad_in_out(1.0), 1.0);
}

#[test]
fn test_inverse_quad_in_out() {
    assert_eq!(inverse_quad_in_out(0.0), 0.0);
    assert_approx_eq!(inverse_quad_in_out(0.08), 0.2);
    assert_approx_eq!(inverse_quad_in_out(0.32), 0.4);
    assert_approx_eq!(inverse_quad_in_out(0.5), 0.5);
    assert_approx_eq!(inverse_quad_in_out(0.68), 0.6);
    assert_approx_eq!(inverse_quad_in_out(0.92), 0.8);
    assert_eq!(inverse_quad_in_out(1.0), 1.0);
}

#[test]
fn test_cubic_in() {
    assert_eq!(cubic_in(0.0), 0.0);
    assert_approx_eq!(cubic_in(0.2), 0.008);
    assert_approx_eq!(cubic_in(0.4), 0.064);
    assert_approx_eq!(cubic_in(0.5), 0.125);
    assert_approx_eq!(cubic_in(0.6), 0.216);
    assert_approx_eq!(cubic_in(0.8), 0.512);
    assert_eq!(cubic_in(1.0), 1.0);
}

#[test]
fn test_inverse_cubic_in() {
    assert_eq!(inverse_cubic_in(0.0), 0.0);
    assert_approx_eq!(inverse_cubic_in(0.008), 0.2);
    assert_approx_eq!(inverse_cubic_in(0.064), 0.4);
    assert_approx_eq!(inverse_cubic_in(0.125), 0.5);
    assert_approx_eq!(inverse_cubic_in(0.216), 0.6);
    assert_approx_eq!(inverse_cubic_in(0.512), 0.8);
    assert_eq!(inverse_cubic_in(1.0), 1.0);
}

#[test]
fn test_cubic_out() {
    assert_eq!(cubic_out(0.0), 0.0);
    assert_approx_eq!(cubic_out(0.2), 0.488);
    assert_approx_eq!(cubic_out(0.4), 0.784);
    assert_approx_eq!(cubic_out(0.5), 0.875);
    assert_approx_eq!(cubic_out(0.6), 0.936);
    assert_approx_eq!(cubic_out(0.8), 0.992);
    assert_eq!(cubic_out(1.0), 1.0);
}

#[test]
fn test_inverse_cubic_out() {
    assert_eq!(inverse_cubic_out(0.0), 0.0);
    assert_approx_eq!(inverse_cubic_out(0.488), 0.2);
    assert_approx_eq!(inverse_cubic_out(0.784), 0.4);
    assert_approx_eq!(inverse_cubic_out(0.875), 0.5);
    assert_approx_eq!(inverse_cubic_out(0.936), 0.6);
    assert_approx_eq!(inverse_cubic_out(0.992), 0.8);
    assert_eq!(inverse_cubic_out(1.0), 1.0);
}

#[test]
fn test_cubic_in_out() {
    assert_eq!(cubic_in_out(0.0), 0.0);
    assert_approx_eq!(cubic_in_out(0.2), 0.032);
    assert_approx_eq!(cubic_in_out(0.4), 0.256);
    assert_approx_eq!(cubic_in_out(0.5), 0.5);
    assert_approx_eq!(cubic_in_out(0.6), 0.744);
    assert_approx_eq!(cubic_in_out(0.8), 0.968);
    assert_eq!(cubic_in_out(1.0), 1.0);
}

#[test]
fn test_inverse_cubic_in_out() {
    assert_eq!(inverse_cubic_in_out(0.0), 0.0);
    assert_approx_eq!(inverse_cubic_in_out(0.032), 0.2);
    assert_approx_eq!(inverse_cubic_in_out(0.256), 0.4);
    assert_approx_eq!(inverse_cubic_in_out(0.5), 0.5);
    assert_approx_eq!(inverse_cubic_in_out(0.744), 0.6);
    assert_approx_eq!(inverse_cubic_in_out(0.968), 0.8);
    assert_eq!(inverse_cubic_in_out(1.0), 1.0);
}

#[test]
fn test_quart_in() {
    assert_eq!(quart_in(0.0), 0.0);
    assert_approx_eq!(quart_in(0.2), 0.0016);
    assert_approx_eq!(quart_in(0.4), 0.0256);
    assert_approx_eq!(quart_in(0.5), 0.0625);
    assert_approx_eq!(quart_in(0.6), 0.1296);
    assert_approx_eq!(quart_in(0.8), 0.4096);
    assert_eq!(quart_in(1.0), 1.0);
}

#[test]
fn test_inverse_quart_in() {
    assert_eq!(inverse_quart_in(0.0), 0.0);
    assert_approx_eq!(inverse_quart_in(0.0016), 0.2);
    assert_approx_eq!(inverse_quart_in(0.0256), 0.4);
    assert_approx_eq!(inverse_quart_in(0.0625), 0.5);
    assert_approx_eq!(inverse_quart_in(0.1296), 0.6);
    assert_approx_eq!(inverse_quart_in(0.4096), 0.8);
    assert_eq!(inverse_quart_in(1.0), 1.0);
}

#[test]
fn test_quart_out() {
    assert_eq!(quart_out(0.0), 0.0);
    assert_approx_eq!(quart_out(0.2), 0.5904);
    assert_approx_eq!(quart_out(0.4), 0.8704);
    assert_approx_eq!(quart_out(0.5), 0.9375);
    assert_approx_eq!(quart_out(0.6), 0.9744);
    assert_approx_eq!(quart_out(0.8), 0.9984);
    assert_eq!(quart_out(1.0), 1.0);
}

#[test]
fn test_inverse_quart_out() {
    assert_eq!(inverse_quart_out(0.0), 0.0);
    assert_approx_eq!(inverse_quart_out(0.5904), 0.2);
    assert_approx_eq!(inverse_quart_out(0.8704), 0.4);
    assert_approx_eq!(inverse_quart_out(0.9375), 0.5);
    assert_approx_eq!(inverse_quart_out(0.9744), 0.6);
    assert_approx_eq!(inverse_quart_out(0.9984), 0.8);
    assert_eq!(inverse_quart_out(1.0), 1.0);
}

#[test]
fn test_quart_in_out() {
    assert_eq!(quart_in_out(0.0), 0.0);
    assert_approx_eq!(quart_in_out(0.2), 0.0128);
    assert_approx_eq!(quart_in_out(0.4), 0.2048);
    assert_approx_eq!(quart_in_out(0.5), 0.5);
    assert_approx_eq!(quart_in_out(0.6), 0.7952);
    assert_approx_eq!(quart_in_out(0.8), 0.9872);
    assert_eq!(quart_in_out(1.0), 1.0);
}

#[test]
fn test_inverse_quart_in_out() {
    assert_eq!(inverse_quart_in_out(0.0), 0.0);
    assert_approx_eq!(inverse_quart_in_out(0.0128), 0.2);
    assert_approx_eq!(inverse_quart_in_out(0.2048), 0.4);
    assert_approx_eq!(inverse_quart_in_out(0.5), 0.5);
    assert_approx_eq!(inverse_quart_in_out(0.7952), 0.6);
    assert_approx_eq!(inverse_quart_in_out(0.9872), 0.8);
    assert_eq!(inverse_quart_in_out(1.0), 1.0);
}

#[test]
fn test_quint_in() {
    assert_eq!(quint_in(0.0), 0.0);
    assert_approx_eq!(quint_in(0.2), 0.00032);
    assert_approx_eq!(quint_in(0.4), 0.01024);
    assert_approx_eq!(quint_in(0.5), 0.03125);
    assert_approx_eq!(quint_in(0.6), 0.07776);
    assert_approx_eq!(quint_in(0.8), 0.32768);
    assert_eq!(quint_in(1.0), 1.0);
}

#[test]
fn test_inverse_quint_in() {
    assert_eq!(inverse_quint_in(0.0), 0.0);
    assert_approx_eq!(inverse_quint_in(0.00032), 0.2);
    assert_approx_eq!(inverse_quint_in(0.01024), 0.4);
    assert_approx_eq!(inverse_quint_in(0.03125), 0.5);
    assert_approx_eq!(inverse_quint_in(0.07776), 0.6);
    assert_approx_eq!(inverse_quint_in(0.32768), 0.8);
    assert_eq!(inverse_quint_in(1.0), 1.0);
}

#[test]
fn test_quint_out() {
    assert_eq!(quint_out(0.0), 0.0);
    assert_approx_eq!(quint_out(0.2), 0.67232);
    assert_approx_eq!(quint_out(0.4), 0.92224);
    assert_approx_eq!(quint_out(0.5), 0.96875);
    assert_approx_eq!(quint_out(0.6), 0.98976);
    assert_approx_eq!(quint_out(0.8), 0.99968);
    assert_eq!(quint_out(1.0), 1.0);
}

#[test]
fn test_inverse_quint_out() {
    assert_eq!(inverse_quint_out(0.0), 0.0);
    assert_approx_eq!(inverse_quint_out(0.67232), 0.2);
    assert_approx_eq!(inverse_quint_out(0.92224), 0.4);
    assert_approx_eq!(inverse_quint_out(0.96875), 0.5);
    assert_approx_eq!(inverse_quint_out(0.98976), 0.6);
    assert_approx_eq!(inverse_quint_out(0.99968), 0.8);
    assert_eq!(inverse_quint_out(1.0), 1.0);
}

#[test]
fn test_quint_in_out() {
    assert_eq!(quint_in_out(0.0), 0.0);
    assert_approx_eq!(quint_in_out(0.2), 0.00512);
    assert_approx_eq!(quint_in_out(0.4), 0.16384);
    assert_approx_eq!(quint_in_out(0.5), 0.5);
    assert_approx_eq!(quint_in_out(0.6), 0.83616);
    assert_approx_eq!(quint_in_out(0.8), 0.99488);
    assert_eq!(quint_in_out(1.0), 1.0);
}

#[test]
fn test_inverse_quint_in_out() {
    assert_eq!(inverse_quint_in_out(0.0), 0.0);
    assert_approx_eq!(inverse_quint_in_out(0.00512), 0.2);
    assert_approx_eq!(inverse_quint_in_out(0.16384), 0.4);
    assert_approx_eq!(inverse_quint_in_out(0.5), 0.5);
    assert_approx_eq!(inverse_quint_in_out(0.83616), 0.6);
    assert_approx_eq!(inverse_quint_in_out(0.99488), 0.8);
    assert_eq!(inverse_quint_in_out(1.0), 1.0);
}

#[test]
fn test_expo_in() {
    assert_eq!(expo_in(0.0), 0.0);
    assert_approx_eq!(expo_in(0.2), 0.003906);
    assert_approx_eq!(expo_in(0.4), 0.015625);
    assert_approx_eq!(expo_in(0.5), 0.03125);
    assert_approx_eq!(expo_in(0.6), 0.0625);
    assert_approx_eq!(expo_in(0.8), 0.25);
    assert_approx_eq!(expo_in(0.9), 0.5);
    assert_approx_eq!(expo_in(0.95), 0.707106);
    assert_approx_eq!(expo_in(0.98), 0.870550);
    assert_eq!(expo_in(1.0), 1.0);
}

#[test]
fn test_inverse_expo_in() {
    assert_eq!(inverse_expo_in(0.0), 0.0);
    assert_approx_eq!(inverse_expo_in(0.003906), 0.199990);
    assert_approx_eq!(inverse_expo_in(0.015625), 0.4);
    assert_approx_eq!(inverse_expo_in(0.03125), 0.5);
    assert_approx_eq!(inverse_expo_in(0.0625), 0.6);
    assert_approx_eq!(inverse_expo_in(0.25), 0.8);
    assert_approx_eq!(inverse_expo_in(0.5), 0.9);
    assert_approx_eq!(inverse_expo_in(0.707106), 0.95);
    assert_approx_eq!(inverse_expo_in(0.870550), 0.98);
    assert_eq!(inverse_expo_in(1.0), 1.0);
}

#[test]
fn test_expo_out() {
    assert_eq!(expo_out(0.0), 0.0);
    assert_approx_eq!(expo_out(0.1), 0.5);
    assert_approx_eq!(expo_out(0.2), 0.75);
    assert_approx_eq!(expo_out(0.4), 0.9375);
    assert_approx_eq!(expo_out(0.5), 0.96875);
    assert_approx_eq!(expo_out(0.6), 0.984375);
    assert_approx_eq!(expo_out(0.8), 0.996093);
    assert_approx_eq!(expo_out(0.9), 0.998046);
    assert_approx_eq!(expo_out(0.95), 0.998618);
    assert_approx_eq!(expo_out(0.98), 0.998878);
    assert_eq!(expo_out(1.0), 1.0);
}

#[test]
fn test_inverse_expo_out() {
    assert_eq!(inverse_expo_out(0.0), 0.0);
    assert_approx_eq!(inverse_expo_out(0.5), 0.1);
    assert_approx_eq!(inverse_expo_out(0.75), 0.2);
    assert_approx_eq!(inverse_expo_out(0.9375), 0.4);
    assert_approx_eq!(inverse_expo_out(0.96875), 0.5);
    assert_approx_eq!(inverse_expo_out(0.984375), 0.6);
    assert_approx_eq!(inverse_expo_out(0.996093), 0.799972);
    assert_approx_eq!(inverse_expo_out(0.998046), 0.899935);
    assert_approx_eq!(inverse_expo_out(0.998618), 0.949902);
    assert_approx_eq!(inverse_expo_out(0.998878), 0.979971);
    assert_eq!(inverse_expo_out(1.0), 1.0);
}

#[test]
fn test_expo_in_out() {
    assert_eq!(expo_in_out(0.0), 0.0);
    assert_approx_eq!(expo_in_out(0.1), 0.001953);
    assert_approx_eq!(expo_in_out(0.12), 0.002577);
    assert_approx_eq!(expo_in_out(0.15), 0.003906);
    assert_approx_eq!(expo_in_out(0.2), 0.007812);
    assert_approx_eq!(expo_in_out(0.4), 0.125);
    assert_approx_eq!(expo_in_out(0.5), 0.5);
    assert_approx_eq!(expo_in_out(0.6), 0.875);
    assert_approx_eq!(expo_in_out(0.8), 0.992187);
    assert_approx_eq!(expo_in_out(0.9), 0.998046);
    assert_approx_eq!(expo_in_out(0.95), 0.999023);
    assert_approx_eq!(expo_in_out(0.98), 0.999355);
    assert_eq!(expo_in_out(1.0), 1.0);
}

#[test]
fn test_inverse_expo_in_out() {
    assert_eq!(inverse_expo_in_out(0.0), 0.0);
    assert_approx_eq!(inverse_expo_in_out(0.001953), 0.099995);
    assert_approx_eq!(inverse_expo_in_out(0.002577), 0.119995);
    assert_approx_eq!(inverse_expo_in_out(0.003906), 0.149995);
    assert_approx_eq!(inverse_expo_in_out(0.007812), 0.199995);
    assert_approx_eq!(inverse_expo_in_out(0.125), 0.4);
    assert_approx_eq!(inverse_expo_in_out(0.5), 0.5);
    assert_approx_eq!(inverse_expo_in_out(0.875), 0.6);
    assert_approx_eq!(inverse_expo_in_out(0.992187), 0.799995);
    assert_approx_eq!(inverse_expo_in_out(0.998046), 0.899967);
    assert_approx_eq!(inverse_expo_in_out(0.999023), 0.949967);
    assert_approx_eq!(inverse_expo_in_out(0.999355), 0.979920);
    assert_eq!(inverse_expo_in_out(1.0), 1.0);
}

#[test]
fn test_circ_in() {
    assert_eq!(circ_in(0.0), 0.0);
    assert_approx_eq!(circ_in(0.1), 0.005012);
    assert_approx_eq!(circ_in(0.2), 0.020204);
    assert_approx_eq!(circ_in(0.4), 0.083484);
    assert_approx_eq!(circ_in(0.5), 0.133974);
    assert_approx_eq!(circ_in(0.6), 0.2);
    assert_approx_eq!(circ_in(0.8), 0.4);
    assert_approx_eq!(circ_in(0.9), 0.564110);
    assert_approx_eq!(circ_in(0.95), 0.687750);
    assert_approx_eq!(circ_in(0.98), 0.801002);
    assert_approx_eq!(circ_in(0.99), 0.858932);
    assert_approx_eq!(circ_in(0.999), 0.955289);
    assert_eq!(circ_in(1.0), 1.0);
}

#[test]
fn test_inverse_circ_in() {
    assert_eq!(inverse_circ_in(0.0), 0.0);
    assert_approx_eq!(inverse_circ_in(0.005012), 0.099994);
    assert_approx_eq!(inverse_circ_in(0.020204), 0.2);
    assert_approx_eq!(inverse_circ_in(0.083484), 0.399998);
    assert_approx_eq!(inverse_circ_in(0.133974), 0.499998);
    assert_approx_eq!(inverse_circ_in(0.2), 0.6);
    assert_approx_eq!(inverse_circ_in(0.4), 0.8);
    assert_approx_eq!(inverse_circ_in(0.564110), 0.9);
    assert_approx_eq!(inverse_circ_in(0.687750), 0.95);
    assert_approx_eq!(inverse_circ_in(0.801002), 0.98);
    assert_approx_eq!(inverse_circ_in(0.858932), 0.99);
    assert_approx_eq!(inverse_circ_in(0.955289), 0.999);
    assert_eq!(inverse_circ_in(1.0), 1.0);
}

#[test]
fn test_circ_out() {
    assert_eq!(circ_out(0.0), 0.0);
    assert_approx_eq!(circ_out(0.01), 0.141067);
    assert_approx_eq!(circ_out(0.025), 0.222204);
    assert_approx_eq!(circ_out(0.05), 0.312249);
    assert_approx_eq!(circ_out(0.1), 0.435889);
    assert_approx_eq!(circ_out(0.2), 0.6);
    assert_approx_eq!(circ_out(0.4), 0.8);
    assert_approx_eq!(circ_out(0.5), 0.866025);
    assert_approx_eq!(circ_out(0.6), 0.916515);
    assert_approx_eq!(circ_out(0.8), 0.979795);
    assert_approx_eq!(circ_out(0.9), 0.994987);
    assert_eq!(circ_out(1.0), 1.0);
}

#[test]
fn test_inverse_circ_out() {
    assert_eq!(inverse_circ_out(0.0), 0.0);
    assert_approx_eq!(inverse_circ_out(0.141067), 0.01);
    assert_approx_eq!(inverse_circ_out(0.222204), 0.025);
    assert_approx_eq!(inverse_circ_out(0.312249), 0.05);
    assert_approx_eq!(inverse_circ_out(0.435889), 0.1);
    assert_approx_eq!(inverse_circ_out(0.6), 0.2);
    assert_approx_eq!(inverse_circ_out(0.8), 0.4);
    assert_approx_eq!(inverse_circ_out(0.866025), 0.5);
    assert_approx_eq!(inverse_circ_out(0.916515), 0.6);
    assert_approx_eq!(inverse_circ_out(0.979795), 0.799995);
    assert_approx_eq!(inverse_circ_out(0.994987), 0.899995);
    assert_eq!(inverse_circ_out(1.0), 1.0);
}

#[test]
fn test_circ_in_out() {
    assert_eq!(circ_in_out(0.0), 0.0);
    assert_approx_eq!(circ_in_out(0.01), 0.000100);
    assert_approx_eq!(circ_in_out(0.025), 0.000625);
    assert_approx_eq!(circ_in_out(0.05), 0.002506);
    assert_approx_eq!(circ_in_out(0.1), 0.010102);
    assert_approx_eq!(circ_in_out(0.2), 0.041742);
    assert_approx_eq!(circ_in_out(0.4), 0.2);
    assert_approx_eq!(circ_in_out(0.5), 0.5);
    assert_approx_eq!(circ_in_out(0.6), 0.8);
    assert_approx_eq!(circ_in_out(0.8), 0.958257);
    assert_approx_eq!(circ_in_out(0.9), 0.989897);
    assert_approx_eq!(circ_in_out(0.925), 0.994342);
    assert_approx_eq!(circ_in_out(0.95), 0.997493);
    assert_approx_eq!(circ_in_out(0.99), 0.999899);
    assert_eq!(circ_in_out(1.0), 1.0);
}

#[test]
fn test_inverse_circ_in_out() {
    assert_eq!(inverse_circ_in_out(0.0), 0.0);
    assert_approx_eq!(inverse_circ_in_out(0.000100), 0.01);
    assert_approx_eq!(inverse_circ_in_out(0.000625), 0.024992);
    assert_approx_eq!(inverse_circ_in_out(0.002506), 0.049997);
    assert_approx_eq!(inverse_circ_in_out(0.010102), 0.1);
    assert_approx_eq!(inverse_circ_in_out(0.041742), 0.2);
    assert_approx_eq!(inverse_circ_in_out(0.2), 0.4);
    assert_approx_eq!(inverse_circ_in_out(0.5), 0.5);
    assert_approx_eq!(inverse_circ_in_out(0.8), 0.6);
    assert_approx_eq!(inverse_circ_in_out(0.958257), 0.799998);
    assert_approx_eq!(inverse_circ_in_out(0.989897), 0.899995);
    assert_approx_eq!(inverse_circ_in_out(0.994342), 0.924993);
    assert_approx_eq!(inverse_circ_in_out(0.997493), 0.949992);
    assert_approx_eq!(inverse_circ_in_out(0.999899), 0.989950);
    assert_eq!(inverse_circ_in_out(1.0), 1.0);
}

#[test]
fn test_back_in() {
    assert_eq!(back_in(0.0), 0.0);
    assert_approx_eq!(back_in(0.01), -0.000167);
    assert_approx_eq!(back_in(0.05), -0.003916);
    assert_approx_eq!(back_in(0.1), -0.014314);
    assert_approx_eq!(back_in(0.2), -0.046450);
    assert_approx_eq!(back_in(0.3), -0.080199);
    assert_approx_eq!(back_in(0.4), -0.099351);
    assert_approx_eq!(back_in(0.5), -0.087697);
    assert_approx_eq!(back_in(0.6), -0.029027);
    assert_approx_eq!(back_in(0.7), 0.092867);
    assert_approx_eq!(back_in(0.8), 0.294197);
    assert_approx_eq!(back_in(0.9), 0.591172);
    assert_approx_eq!(back_in(0.95), 0.780591);
    assert_approx_eq!(back_in(0.99), 0.953621);
    assert_eq!(back_in(1.0), 1.0);
}

#[test]
#[should_panic]
fn test_inverse_back_in() {
    assert_eq!(inverse_back_in(0.0), 0.0);
    assert_eq!(inverse_back_in(1.0), 1.0);
}

#[test]
fn test_back_out() {
    assert_eq!(back_out(0.0), 0.0);
    assert_approx_eq!(back_out(0.01), 0.046378);
    assert_approx_eq!(back_out(0.05), 0.219408);
    assert_approx_eq!(back_out(0.1), 0.408828);
    assert_approx_eq!(back_out(0.2), 0.705803);
    assert_approx_eq!(back_out(0.3), 0.907132);
    assert_approx_eq!(back_out(0.4), 1.029027);
    assert_approx_eq!(back_out(0.5), 1.087697);
    assert_approx_eq!(back_out(0.6), 1.099351);
    assert_approx_eq!(back_out(0.7), 1.080199);
    assert_approx_eq!(back_out(0.8), 1.046450);
    assert_approx_eq!(back_out(0.9), 1.014314);
    assert_approx_eq!(back_out(0.95), 1.003916);
    assert_approx_eq!(back_out(0.99), 1.000167);
    assert_eq!(back_out(1.0), 1.0);
}

#[test]
#[should_panic]
fn test_inverse_back_out() {
    assert_eq!(inverse_back_out(0.0), 0.0);
    assert_eq!(inverse_back_out(1.0), 1.0);
}

#[test]
fn test_back_in_out() {
    assert_eq!(back_in_out(0.0), 0.0);
    assert_approx_eq!(back_in_out(0.01), -0.000504);
    assert_approx_eq!(back_in_out(0.05), -0.011177);
    assert_approx_eq!(back_in_out(0.1), -0.037518);
    assert_approx_eq!(back_in_out(0.2), -0.092555);
    assert_approx_eq!(back_in_out(0.3), -0.078833);
    assert_approx_eq!(back_in_out(0.4), 0.089925);
    assert_approx_eq!(back_in_out(0.5), 0.5);
    assert_approx_eq!(back_in_out(0.6), 0.910074);
    assert_approx_eq!(back_in_out(0.7), 1.078833);
    assert_approx_eq!(back_in_out(0.8), 1.092555);
    assert_approx_eq!(back_in_out(0.9), 1.037518);
    assert_approx_eq!(back_in_out(0.95), 1.011177);
    assert_approx_eq!(back_in_out(0.99), 1.000504);
    assert_eq!(back_in_out(1.0), 1.0);
}

#[test]
#[should_panic]
fn test_inverse_back_in_out() {
    assert_eq!(inverse_back_in_out(0.0), 0.0);
    assert_eq!(inverse_back_in_out(1.0), 1.0);
}

#[test]
fn test_elastic_in() {
    assert_eq!(elastic_in(0.0), 0.0);
    assert_approx_eq!(elastic_in(0.01), 0.0);
    assert_approx_eq!(elastic_in(0.05), 0.000004);
    assert_approx_eq!(elastic_in(0.1), 0.000098);
    assert_approx_eq!(elastic_in(0.2), 0.000494);
    assert_approx_eq!(elastic_in(0.3), -0.007217);
    assert_approx_eq!(elastic_in(0.4), -0.015047);
    assert_approx_eq!(elastic_in(0.5), 0.044194);
    assert_approx_eq!(elastic_in(0.6), 0.104848);
    assert_approx_eq!(elastic_in(0.7), -0.109003);
    assert_approx_eq!(elastic_in(0.8), -0.389552);
    assert_approx_eq!(elastic_in(0.9), 0.102636);
    assert_approx_eq!(elastic_in(0.95), 0.619355);
    assert_approx_eq!(elastic_in(0.99), 0.951012);
    assert_eq!(elastic_in(1.0), 1.0);
}

#[test]
#[should_panic]
fn test_inverse_elastic_in() {
    assert_eq!(inverse_elastic_in(0.0), 0.0);
    assert_eq!(inverse_elastic_in(1.0), 1.0);
}

#[test]
fn test_elastic_out() {
    assert_eq!(elastic_out(0.0), 0.0);
    assert_approx_eq!(elastic_out(0.01), 0.048987);
    assert_approx_eq!(elastic_out(0.05), 0.380644);
    assert_approx_eq!(elastic_out(0.1), 0.897363);
    assert_approx_eq!(elastic_out(0.2), 1.389552);
    assert_approx_eq!(elastic_out(0.3), 1.109003);
    assert_approx_eq!(elastic_out(0.4), 0.895151);
    assert_approx_eq!(elastic_out(0.5), 0.955805);
    assert_approx_eq!(elastic_out(0.6), 1.015047);
    assert_approx_eq!(elastic_out(0.7), 1.007217);
    assert_approx_eq!(elastic_out(0.8), 0.999505);
    assert_approx_eq!(elastic_out(0.9), 0.999901);
    assert_approx_eq!(elastic_out(0.95), 0.999995);
    assert_approx_eq!(elastic_out(0.99), 0.999999);
    assert_eq!(elastic_out(1.0), 1.0);
}

#[test]
#[should_panic]
fn test_inverse_elastic_out() {
    assert_eq!(inverse_elastic_out(0.0), 0.0);
    assert_eq!(inverse_elastic_out(1.0), 1.0);
}

#[test]
fn test_elastic_in_out() {
    assert_eq!(elastic_in_out(0.0), 0.0);
    assert_approx_eq!(elastic_in_out(0.01), 0.0);
    assert_approx_eq!(elastic_in_out(0.05), 0.000049);
    assert_approx_eq!(elastic_in_out(0.1), 0.000247);
    assert_approx_eq!(elastic_in_out(0.2), -0.007523);
    assert_approx_eq!(elastic_in_out(0.3), 0.052424);
    assert_approx_eq!(elastic_in_out(0.4), -0.194776);
    assert_approx_eq!(elastic_in_out(0.5), 0.5);
    assert_approx_eq!(elastic_in_out(0.6), 1.194776);
    assert_approx_eq!(elastic_in_out(0.7), 0.947575);
    assert_approx_eq!(elastic_in_out(0.8), 1.007523);
    assert_approx_eq!(elastic_in_out(0.9), 0.999752);
    assert_approx_eq!(elastic_in_out(0.95), 0.999950);
    assert_approx_eq!(elastic_in_out(0.99), 0.999999);
    assert_eq!(elastic_in_out(1.0), 1.0);
}

#[test]
#[should_panic]
fn test_inverse_elastic_in_out() {
    assert_eq!(inverse_bounce_in(0.0), 0.0);
    assert_eq!(inverse_bounce_in(1.0), 1.0);
}

#[test]
fn test_bounce_in() {
    assert_eq!(bounce_in(0.0), 0.0);
    assert_approx_eq!(bounce_in(0.01), 0.001787);
    assert_approx_eq!(bounce_in(0.05), 0.010051);
    assert_approx_eq!(bounce_in(0.1), 0.021101);
    assert_approx_eq!(bounce_in(0.2), 0.029041);
    assert_approx_eq!(bounce_in(0.3), 0.008511);
    assert_approx_eq!(bounce_in(0.4), 0.078432);
    assert_approx_eq!(bounce_in(0.5), 0.088388);
    assert_approx_eq!(bounce_in(0.6), 0.058547);
    assert_approx_eq!(bounce_in(0.7), 0.283638);
    assert_approx_eq!(bounce_in(0.8), 0.255848);
    assert_approx_eq!(bounce_in(0.9), 0.299522);
    assert_approx_eq!(bounce_in(0.95), 0.692559);
    assert_approx_eq!(bounce_in(0.99), 0.953471);
    assert_eq!(bounce_in(1.0), 1.0);
}

#[test]
#[should_panic]
fn test_inverse_bounce_in() {
    assert_eq!(inverse_bounce_in(0.0), 0.0);
    assert_eq!(inverse_bounce_in(1.0), 1.0);
}

#[test]
fn test_bounce_out() {
    assert_eq!(bounce_out(0.0), 0.0);
    assert_approx_eq!(bounce_out(0.01), 0.046528);
    assert_approx_eq!(bounce_out(0.05), 0.307440);
    assert_approx_eq!(bounce_out(0.1), 0.700477);
    assert_approx_eq!(bounce_out(0.2), 0.744151);
    assert_approx_eq!(bounce_out(0.3), 0.716361);
    assert_approx_eq!(bounce_out(0.4), 0.941452);
    assert_approx_eq!(bounce_out(0.5), 0.911611);
    assert_approx_eq!(bounce_out(0.6), 0.921567);
    assert_approx_eq!(bounce_out(0.7), 0.991488);
    assert_approx_eq!(bounce_out(0.8), 0.970958);
    assert_approx_eq!(bounce_out(0.9), 0.978898);
    assert_approx_eq!(bounce_out(0.95), 0.989948);
    assert_approx_eq!(bounce_out(0.99), 0.998212);
    assert_eq!(bounce_out(1.0), 1.0);
}

#[test]
#[should_panic]
fn test_inverse_bounce_out() {
    assert_eq!(inverse_bounce_out(0.0), 0.0);
    assert_eq!(inverse_bounce_out(1.0), 1.0);
}

#[test]
fn test_bounce_in_out() {
    assert_eq!(bounce_in_out(0.0), 0.0);
    assert_approx_eq!(bounce_in_out(0.01), 0.007205);
    assert_approx_eq!(bounce_in_out(0.05), 0.036740);
    assert_approx_eq!(bounce_in_out(0.1), 0.044018);
    assert_approx_eq!(bounce_in_out(0.2), 0.090095);
    assert_approx_eq!(bounce_in_out(0.3), 0.050968);
    assert_approx_eq!(bounce_in_out(0.4), 0.168796);
    assert_approx_eq!(bounce_in_out(0.5), 0.5);
    assert_approx_eq!(bounce_in_out(0.6), 0.831203);
    assert_approx_eq!(bounce_in_out(0.7), 0.949031);
    assert_approx_eq!(bounce_in_out(0.8), 0.909904);
    assert_approx_eq!(bounce_in_out(0.9), 0.955981);
    assert_approx_eq!(bounce_in_out(0.95), 0.963259);
    assert_approx_eq!(bounce_in_out(0.99), 0.992794);
    assert_eq!(bounce_in_out(1.0), 1.0);
}

#[test]
#[should_panic]
fn test_inverse_bounce_in_out() {
    assert_eq!(inverse_bounce_in_out(0.0), 0.0);
    assert_eq!(inverse_bounce_in_out(1.0), 1.0);
}
