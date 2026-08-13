//! LensOS Animation & Physics Framework
//!
//! Provides easing curve evaluation, spring physics solvers, smooth value transitions,
//! keyframe interpolation, and temporal timeline management.

/// Easing curve algorithms for fluid UI transitions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Easing {
    Linear,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutCubic,
    EaseOutBack,
    Spring { tension: f32, friction: f32 },
}

impl Easing {
    /// Evaluates normalized progress `t` (`0.0..=1.0`) returning eased output factor.
    pub fn evaluate(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Easing::Linear => t,
            Easing::EaseInQuad => t * t,
            Easing::EaseOutQuad => t * (2.0 - t),
            Easing::EaseInOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    let p = -2.0 * t + 2.0;
                    1.0 - (p * p * p) / 2.0
                }
            }
            Easing::EaseOutBack => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
            }
            Easing::Spring { tension, friction } => {
                let omega = (tension / 1.0).sqrt();
                let alpha = friction / (2.0 * (1.0 * tension).sqrt());
                let decay = (-alpha * omega * t).exp();
                1.0 - decay * ((1.0 - alpha * alpha).sqrt() * omega * t).cos()
            }
        }
    }
}

/// Dynamic value state machine tracking property animation lifecycles.
#[derive(Debug, Clone, PartialEq)]
pub struct AnimationState {
    pub start_value: f32,
    pub target_value: f32,
    pub current_value: f32,
    pub duration_secs: f32,
    pub elapsed_secs: f32,
    pub easing: Easing,
    pub is_finished: bool,
}

impl AnimationState {
    pub fn new(start_value: f32, target_value: f32, duration_secs: f32, easing: Easing) -> Self {
        Self {
            start_value,
            target_value,
            current_value: start_value,
            duration_secs: duration_secs.max(0.001),
            elapsed_secs: 0.0,
            easing,
            is_finished: false,
        }
    }

    /// Advances animation progression time step by `dt` seconds.
    pub fn update(&mut self, dt_secs: f32) {
        if self.is_finished {
            return;
        }

        self.elapsed_secs += dt_secs;
        let progress = (self.elapsed_secs / self.duration_secs).clamp(0.0, 1.0);
        let eased_t = self.easing.evaluate(progress);

        self.current_value = self.start_value + (self.target_value - self.start_value) * eased_t;

        if progress >= 1.0 {
            self.current_value = self.target_value;
            self.is_finished = true;
        }
    }
}

/// Generic container for interpolating arbitrary animated properties.
#[derive(Debug, Clone, PartialEq)]
pub struct Transition<T> {
    pub current: T,
    pub target: T,
    pub animation: AnimationState,
}

impl Transition<f32> {
    pub fn new(value: f32) -> Self {
        Self {
            current: value,
            target: value,
            animation: AnimationState::new(value, value, 0.2, Easing::EaseOutQuad),
        }
    }

    pub fn set_target(&mut self, target: f32, duration_secs: f32, easing: Easing) {
        self.target = target;
        self.animation = AnimationState::new(self.current, target, duration_secs, easing);
    }

    pub fn tick(&mut self, dt_secs: f32) {
        self.animation.update(dt_secs);
        self.current = self.animation.current_value;
    }
}

/// Animation controller driving frame timeline updates.
#[derive(Debug, Default)]
pub struct AnimationController {
    active_animations: Vec<AnimationState>,
}

impl AnimationController {
    pub fn new() -> Self {
        Self {
            active_animations: Vec::new(),
        }
    }

    pub fn start_animation(&mut self, anim: AnimationState) {
        self.active_animations.push(anim);
    }

    pub fn tick_all(&mut self, dt_secs: f32) {
        for anim in self.active_animations.iter_mut() {
            anim.update(dt_secs);
        }
        self.active_animations.retain(|anim| !anim.is_finished);
    }

    pub fn active_count(&self) -> usize {
        self.active_animations.len()
    }
}
