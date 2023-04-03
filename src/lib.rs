#![no_std]
//#![forbid(unsafe_code)]
//#![deny(warnings, clippy::all)]

// TODO search/replace once done
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

// TODO
fn expf(_: f32) -> f32 {
    todo!()
}
fn sqrtf(_: f32) -> f32 {
    todo!()
}

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
    pub fn new(mAlgorithm_Type: AlgorithmType, sampling_interval: f32) -> Self {
        let mut state = GasIndexAlgorithmParams {
            mAlgorithm_Type,
            mSamplingInterval: 0.0,
            mIndex_Offset: 0.0,
            mSraw_Minimum: 0,
            mGating_Max_Duration_Minutes: 0.0,
            mInit_Duration_Mean: 0.0,
            mInit_Duration_Variance: 0.0,
            mGating_Threshold: 0.0,
            mIndex_Gain: 0.0,
            mTau_Mean_Hours: 0.0,
            mTau_Variance_Hours: 0.0,
            mSraw_Std_Initial: 0.0,
            mUptime: 0.0,
            mSraw: 0.0,
            mGas_Index: 0.0,
            m_Mean_Variance_Estimator___Initialized: false,
            m_Mean_Variance_Estimator___Mean: 0.0,
            m_Mean_Variance_Estimator___Sraw_Offset: 0.0,
            m_Mean_Variance_Estimator___Std: 0.0,
            m_Mean_Variance_Estimator___Gamma_Mean: 0.0,
            m_Mean_Variance_Estimator___Gamma_Variance: 0.0,
            m_Mean_Variance_Estimator___Gamma_Initial_Mean: 0.0,
            m_Mean_Variance_Estimator___Gamma_Initial_Variance: 0.0,
            m_Mean_Variance_Estimator__Gamma_Mean: 0.0,
            m_Mean_Variance_Estimator__Gamma_Variance: 0.0,
            m_Mean_Variance_Estimator___Uptime_Gamma: 0.0,
            m_Mean_Variance_Estimator___Uptime_Gating: 0.0,
            m_Mean_Variance_Estimator___Gating_Duration_Minutes: 0.0,
            m_Mean_Variance_Estimator___Sigmoid__K: 0.0,
            m_Mean_Variance_Estimator___Sigmoid__X0: 0.0,
            m_Mox_Model__Sraw_Std: 0.0,
            m_Mox_Model__Sraw_Mean: 0.0,
            m_Sigmoid_Scaled__K: 0.0,
            m_Sigmoid_Scaled__X0: 0.0,
            m_Sigmoid_Scaled__Offset_Default: 0.0,
            m_Adaptive_Lowpass__A1: 0.0,
            m_Adaptive_Lowpass__A2: 0.0,
            m_Adaptive_Lowpass___Initialized: false,
            m_Adaptive_Lowpass___X1: 0.0,
            m_Adaptive_Lowpass___X2: 0.0,
            m_Adaptive_Lowpass___X3: 0.0,
        };

        state.init_with_sampling_interval(mAlgorithm_Type, sampling_interval);

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
    pub mAlgorithm_Type: AlgorithmType,
    pub mSamplingInterval: f32,
    pub mIndex_Offset: f32,
    pub mSraw_Minimum: i32,
    pub mGating_Max_Duration_Minutes: f32,
    pub mInit_Duration_Mean: f32,
    pub mInit_Duration_Variance: f32,
    pub mGating_Threshold: f32,
    pub mIndex_Gain: f32,
    pub mTau_Mean_Hours: f32,
    pub mTau_Variance_Hours: f32,
    pub mSraw_Std_Initial: f32,
    pub mUptime: f32,
    pub mSraw: f32,
    pub mGas_Index: f32,
    pub m_Mean_Variance_Estimator___Initialized: bool,
    pub m_Mean_Variance_Estimator___Mean: f32,
    pub m_Mean_Variance_Estimator___Sraw_Offset: f32,
    pub m_Mean_Variance_Estimator___Std: f32,
    pub m_Mean_Variance_Estimator___Gamma_Mean: f32,
    pub m_Mean_Variance_Estimator___Gamma_Variance: f32,
    pub m_Mean_Variance_Estimator___Gamma_Initial_Mean: f32,
    pub m_Mean_Variance_Estimator___Gamma_Initial_Variance: f32,
    pub m_Mean_Variance_Estimator__Gamma_Mean: f32,
    pub m_Mean_Variance_Estimator__Gamma_Variance: f32,
    pub m_Mean_Variance_Estimator___Uptime_Gamma: f32,
    pub m_Mean_Variance_Estimator___Uptime_Gating: f32,
    pub m_Mean_Variance_Estimator___Gating_Duration_Minutes: f32,
    pub m_Mean_Variance_Estimator___Sigmoid__K: f32,
    pub m_Mean_Variance_Estimator___Sigmoid__X0: f32,
    pub m_Mox_Model__Sraw_Std: f32,
    pub m_Mox_Model__Sraw_Mean: f32,
    pub m_Sigmoid_Scaled__K: f32,
    pub m_Sigmoid_Scaled__X0: f32,
    pub m_Sigmoid_Scaled__Offset_Default: f32,
    pub m_Adaptive_Lowpass__A1: f32,
    pub m_Adaptive_Lowpass__A2: f32,
    pub m_Adaptive_Lowpass___Initialized: bool,
    pub m_Adaptive_Lowpass___X1: f32,
    pub m_Adaptive_Lowpass___X2: f32,
    pub m_Adaptive_Lowpass___X3: f32,
}

impl GasIndexAlgorithmParams {
    fn init_with_sampling_interval(
        &mut self,
        algorithm_type: AlgorithmType,
        sampling_interval: f32,
    ) {
        self.mAlgorithm_Type = algorithm_type;
        self.mSamplingInterval = sampling_interval;
        match self.mAlgorithm_Type {
            AlgorithmType::Nox => {
                self.mIndex_Offset = 1.0f32;
                self.mSraw_Minimum = 10000;
                self.mGating_Max_Duration_Minutes = 60.0f32 * 12.0f32;
                self.mInit_Duration_Mean = 3600.0f32 * 4.75f32;
                self.mInit_Duration_Variance = 3600.0f32 * 5.70f32;
                self.mGating_Threshold = 30.0f32;
            }
            AlgorithmType::Voc => {
                self.mIndex_Offset = 100.0f32;
                self.mSraw_Minimum = 20000;
                self.mGating_Max_Duration_Minutes = 60.0f32 * 3.0f32;
                self.mInit_Duration_Mean = 3600.0f32 * 0.75f32;
                self.mInit_Duration_Variance = 3600.0f32 * 1.45f32;
                self.mGating_Threshold = 340.0f32;
            }
        }
        self.mIndex_Gain = 230.0f32;
        self.mTau_Mean_Hours = 12.0f32;
        self.mTau_Variance_Hours = 12.0f32;
        self.mSraw_Std_Initial = 50.0f32;
        self.reset();
    }

    fn init(&mut self, algorithm_type: AlgorithmType) {
        self.init_with_sampling_interval(algorithm_type, 1.0f32);
    }

    fn reset(&mut self) {
        self.mUptime = 0.0;
        self.mSraw = 0.0;
        self.mGas_Index = 0.0;
        self.init_instances();
    }

    fn init_instances(&mut self) {
        self.mean_variance_estimator__set_parameters();
        let sraw_std = self.mean_variance_estimator__get_std();
        let sraw_mean = self.mean_variance_estimator__get_mean();
        self.mox_model__set_parameters(sraw_std, sraw_mean);
        match self.mAlgorithm_Type {
            AlgorithmType::Nox => {
                self.sigmoid_scaled__set_parameters(614.0f32, -0.0101f32, 1.0f32);
            }
            AlgorithmType::Voc => {
                self.sigmoid_scaled__set_parameters(213.0f32, -0.0065f32, 100.0f32);
            }
        }
        self.adaptive_lowpass__set_parameters();
    }

    fn process(&mut self, mut sraw: i32) -> i32 {
        if self.mUptime <= 45.0f32 {
            self.mUptime = self.mUptime + self.mSamplingInterval;
        } else {
            if sraw > 0 && sraw < 65000 {
                if sraw < self.mSraw_Minimum + 1 {
                    sraw = self.mSraw_Minimum + 1;
                } else if sraw > self.mSraw_Minimum + 32767 {
                    sraw = self.mSraw_Minimum + 32767;
                }
                self.mSraw = (sraw - self.mSraw_Minimum) as f32;
            }
            if self.mAlgorithm_Type == AlgorithmType::Voc
                || self.mean_variance_estimator__is_initialized()
            {
                self.mGas_Index = self.mox_model__process(self.mSraw);
                self.mGas_Index = self.sigmoid_scaled__process(self.mGas_Index);
            } else {
                self.mGas_Index = self.mIndex_Offset;
            }
            self.mGas_Index = self.adaptive_lowpass__process(self.mGas_Index);
            if self.mGas_Index < 0.5f32 {
                self.mGas_Index = 0.5f32;
            }
            if self.mSraw > 0.0f32 {
                self.mean_variance_estimator__process(self.mSraw);

                let sraw_std = self.mean_variance_estimator__get_std();
                let sraw_mean = self.mean_variance_estimator__get_mean();
                self.mox_model__set_parameters(sraw_std, sraw_mean);
            }
        }
        (self.mGas_Index + 0.5f32) as i32
    }

    fn mean_variance_estimator__set_parameters(&mut self) {
        self.m_Mean_Variance_Estimator___Initialized = false;
        self.m_Mean_Variance_Estimator___Mean = 0.0f32;
        self.m_Mean_Variance_Estimator___Sraw_Offset = 0.0f32;
        self.m_Mean_Variance_Estimator___Std = self.mSraw_Std_Initial;
        self.m_Mean_Variance_Estimator___Gamma_Mean =
            8.0f32 * 64.0f32 * (self.mSamplingInterval / 3600.0f32)
                / (self.mTau_Mean_Hours + self.mSamplingInterval / 3600.0f32);
        self.m_Mean_Variance_Estimator___Gamma_Variance = 64.0f32
            * (self.mSamplingInterval / 3600.0f32)
            / (self.mTau_Variance_Hours + self.mSamplingInterval / 3600.0f32);
        match self.mAlgorithm_Type {
            AlgorithmType::Nox => {
                self.m_Mean_Variance_Estimator___Gamma_Initial_Mean =
                    8.0f32 * 64.0f32 * self.mSamplingInterval
                        / (1200.0f32 + self.mSamplingInterval);
            }
            AlgorithmType::Voc => {
                self.m_Mean_Variance_Estimator___Gamma_Initial_Mean =
                    8.0f32 * 64.0f32 * self.mSamplingInterval / (20.0f32 + self.mSamplingInterval);
            }
        }
        self.m_Mean_Variance_Estimator___Gamma_Initial_Variance =
            64.0f32 * self.mSamplingInterval / (2500.0f32 + self.mSamplingInterval);
        self.m_Mean_Variance_Estimator__Gamma_Mean = 0.0f32;
        self.m_Mean_Variance_Estimator__Gamma_Variance = 0.0f32;
        self.m_Mean_Variance_Estimator___Uptime_Gamma = 0.0f32;
        self.m_Mean_Variance_Estimator___Uptime_Gating = 0.0f32;
        self.m_Mean_Variance_Estimator___Gating_Duration_Minutes = 0.0f32;
    }

    fn mean_variance_estimator__set_states(
        &mut self,
        mut mean: f32,
        mut std: f32,
        mut uptime_gamma: f32,
    ) {
        self.m_Mean_Variance_Estimator___Mean = mean;
        self.m_Mean_Variance_Estimator___Std = std;
        self.m_Mean_Variance_Estimator___Uptime_Gamma = uptime_gamma;
        self.m_Mean_Variance_Estimator___Initialized = true;
    }

    fn mean_variance_estimator__get_std(&mut self) -> f32 {
        self.m_Mean_Variance_Estimator___Std
    }

    fn mean_variance_estimator__get_mean(&mut self) -> f32 {
        self.m_Mean_Variance_Estimator___Mean + self.m_Mean_Variance_Estimator___Sraw_Offset
    }

    fn mean_variance_estimator__is_initialized(&mut self) -> bool {
        self.m_Mean_Variance_Estimator___Initialized
    }

    fn mean_variance_estimator___calculate_gamma(&mut self) {
        let mut uptime_limit: f32 = 0.;
        let mut sigmoid_gamma_mean: f32 = 0.;
        let mut gamma_mean: f32 = 0.;
        let mut gating_threshold_mean: f32 = 0.;
        let mut sigmoid_gating_mean: f32 = 0.;
        let mut sigmoid_gamma_variance: f32 = 0.;
        let mut gamma_variance: f32 = 0.;
        let mut gating_threshold_variance: f32 = 0.;
        let mut sigmoid_gating_variance: f32 = 0.;
        uptime_limit = 32767.0f32 - self.mSamplingInterval;
        if self.m_Mean_Variance_Estimator___Uptime_Gamma < uptime_limit {
            self.m_Mean_Variance_Estimator___Uptime_Gamma =
                self.m_Mean_Variance_Estimator___Uptime_Gamma + self.mSamplingInterval;
        }
        if self.m_Mean_Variance_Estimator___Uptime_Gating < uptime_limit {
            self.m_Mean_Variance_Estimator___Uptime_Gating =
                self.m_Mean_Variance_Estimator___Uptime_Gating + self.mSamplingInterval;
        }
        self.mean_variance_estimator___sigmoid__set_parameters(self.mInit_Duration_Mean, 0.01f32);
        sigmoid_gamma_mean = self.mean_variance_estimator___sigmoid__process(
            self.m_Mean_Variance_Estimator___Uptime_Gamma,
        );
        gamma_mean = self.m_Mean_Variance_Estimator___Gamma_Mean
            + (self.m_Mean_Variance_Estimator___Gamma_Initial_Mean
                - self.m_Mean_Variance_Estimator___Gamma_Mean)
                * sigmoid_gamma_mean;
        gating_threshold_mean = self.mGating_Threshold
            + (510.0f32 - self.mGating_Threshold)
                * self.mean_variance_estimator___sigmoid__process(
                    self.m_Mean_Variance_Estimator___Uptime_Gating,
                );
        self.mean_variance_estimator___sigmoid__set_parameters(gating_threshold_mean, 0.09f32);
        sigmoid_gating_mean = self.mean_variance_estimator___sigmoid__process(self.mGas_Index);
        self.m_Mean_Variance_Estimator__Gamma_Mean = sigmoid_gating_mean * gamma_mean;
        self.mean_variance_estimator___sigmoid__set_parameters(
            self.mInit_Duration_Variance,
            0.01f32,
        );
        sigmoid_gamma_variance = self.mean_variance_estimator___sigmoid__process(
            self.m_Mean_Variance_Estimator___Uptime_Gamma,
        );
        gamma_variance = self.m_Mean_Variance_Estimator___Gamma_Variance
            + (self.m_Mean_Variance_Estimator___Gamma_Initial_Variance
                - self.m_Mean_Variance_Estimator___Gamma_Variance)
                * (sigmoid_gamma_variance - sigmoid_gamma_mean);
        gating_threshold_variance = self.mGating_Threshold
            + (510.0f32 - self.mGating_Threshold)
                * self.mean_variance_estimator___sigmoid__process(
                    self.m_Mean_Variance_Estimator___Uptime_Gating,
                );
        self.mean_variance_estimator___sigmoid__set_parameters(gating_threshold_variance, 0.09f32);
        sigmoid_gating_variance = self.mean_variance_estimator___sigmoid__process(self.mGas_Index);
        self.m_Mean_Variance_Estimator__Gamma_Variance = sigmoid_gating_variance * gamma_variance;
        self.m_Mean_Variance_Estimator___Gating_Duration_Minutes = self
            .m_Mean_Variance_Estimator___Gating_Duration_Minutes
            + self.mSamplingInterval / 60.0f32
                * ((1.0f32 - sigmoid_gating_mean) * (1.0f32 + 0.3f32) - 0.3f32);
        if self.m_Mean_Variance_Estimator___Gating_Duration_Minutes < 0.0f32 {
            self.m_Mean_Variance_Estimator___Gating_Duration_Minutes = 0.0f32;
        }
        if self.m_Mean_Variance_Estimator___Gating_Duration_Minutes
            > self.mGating_Max_Duration_Minutes
        {
            self.m_Mean_Variance_Estimator___Uptime_Gating = 0.0f32;
        }
    }

    fn mean_variance_estimator__process(&mut self, mut sraw: f32) {
        let mut delta_sgp: f32 = 0.;
        let mut c: f32 = 0.;
        let mut additional_scaling: f32 = 0.;
        if !self.m_Mean_Variance_Estimator___Initialized {
            self.m_Mean_Variance_Estimator___Initialized = true;
            self.m_Mean_Variance_Estimator___Sraw_Offset = sraw;
            self.m_Mean_Variance_Estimator___Mean = 0.0f32;
        } else {
            if self.m_Mean_Variance_Estimator___Mean >= 100.0f32
                || self.m_Mean_Variance_Estimator___Mean <= -100.0f32
            {
                self.m_Mean_Variance_Estimator___Sraw_Offset = self
                    .m_Mean_Variance_Estimator___Sraw_Offset
                    + self.m_Mean_Variance_Estimator___Mean;
                self.m_Mean_Variance_Estimator___Mean = 0.0f32;
            }
            sraw = sraw - self.m_Mean_Variance_Estimator___Sraw_Offset;
            self.mean_variance_estimator___calculate_gamma();
            delta_sgp = (sraw - self.m_Mean_Variance_Estimator___Mean) / 64.0f32;
            if delta_sgp < 0.0f32 {
                c = self.m_Mean_Variance_Estimator___Std - delta_sgp;
            } else {
                c = self.m_Mean_Variance_Estimator___Std + delta_sgp;
            }
            additional_scaling = 1.0f32;
            if c > 1440.0f32 {
                additional_scaling = c / 1440.0f32 * (c / 1440.0f32);
            }
            self.m_Mean_Variance_Estimator___Std = sqrtf(
                additional_scaling * (64.0f32 - self.m_Mean_Variance_Estimator__Gamma_Variance),
            ) * sqrtf(
                self.m_Mean_Variance_Estimator___Std
                    * (self.m_Mean_Variance_Estimator___Std / (64.0f32 * additional_scaling))
                    + self.m_Mean_Variance_Estimator__Gamma_Variance * delta_sgp
                        / additional_scaling
                        * delta_sgp,
            );
            self.m_Mean_Variance_Estimator___Mean = self.m_Mean_Variance_Estimator___Mean
                + self.m_Mean_Variance_Estimator__Gamma_Mean * delta_sgp / 8.0f32;
        };
    }

    fn mean_variance_estimator___sigmoid__set_parameters(&mut self, mut X0: f32, mut K: f32) {
        self.m_Mean_Variance_Estimator___Sigmoid__K = K;
        self.m_Mean_Variance_Estimator___Sigmoid__X0 = X0;
    }

    fn mean_variance_estimator___sigmoid__process(&mut self, mut sample: f32) -> f32 {
        let x: f32 = self.m_Mean_Variance_Estimator___Sigmoid__K
            * (sample - self.m_Mean_Variance_Estimator___Sigmoid__X0);
        if x < -50.0f32 {
            1.0f32
        } else if x > 50.0f32 {
            0.0f32
        } else {
            1.0f32 / (1.0f32 + expf(x))
        }
    }

    fn mox_model__set_parameters(&mut self, SRAW_STD: f32, SRAW_MEAN: f32) {
        self.m_Mox_Model__Sraw_Std = SRAW_STD;
        self.m_Mox_Model__Sraw_Mean = SRAW_MEAN;
    }

    fn mox_model__process(&mut self, sraw: f32) -> f32 {
        match self.mAlgorithm_Type {
            AlgorithmType::Nox => {
                (sraw - self.m_Mox_Model__Sraw_Mean) / 2000.0f32 * self.mIndex_Gain
            }
            AlgorithmType::Voc => {
                (sraw - self.m_Mox_Model__Sraw_Mean)
                    / (-1.0f32 * (self.m_Mox_Model__Sraw_Std + 220.0f32))
                    * self.mIndex_Gain
            }
        }
    }

    fn sigmoid_scaled__set_parameters(&mut self, X0: f32, K: f32, offset_default: f32) {
        self.m_Sigmoid_Scaled__K = K;
        self.m_Sigmoid_Scaled__X0 = X0;
        self.m_Sigmoid_Scaled__Offset_Default = offset_default;
    }

    fn sigmoid_scaled__process(&mut self, sample: f32) -> f32 {
        let mut x: f32 = 0.;
        let mut shift: f32 = 0.;
        x = self.m_Sigmoid_Scaled__K * (sample - self.m_Sigmoid_Scaled__X0);
        if x < -50.0f32 {
            return 500.0f32;
        } else if x > 50.0f32 {
            return 0.0f32;
        } else if sample >= 0.0f32 {
            if self.m_Sigmoid_Scaled__Offset_Default == 1.0f32 {
                shift = 500.0f32 / 499.0f32 * (1.0f32 - self.mIndex_Offset);
            } else {
                shift = (500.0f32 - 5.0f32 * self.mIndex_Offset) / 4.0f32;
            }
            return (500.0f32 + shift) / (1.0f32 + expf(x)) - shift;
        } else {
            return self.mIndex_Offset / self.m_Sigmoid_Scaled__Offset_Default
                * (500.0f32 / (1.0f32 + expf(x)));
        };
    }

    fn adaptive_lowpass__set_parameters(&mut self) {
        self.m_Adaptive_Lowpass__A1 = self.mSamplingInterval / (20.0f32 + self.mSamplingInterval);
        self.m_Adaptive_Lowpass__A2 = self.mSamplingInterval / (500.0f32 + self.mSamplingInterval);
        self.m_Adaptive_Lowpass___Initialized = false;
    }

    fn adaptive_lowpass__process(&mut self, mut sample: f32) -> f32 {
        let mut abs_delta: f32 = 0.;
        let mut F1: f32 = 0.;
        let mut tau_a: f32 = 0.;
        let mut a3: f32 = 0.;
        if !self.m_Adaptive_Lowpass___Initialized {
            self.m_Adaptive_Lowpass___X1 = sample;
            self.m_Adaptive_Lowpass___X2 = sample;
            self.m_Adaptive_Lowpass___X3 = sample;
            self.m_Adaptive_Lowpass___Initialized = true;
        }
        self.m_Adaptive_Lowpass___X1 = (1.0f32 - self.m_Adaptive_Lowpass__A1)
            * self.m_Adaptive_Lowpass___X1
            + self.m_Adaptive_Lowpass__A1 * sample;
        self.m_Adaptive_Lowpass___X2 = (1.0f32 - self.m_Adaptive_Lowpass__A2)
            * self.m_Adaptive_Lowpass___X2
            + self.m_Adaptive_Lowpass__A2 * sample;
        abs_delta = self.m_Adaptive_Lowpass___X1 - self.m_Adaptive_Lowpass___X2;
        if abs_delta < 0.0f32 {
            abs_delta = -1.0f32 * abs_delta;
        }
        F1 = expf(-0.2f32 * abs_delta);
        tau_a = (500.0f32 - 20.0f32) * F1 + 20.0f32;
        a3 = self.mSamplingInterval / (self.mSamplingInterval + tau_a);
        self.m_Adaptive_Lowpass___X3 = (1.0f32 - a3) * self.m_Adaptive_Lowpass___X3 + a3 * sample;
        return self.m_Adaptive_Lowpass___X3;
    }
}
