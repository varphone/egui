/// A collection of easing functions for use in animations and transitions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
}

impl Easing {
    /// Returns the easing function for the given easing type.
    #[inline]
    pub fn apply_function(self) -> fn(f64) -> f64 {
        match self {
            Easing::SineIn => ease_in_sine,
            Easing::SineOut => ease_out_sine,
            Easing::SineInOut => ease_in_out_sine,
            Easing::QuadIn => ease_in_quad,
            Easing::QuadOut => ease_out_quad,
            Easing::QuadInOut => ease_in_out_quad,
            Easing::CubicIn => ease_in_cubic,
            Easing::CubicOut => ease_out_cubic,
            Easing::CubicInOut => ease_in_out_cubic,
            Easing::QuartIn => ease_in_quart,
            Easing::QuartOut => ease_out_quart,
            Easing::QuartInOut => ease_in_out_quart,
            Easing::QuintIn => ease_in_quint,
            Easing::QuintOut => ease_out_quint,
            Easing::QuintInOut => ease_in_out_quint,
            Easing::ExpoIn => ease_in_expo,
            Easing::ExpoOut => ease_out_expo,
            Easing::ExpoInOut => ease_in_out_expo,
            Easing::CircIn => ease_in_circ,
            Easing::CircOut => ease_out_circ,
            Easing::CircInOut => ease_in_out_circ,
            Easing::BackIn => ease_in_back,
            Easing::BackOut => ease_out_back,
            Easing::BackInOut => ease_in_out_back,
            Easing::ElasticIn => ease_in_elastic,
            Easing::ElasticOut => ease_out_elastic,
            Easing::ElasticInOut => ease_in_out_elastic,
            Easing::BounceIn => ease_in_bounce,
            Easing::BounceOut => ease_out_bounce,
            Easing::BounceInOut => ease_in_out_bounce,
        }
    }

    /// Returns the inverse easing function for the given easing type.
    #[inline]
    pub fn inverse_function(self) -> fn(f64) -> f64 {
        match self {
            Easing::SineIn => ease_in_sine_inverse,
            Easing::SineOut => ease_out_sine_inverse,
            Easing::SineInOut => ease_in_out_sine_inverse,
            Easing::QuadIn => ease_in_quad_inverse,
            Easing::QuadOut => ease_out_quad_inverse,
            Easing::QuadInOut => ease_in_out_quad_inverse,
            Easing::CubicIn => ease_in_cubic_inverse,
            Easing::CubicOut => ease_out_cubic_inverse,
            Easing::CubicInOut => ease_in_out_cubic_inverse,
            Easing::QuartIn => ease_in_quart_inverse,
            Easing::QuartOut => ease_out_quart_inverse,
            Easing::QuartInOut => ease_in_out_quart_inverse,
            Easing::QuintIn => ease_in_quint_inverse,
            Easing::QuintOut => ease_out_quint_inverse,
            Easing::QuintInOut => ease_in_out_quint_inverse,
            Easing::ExpoIn => ease_in_expo_inverse,
            Easing::ExpoOut => ease_out_expo_inverse,
            Easing::ExpoInOut => ease_in_out_expo_inverse,
            Easing::CircIn => ease_in_circ_inverse,
            Easing::CircOut => ease_out_circ_inverse,
            Easing::CircInOut => ease_in_out_circ_inverse,
            Easing::BackIn => ease_in_back_inverse,
            Easing::BackOut => ease_out_back_inverse,
            Easing::BackInOut => ease_in_out_back_inverse,
            Easing::ElasticIn => ease_in_elastic_inverse,
            Easing::ElasticOut => ease_out_elastic_inverse,
            Easing::ElasticInOut => ease_in_out_elastic_inverse,
            Easing::BounceIn => ease_in_bounce_inverse,
            Easing::BounceOut => ease_out_bounce_inverse,
            Easing::BounceInOut => ease_in_out_bounce_inverse,
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
    /// let easing = Easing::InSine;
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
    /// let easing = Easing::InSine;
    /// let t = 0.5;
    /// let result = easing.inverse(t);
    /// ```
    pub fn inverse(self, t: f64) -> f64 {
        (self.inverse_function())(t)
    }
}

#[inline]
pub fn ease_in_sine(x: f64) -> f64 {
    1.0 - (1.0 - x).cos()
}

#[inline]
pub fn ease_in_sine_inverse(x: f64) -> f64 {
    (1.0 - x).acos()
}

#[inline]
pub fn ease_out_sine(x: f64) -> f64 {
    x.sin()
}

#[inline]
pub fn ease_out_sine_inverse(x: f64) -> f64 {
    x.asin()
}

#[inline]
pub fn ease_in_out_sine(x: f64) -> f64 {
    0.5 - (0.5 * x).cos()
}

#[inline]
pub fn ease_in_out_sine_inverse(x: f64) -> f64 {
    (0.5 - x).acos() * 2.0
}

#[inline]
pub fn ease_in_quad(x: f64) -> f64 {
    x * x
}

#[inline]
pub fn ease_in_quad_inverse(x: f64) -> f64 {
    x.sqrt()
}

#[inline]
pub fn ease_out_quad(x: f64) -> f64 {
    1.0 - (1.0 - x) * (1.0 - x)
}

#[inline]
pub fn ease_out_quad_inverse(x: f64) -> f64 {
    1.0 - (1.0 - x).sqrt()
}

#[inline]
pub fn ease_in_out_quad(x: f64) -> f64 {
    if x < 0.5 {
        2.0 * x * x
    } else {
        1.0 - (-2.0 * x + 2.0) * (-2.0 * x + 2.0) / 2.0
    }
}

#[inline]
pub fn ease_in_out_quad_inverse(x: f64) -> f64 {
    if x < 0.5 {
        (x / 2.0).sqrt()
    } else {
        1.0 - ((1.0 - x) / 2.0).sqrt()
    }
}

#[inline]
pub fn ease_in_cubic(x: f64) -> f64 {
    x * x * x
}

#[inline]
pub fn ease_in_cubic_inverse(x: f64) -> f64 {
    x.cbrt()
}

#[inline]
pub fn ease_out_cubic(x: f64) -> f64 {
    1.0 - (1.0 - x) * (1.0 - x) * (1.0 - x)
}

#[inline]
pub fn ease_out_cubic_inverse(x: f64) -> f64 {
    1.0 - (1.0 - x).cbrt()
}

#[inline]
pub fn ease_in_out_cubic(x: f64) -> f64 {
    if x < 0.5 {
        4.0 * x * x * x
    } else {
        1.0 - (-2.0 * x + 2.0) * (-2.0 * x + 2.0) * (-2.0 * x + 2.0) / 2.0
    }
}

#[inline]
pub fn ease_in_out_cubic_inverse(x: f64) -> f64 {
    if x < 0.5 {
        (x / 4.0).cbrt()
    } else {
        1.0 - ((1.0 - x) / 2.0).cbrt()
    }
}

#[inline]
pub fn ease_in_quart(x: f64) -> f64 {
    x * x * x * x
}

#[inline]
pub fn ease_in_quart_inverse(x: f64) -> f64 {
    x.powf(1.0 / 4.0)
}

#[inline]
pub fn ease_out_quart(x: f64) -> f64 {
    1.0 - (1.0 - x) * (1.0 - x) * (1.0 - x) * (1.0 - x)
}

#[inline]
pub fn ease_out_quart_inverse(x: f64) -> f64 {
    1.0 - (1.0 - x).powf(1.0 / 4.0)
}

#[inline]
pub fn ease_in_out_quart(x: f64) -> f64 {
    if x < 0.5 {
        8.0 * x * x * x * x
    } else {
        1.0 - (-2.0 * x + 2.0) * (-2.0 * x + 2.0) * (-2.0 * x + 2.0) * (-2.0 * x + 2.0) / 2.0
    }
}

#[inline]
pub fn ease_in_out_quart_inverse(x: f64) -> f64 {
    if x < 0.5 {
        (x / 8.0).powf(1.0 / 4.0)
    } else {
        1.0 - ((1.0 - x) / 2.0).powf(1.0 / 4.0)
    }
}

#[inline]
pub fn ease_in_quint(x: f64) -> f64 {
    x * x * x * x * x
}

#[inline]
pub fn ease_in_quint_inverse(x: f64) -> f64 {
    x.powf(1.0 / 5.0)
}

#[inline]
pub fn ease_out_quint(x: f64) -> f64 {
    let t = x - 1.0;
    t * t * t * t * t + 1.0
}

#[inline]
pub fn ease_out_quint_inverse(x: f64) -> f64 {
    let t = x - 1.0;
    t.powf(1.0 / 5.0) + 1.0
}

#[inline]
pub fn ease_in_out_quint(x: f64) -> f64 {
    if x < 0.5 {
        16.0 * x * x * x * x * x
    } else {
        let t = (2.0 * x) - 2.0;
        0.5 * t * t * t * t * t + 1.0
    }
}

#[inline]
pub fn ease_in_out_quint_inverse(x: f64) -> f64 {
    if x < 0.5 {
        (x / 16.0).powf(1.0 / 5.0)
    } else {
        let t = (2.0 * x) - 2.0;
        0.5 * t.powf(1.0 / 5.0) + 1.0
    }
}

#[inline]
pub fn ease_in_expo(x: f64) -> f64 {
    if x == 0.0 {
        0.0
    } else {
        2.0f64.powf(10.0 * x - 10.0)
    }
}

#[inline]
pub fn ease_in_expo_inverse(x: f64) -> f64 {
    if x == 0.0 {
        0.0
    } else {
        (10.0 * x).log2() + 1.0
    }
}

#[inline]
pub fn ease_out_expo(x: f64) -> f64 {
    if x == 1.0 {
        1.0
    } else {
        1.0 - 2.0f64.powf(-10.0 * x)
    }
}

#[inline]
pub fn ease_out_expo_inverse(x: f64) -> f64 {
    if x == 1.0 {
        1.0
    } else {
        1.0 - (10.0 * (1.0 - x)).log2()
    }
}

#[inline]
pub fn ease_in_out_expo(x: f64) -> f64 {
    if x == 0.0 {
        0.0
    } else if x == 1.0 {
        1.0
    } else if x < 0.5 {
        2.0f64.powf(20.0 * x - 10.0) / 2.0
    } else {
        (2.0 - 2.0f64.powf(-20.0 * x + 10.0)) / 2.0
    }
}

#[inline]
pub fn ease_in_out_expo_inverse(x: f64) -> f64 {
    if x == 0.0 {
        0.0
    } else if x == 1.0 {
        1.0
    } else if x < 0.5 {
        (10.0 * x).log2() / 20.0 + 0.5
    } else {
        (10.0 * (1.0 - x)).log2() / -20.0 + 0.5
    }
}

#[inline]
pub fn ease_in_circ(x: f64) -> f64 {
    1.0 - (1.0 - x * x).sqrt()
}

#[inline]
pub fn ease_in_circ_inverse(x: f64) -> f64 {
    1.0 - (1.0 - x).powf(2.0)
}

#[inline]
pub fn ease_out_circ(x: f64) -> f64 {
    (1.0 - (1.0 - x) * (1.0 - x)).sqrt()
}

#[inline]
pub fn ease_out_circ_inverse(x: f64) -> f64 {
    1.0 - (1.0 - x).sqrt()
}

#[inline]
pub fn ease_in_out_circ(x: f64) -> f64 {
    if x < 0.5 {
        (1.0 - (1.0 - 2.0 * x) * (1.0 - 2.0 * x)).sqrt() / 2.0
    } else {
        (1.0 - (-2.0 * x + 2.0) * (-2.0 * x + 2.0)).sqrt() / 2.0 + 0.5
    }
}

#[inline]
pub fn ease_in_out_circ_inverse(x: f64) -> f64 {
    if x < 0.5 {
        1.0 - (1.0 - 2.0 * x).sqrt()
    } else {
        1.0 - (-2.0 * x + 2.0).sqrt()
    }
}

#[inline]
pub fn ease_in_back(x: f64) -> f64 {
    2.70158 * x * x * x - 1.70158 * x * x
}

#[inline]
pub fn ease_in_back_inverse(x: f64) -> f64 {
    (x + 1.70158 * x * x) / 2.70158
}

#[inline]
pub fn ease_out_back(x: f64) -> f64 {
    1.0 + 2.70158 * (x - 1.0) * (x - 1.0) * (x - 1.0) + 1.70158 * (x - 1.0) * (x - 1.0)
}

#[inline]
pub fn ease_out_back_inverse(x: f64) -> f64 {
    1.0 - (2.70158 * (1.0 - x) * (1.0 - x) * (1.0 - x) + 1.70158 * (1.0 - x) * (1.0 - x))
}

#[inline]
pub fn ease_in_out_back(x: f64) -> f64 {
    if x < 0.5 {
        4.07407 * x * x * x - 2.03704 * x * x
    } else {
        1.0 + 2.70158 * (2.0 * x - 2.0) * (2.0 * x - 2.0) * (2.0 * x - 2.0)
            + 1.70158 * (2.0 * x - 2.0) * (2.0 * x - 2.0)
    }
}

#[inline]
pub fn ease_in_out_back_inverse(x: f64) -> f64 {
    if x < 0.5 {
        (x + 2.03704 * x * x) / 4.07407
    } else {
        1.0 - (2.70158 * (2.0 - x) * (2.0 - x) * (2.0 - x) + 1.70158 * (2.0 - x) * (2.0 - x))
    }
}

#[inline]
pub fn ease_in_elastic(x: f64) -> f64 {
    if x == 0.0 {
        0.0
    } else if x == 1.0 {
        1.0
    } else {
        -2.0f64.powf(10.0 * x - 10.0) * (x - 1.0) * (2.0 * std::f64::consts::PI / 3.0).sin()
    }
}

#[inline]
pub fn ease_in_elastic_inverse(x: f64) -> f64 {
    if x == 0.0 {
        0.0
    } else if x == 1.0 {
        1.0
    } else {
        (10.0 * x - 10.0).asin() / (2.0 * std::f64::consts::PI / 3.0) + 1.0
    }
}

#[inline]
pub fn ease_out_elastic(x: f64) -> f64 {
    if x == 0.0 {
        0.0
    } else if x == 1.0 {
        1.0
    } else {
        2.0f64.powf(-10.0 * x) * x * (2.0 * std::f64::consts::PI / 3.0).sin() + 1.0
    }
}

#[inline]
pub fn ease_out_elastic_inverse(x: f64) -> f64 {
    if x == 0.0 {
        0.0
    } else if x == 1.0 {
        1.0
    } else {
        (-10.0 * x).asin() / (2.0 * std::f64::consts::PI / 3.0)
    }
}

#[inline]
pub fn ease_in_out_elastic(x: f64) -> f64 {
    if x == 0.0 {
        0.0
    } else if x == 1.0 {
        1.0
    } else if x < 0.5 {
        -2.0f64.powf(20.0 * x - 10.0) * (2.0 * x - 1.0) * (2.0 * std::f64::consts::PI / 3.0).sin()
            / 2.0
    } else {
        2.0f64.powf(-20.0 * x + 10.0) * (2.0 * x - 1.0) * (2.0 * std::f64::consts::PI / 3.0).sin()
            / 2.0
            + 1.0
    }
}

#[inline]
pub fn ease_in_out_elastic_inverse(x: f64) -> f64 {
    if x == 0.0 {
        0.0
    } else if x == 1.0 {
        1.0
    } else if x < 0.5 {
        (20.0 * x - 10.0).asin() / (2.0 * std::f64::consts::PI / 3.0) / 2.0 + 0.5
    } else {
        (-20.0 * x + 10.0).asin() / (2.0 * std::f64::consts::PI / 3.0) / 2.0 + 0.5
    }
}

#[inline]
pub fn ease_in_bounce(x: f64) -> f64 {
    1.0 - ease_out_bounce(1.0 - x)
}

#[inline]
pub fn ease_in_bounce_inverse(x: f64) -> f64 {
    1.0 - ease_out_bounce_inverse(x)
}

#[inline]
pub fn ease_out_bounce(x: f64) -> f64 {
    if x < 4.0 / 11.0 {
        (121.0 * x * x) / 16.0
    } else if x < 8.0 / 11.0 {
        (363.0 / 40.0 * x * x) - (99.0 / 10.0 * x) + 17.0 / 5.0
    } else if x < 9.0 / 10.0 {
        (4356.0 / 361.0 * x * x) - (35442.0 / 1805.0 * x) + 16061.0 / 1805.0
    } else {
        (54.0 / 5.0 * x * x) - (513.0 / 25.0 * x) + 268.0 / 25.0
    }
}

#[inline]
pub fn ease_out_bounce_inverse(x: f64) -> f64 {
    if x < 4.0 / 11.0 {
        (16.0 * x).sqrt() / 11.0
    } else if x < 8.0 / 11.0 {
        (40.0 * x + 99.0) / 363.0
    } else if x < 9.0 / 10.0 {
        (361.0 * x + 35442.0) / 4356.0
    } else {
        (5.0 * x + 513.0) / 54.0
    }
}

#[inline]
pub fn ease_in_out_bounce(x: f64) -> f64 {
    if x < 0.5 {
        0.5 * ease_in_bounce(2.0 * x)
    } else {
        0.5 * ease_out_bounce(2.0 * x - 1.0) + 0.5
    }
}

#[inline]
pub fn ease_in_out_bounce_inverse(x: f64) -> f64 {
    if x < 0.5 {
        0.5 * ease_in_bounce_inverse(2.0 * x)
    } else {
        0.5 * ease_out_bounce_inverse(2.0 * x - 1.0) + 0.5
    }
}
