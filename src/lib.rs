#![no_std]
#![forbid(unsafe_code)]
#![deny(warnings, clippy::all)]

use micromath::F32Ext;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum AlgorithmType {
    Voc,
    Nox,
}

#[derive(Debug)]
pub struct GasIndexAlgorithm {
    state: GasIndexAlgorithmParams,
}

impl GasIndexAlgorithm {
    pub fn new(algorithm_type: AlgorithmType, sampling_interval: f32) -> Self {
        let mut state = GasIndexAlgorithmParams {
            algorithm_type,
            sampling_interval: 0.0,
            index_offset: 0.0,
            sraw_minimum: 0,
            gating_max_duration_minutes: 0.0,
            init_duration_mean: 0.0,
            init_duration_variance: 0.0,
            gating_threshold: 0.0,
            index_gain: 0.0,
            tau_mean_hours: 0.0,
            tau_variance_hours: 0.0,
            sraw_std_initial: 0.0,
            uptime: 0.0,
            sraw: 0.0,
            gas_index: 0.0,
            mean_variance_estimator_initialized: false,
            mean_variance_estimator_mean: 0.0,
            mean_variance_estimator_sraw_offset: 0.0,
            mean_variance_estimator_std: 0.0,
            mean_variance_estimator_gamma_mean: 0.0,
            mean_variance_estimator_gamma_variance: 0.0,
            mean_variance_estimator_gamma_initial_mean: 0.0,
            mean_variance_estimator_gamma_initial_variance: 0.0,
            mean_variance_estimator_gamma_mean2: 0.0,
            mean_variance_estimator_gamma_variance2: 0.0,
            mean_variance_estimator_uptime_gamma: 0.0,
            mean_variance_estimator_uptime_gating: 0.0,
            mean_variance_estimator_gating_duration_minutes: 0.0,
            mean_variance_estimator_sigmoid_k: 0.0,
            mean_variance_estimator_sigmoid_x0: 0.0,
            mox_model_sraw_std: 0.0,
            mox_model_sraw_mean: 0.0,
            sigmoid_scaled_k: 0.0,
            sigmoid_scaled_x0: 0.0,
            sigmoid_scaled_offset_default: 0.0,
            adaptive_lowpass_a1: 0.0,
            adaptive_lowpass_a2: 0.0,
            adaptive_lowpass_initialized: false,
            adaptive_lowpass_x1: 0.0,
            adaptive_lowpass_x2: 0.0,
            adaptive_lowpass_x3: 0.0,
        };

        state.init_with_sampling_interval(algorithm_type, sampling_interval);

        Self { state }
    }

    /// Calculate the gas index value from the raw sensor value.
    ///
    /// Returns the calculated gas index value from the raw sensor value.
    /// Zero during initial blackout period and 1..500 afterwards.
    pub fn process(&mut self, sraw: i32) -> i32 {
        self.state.process(sraw)
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct GasIndexAlgorithmParams {
    pub algorithm_type: AlgorithmType,
    pub sampling_interval: f32,
    pub index_offset: f32,
    pub sraw_minimum: i32,
    pub gating_max_duration_minutes: f32,
    pub init_duration_mean: f32,
    pub init_duration_variance: f32,
    pub gating_threshold: f32,
    pub index_gain: f32,
    pub tau_mean_hours: f32,
    pub tau_variance_hours: f32,
    pub sraw_std_initial: f32,
    pub uptime: f32,
    pub sraw: f32,
    pub gas_index: f32,
    pub mean_variance_estimator_initialized: bool,
    pub mean_variance_estimator_mean: f32,
    pub mean_variance_estimator_sraw_offset: f32,
    pub mean_variance_estimator_std: f32,
    pub mean_variance_estimator_gamma_mean: f32,
    pub mean_variance_estimator_gamma_variance: f32,
    pub mean_variance_estimator_gamma_initial_mean: f32,
    pub mean_variance_estimator_gamma_initial_variance: f32,
    pub mean_variance_estimator_gamma_mean2: f32,
    pub mean_variance_estimator_gamma_variance2: f32,
    pub mean_variance_estimator_uptime_gamma: f32,
    pub mean_variance_estimator_uptime_gating: f32,
    pub mean_variance_estimator_gating_duration_minutes: f32,
    pub mean_variance_estimator_sigmoid_k: f32,
    pub mean_variance_estimator_sigmoid_x0: f32,
    pub mox_model_sraw_std: f32,
    pub mox_model_sraw_mean: f32,
    pub sigmoid_scaled_k: f32,
    pub sigmoid_scaled_x0: f32,
    pub sigmoid_scaled_offset_default: f32,
    pub adaptive_lowpass_a1: f32,
    pub adaptive_lowpass_a2: f32,
    pub adaptive_lowpass_initialized: bool,
    pub adaptive_lowpass_x1: f32,
    pub adaptive_lowpass_x2: f32,
    pub adaptive_lowpass_x3: f32,
}

impl GasIndexAlgorithmParams {
    fn init_with_sampling_interval(
        &mut self,
        algorithm_type: AlgorithmType,
        sampling_interval: f32,
    ) {
        self.algorithm_type = algorithm_type;
        self.sampling_interval = sampling_interval;
        match self.algorithm_type {
            AlgorithmType::Nox => {
                self.index_offset = 1.0;
                self.sraw_minimum = 10000;
                self.gating_max_duration_minutes = 60.0 * 12.0;
                self.init_duration_mean = 3600.0 * 4.75f32;
                self.init_duration_variance = 3600.0 * 5.70;
                self.gating_threshold = 30.0;
            }
            AlgorithmType::Voc => {
                self.index_offset = 100.0;
                self.sraw_minimum = 20000;
                self.gating_max_duration_minutes = 60.0 * 3.0;
                self.init_duration_mean = 3600.0 * 0.75f32;
                self.init_duration_variance = 3600.0 * 1.45f32;
                self.gating_threshold = 340.0;
            }
        }
        self.index_gain = 230.0;
        self.tau_mean_hours = 12.0;
        self.tau_variance_hours = 12.0;
        self.sraw_std_initial = 50.0;
        self.reset();
    }

    fn reset(&mut self) {
        self.uptime = 0.0;
        self.sraw = 0.0;
        self.gas_index = 0.0;
        self.init_instances();
    }

    fn init_instances(&mut self) {
        self.mean_variance_estimator_set_parameters();
        let sraw_std = self.mean_variance_estimator_get_std();
        let sraw_mean = self.mean_variance_estimator_get_mean();
        self.mox_model_set_parameters(sraw_std, sraw_mean);
        match self.algorithm_type {
            AlgorithmType::Nox => {
                self.sigmoid_scaled_set_parameters(614.0, -0.0101f32, 1.0);
            }
            AlgorithmType::Voc => {
                self.sigmoid_scaled_set_parameters(213.0, -0.0065f32, 100.0);
            }
        }
        self.adaptive_lowpass_set_parameters();
    }

    fn process(&mut self, mut sraw: i32) -> i32 {
        if self.uptime <= 45.0 {
            self.uptime += self.sampling_interval;
        } else {
            if sraw > 0 && sraw < 65000 {
                if sraw < self.sraw_minimum + 1 {
                    sraw = self.sraw_minimum + 1;
                } else if sraw > self.sraw_minimum + 32767 {
                    sraw = self.sraw_minimum + 32767;
                }
                self.sraw = (sraw - self.sraw_minimum) as f32;
            }
            if self.algorithm_type == AlgorithmType::Voc
                || self.mean_variance_estimator_is_initialized()
            {
                self.gas_index = self.mox_model_process(self.sraw);
                self.gas_index = self.sigmoid_scaled_process(self.gas_index);
            } else {
                self.gas_index = self.index_offset;
            }
            self.gas_index = self.adaptive_lowpass_process(self.gas_index);
            if self.gas_index < 0.5f32 {
                self.gas_index = 0.5f32;
            }
            if self.sraw > 0.0 {
                self.mean_variance_estimator_process(self.sraw);

                let sraw_std = self.mean_variance_estimator_get_std();
                let sraw_mean = self.mean_variance_estimator_get_mean();
                self.mox_model_set_parameters(sraw_std, sraw_mean);
            }
        }
        (self.gas_index + 0.5f32) as i32
    }

    fn mean_variance_estimator_set_parameters(&mut self) {
        self.mean_variance_estimator_initialized = false;
        self.mean_variance_estimator_mean = 0.0;
        self.mean_variance_estimator_sraw_offset = 0.0;
        self.mean_variance_estimator_std = self.sraw_std_initial;
        self.mean_variance_estimator_gamma_mean = 8.0 * 64.0 * (self.sampling_interval / 3600.0)
            / (self.tau_mean_hours + self.sampling_interval / 3600.0);
        self.mean_variance_estimator_gamma_variance = 64.0 * (self.sampling_interval / 3600.0)
            / (self.tau_variance_hours + self.sampling_interval / 3600.0);
        match self.algorithm_type {
            AlgorithmType::Nox => {
                self.mean_variance_estimator_gamma_initial_mean =
                    8.0 * 64.0 * self.sampling_interval / (1200.0 + self.sampling_interval);
            }
            AlgorithmType::Voc => {
                self.mean_variance_estimator_gamma_initial_mean =
                    8.0 * 64.0 * self.sampling_interval / (20.0 + self.sampling_interval);
            }
        }
        self.mean_variance_estimator_gamma_initial_variance =
            64.0 * self.sampling_interval / (2500.0 + self.sampling_interval);
        self.mean_variance_estimator_gamma_mean2 = 0.0;
        self.mean_variance_estimator_gamma_variance2 = 0.0;
        self.mean_variance_estimator_uptime_gamma = 0.0;
        self.mean_variance_estimator_uptime_gating = 0.0;
        self.mean_variance_estimator_gating_duration_minutes = 0.0;
    }

    fn mean_variance_estimator_get_std(&mut self) -> f32 {
        self.mean_variance_estimator_std
    }

    fn mean_variance_estimator_get_mean(&mut self) -> f32 {
        self.mean_variance_estimator_mean + self.mean_variance_estimator_sraw_offset
    }

    fn mean_variance_estimator_is_initialized(&mut self) -> bool {
        self.mean_variance_estimator_initialized
    }

    fn mean_variance_estimator_calculate_gamma(&mut self) {
        let uptime_limit = 32767.0 - self.sampling_interval;
        if self.mean_variance_estimator_uptime_gamma < uptime_limit {
            self.mean_variance_estimator_uptime_gamma += self.sampling_interval;
        }
        if self.mean_variance_estimator_uptime_gating < uptime_limit {
            self.mean_variance_estimator_uptime_gating += self.sampling_interval;
        }
        self.mean_variance_estimator_sigmoid_set_parameters(self.init_duration_mean, 0.01f32);
        let sigmoid_gamma_mean =
            self.mean_variance_estimator_sigmoid_process(self.mean_variance_estimator_uptime_gamma);
        let gamma_mean = self.mean_variance_estimator_gamma_mean
            + (self.mean_variance_estimator_gamma_initial_mean
                - self.mean_variance_estimator_gamma_mean)
                * sigmoid_gamma_mean;
        let gating_threshold_mean = self.gating_threshold
            + (510.0 - self.gating_threshold)
                * self.mean_variance_estimator_sigmoid_process(
                    self.mean_variance_estimator_uptime_gating,
                );
        self.mean_variance_estimator_sigmoid_set_parameters(gating_threshold_mean, 0.09f32);
        let sigmoid_gating_mean = self.mean_variance_estimator_sigmoid_process(self.gas_index);
        self.mean_variance_estimator_gamma_mean2 = sigmoid_gating_mean * gamma_mean;
        self.mean_variance_estimator_sigmoid_set_parameters(self.init_duration_variance, 0.01f32);
        let sigmoid_gamma_variance =
            self.mean_variance_estimator_sigmoid_process(self.mean_variance_estimator_uptime_gamma);
        let gamma_variance = self.mean_variance_estimator_gamma_variance
            + (self.mean_variance_estimator_gamma_initial_variance
                - self.mean_variance_estimator_gamma_variance)
                * (sigmoid_gamma_variance - sigmoid_gamma_mean);
        let gating_threshold_variance = self.gating_threshold
            + (510.0 - self.gating_threshold)
                * self.mean_variance_estimator_sigmoid_process(
                    self.mean_variance_estimator_uptime_gating,
                );
        self.mean_variance_estimator_sigmoid_set_parameters(gating_threshold_variance, 0.09f32);
        let sigmoid_gating_variance = self.mean_variance_estimator_sigmoid_process(self.gas_index);
        self.mean_variance_estimator_gamma_variance2 = sigmoid_gating_variance * gamma_variance;

        self.mean_variance_estimator_gating_duration_minutes +=
            self.sampling_interval / 60.0 * ((1.0 - sigmoid_gating_mean) * (1.0 + 0.3f32) - 0.3f32);
        if self.mean_variance_estimator_gating_duration_minutes < 0.0 {
            self.mean_variance_estimator_gating_duration_minutes = 0.0;
        }
        if self.mean_variance_estimator_gating_duration_minutes > self.gating_max_duration_minutes {
            self.mean_variance_estimator_uptime_gating = 0.0;
        }
    }

    fn mean_variance_estimator_process(&mut self, mut sraw: f32) {
        if !self.mean_variance_estimator_initialized {
            self.mean_variance_estimator_initialized = true;
            self.mean_variance_estimator_sraw_offset = sraw;
            self.mean_variance_estimator_mean = 0.0;
        } else {
            if self.mean_variance_estimator_mean >= 100.0
                || self.mean_variance_estimator_mean <= -100.0
            {
                self.mean_variance_estimator_sraw_offset += self.mean_variance_estimator_mean;
                self.mean_variance_estimator_mean = 0.0;
            }
            sraw -= self.mean_variance_estimator_sraw_offset;

            self.mean_variance_estimator_calculate_gamma();
            let delta_sgp = (sraw - self.mean_variance_estimator_mean) / 64.0;
            let c = if delta_sgp < 0.0 {
                self.mean_variance_estimator_std - delta_sgp
            } else {
                self.mean_variance_estimator_std + delta_sgp
            };
            let mut additional_scaling = 1.0;
            if c > 1440.0 {
                additional_scaling = c / 1440.0 * (c / 1440.0);
            }
            self.mean_variance_estimator_std =
                sqrtf(additional_scaling * (64.0 - self.mean_variance_estimator_gamma_variance2))
                    * sqrtf(
                        self.mean_variance_estimator_std
                            * (self.mean_variance_estimator_std / (64.0 * additional_scaling))
                            + self.mean_variance_estimator_gamma_variance2 * delta_sgp
                                / additional_scaling
                                * delta_sgp,
                    );
            self.mean_variance_estimator_mean +=
                self.mean_variance_estimator_gamma_mean2 * delta_sgp / 8.0;
        };
    }

    fn mean_variance_estimator_sigmoid_set_parameters(&mut self, x0: f32, k: f32) {
        self.mean_variance_estimator_sigmoid_k = k;
        self.mean_variance_estimator_sigmoid_x0 = x0;
    }

    fn mean_variance_estimator_sigmoid_process(&mut self, sample: f32) -> f32 {
        let x: f32 = self.mean_variance_estimator_sigmoid_k
            * (sample - self.mean_variance_estimator_sigmoid_x0);
        if x < -50.0 {
            1.0
        } else if x > 50.0 {
            0.0
        } else {
            1.0 / (1.0 + expf(x))
        }
    }

    fn mox_model_set_parameters(&mut self, sraw_std: f32, sraw_mean: f32) {
        self.mox_model_sraw_std = sraw_std;
        self.mox_model_sraw_mean = sraw_mean;
    }

    fn mox_model_process(&mut self, sraw: f32) -> f32 {
        match self.algorithm_type {
            AlgorithmType::Nox => (sraw - self.mox_model_sraw_mean) / 2000.0 * self.index_gain,
            AlgorithmType::Voc => {
                (sraw - self.mox_model_sraw_mean) / (-1.0 * (self.mox_model_sraw_std + 220.0))
                    * self.index_gain
            }
        }
    }

    fn sigmoid_scaled_set_parameters(&mut self, x0: f32, k: f32, offset_default: f32) {
        self.sigmoid_scaled_k = k;
        self.sigmoid_scaled_x0 = x0;
        self.sigmoid_scaled_offset_default = offset_default;
    }

    fn sigmoid_scaled_process(&mut self, sample: f32) -> f32 {
        let x = self.sigmoid_scaled_k * (sample - self.sigmoid_scaled_x0);
        if x < -50.0 {
            500.0
        } else if x > 50.0 {
            0.0
        } else if sample >= 0.0 {
            let shift = if self.sigmoid_scaled_offset_default == 1.0 {
                500.0 / 499.0 * (1.0 - self.index_offset)
            } else {
                (500.0 - 5.0 * self.index_offset) / 4.0
            };
            (500.0 + shift) / (1.0 + expf(x)) - shift
        } else {
            self.index_offset / self.sigmoid_scaled_offset_default * (500.0 / (1.0 + expf(x)))
        }
    }

    fn adaptive_lowpass_set_parameters(&mut self) {
        self.adaptive_lowpass_a1 = self.sampling_interval / (20.0 + self.sampling_interval);
        self.adaptive_lowpass_a2 = self.sampling_interval / (500.0 + self.sampling_interval);
        self.adaptive_lowpass_initialized = false;
    }

    fn adaptive_lowpass_process(&mut self, sample: f32) -> f32 {
        if !self.adaptive_lowpass_initialized {
            self.adaptive_lowpass_x1 = sample;
            self.adaptive_lowpass_x2 = sample;
            self.adaptive_lowpass_x3 = sample;
            self.adaptive_lowpass_initialized = true;
        }
        self.adaptive_lowpass_x1 = (1.0 - self.adaptive_lowpass_a1) * self.adaptive_lowpass_x1
            + self.adaptive_lowpass_a1 * sample;
        self.adaptive_lowpass_x2 = (1.0 - self.adaptive_lowpass_a2) * self.adaptive_lowpass_x2
            + self.adaptive_lowpass_a2 * sample;
        let mut abs_delta = self.adaptive_lowpass_x1 - self.adaptive_lowpass_x2;
        if abs_delta < 0.0 {
            abs_delta *= -1.0;
        }
        let f1 = expf(-0.2f32 * abs_delta);
        let tau_a = (500.0 - 20.0) * f1 + 20.0;
        let a3 = self.sampling_interval / (self.sampling_interval + tau_a);
        self.adaptive_lowpass_x3 = (1.0 - a3) * self.adaptive_lowpass_x3 + a3 * sample;
        self.adaptive_lowpass_x3
    }
}

fn expf(value: f32) -> f32 {
    F32Ext::exp(value)
}

fn sqrtf(value: f32) -> f32 {
    F32Ext::sqrt(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn voc_reaches_mean() {
        let mut algo = GasIndexAlgorithm::new(AlgorithmType::Voc, 1.0);
        for _ in 0..200 {
            let _ = algo.process(1337);
        }
        assert_eq!(algo.process(1337), 100);
    }

    #[test]
    fn nox_reaches_mean() {
        let mut algo = GasIndexAlgorithm::new(AlgorithmType::Nox, 1.0);
        for _ in 0..200 {
            let _ = algo.process(1337);
        }
        assert_eq!(algo.process(1337), 1);
    }
}
