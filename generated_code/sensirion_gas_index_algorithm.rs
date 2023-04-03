use ::libc;
extern "C" {
    fn expf(_: libc::c_float) -> libc::c_float;
    fn sqrtf(_: libc::c_float) -> libc::c_float;
}
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GasIndexAlgorithmParams {
    pub mAlgorithm_Type: libc::c_int,
    pub mSamplingInterval: libc::c_float,
    pub mIndex_Offset: libc::c_float,
    pub mSraw_Minimum: int32_t,
    pub mGating_Max_Duration_Minutes: libc::c_float,
    pub mInit_Duration_Mean: libc::c_float,
    pub mInit_Duration_Variance: libc::c_float,
    pub mGating_Threshold: libc::c_float,
    pub mIndex_Gain: libc::c_float,
    pub mTau_Mean_Hours: libc::c_float,
    pub mTau_Variance_Hours: libc::c_float,
    pub mSraw_Std_Initial: libc::c_float,
    pub mUptime: libc::c_float,
    pub mSraw: libc::c_float,
    pub mGas_Index: libc::c_float,
    pub m_Mean_Variance_Estimator___Initialized: bool,
    pub m_Mean_Variance_Estimator___Mean: libc::c_float,
    pub m_Mean_Variance_Estimator___Sraw_Offset: libc::c_float,
    pub m_Mean_Variance_Estimator___Std: libc::c_float,
    pub m_Mean_Variance_Estimator___Gamma_Mean: libc::c_float,
    pub m_Mean_Variance_Estimator___Gamma_Variance: libc::c_float,
    pub m_Mean_Variance_Estimator___Gamma_Initial_Mean: libc::c_float,
    pub m_Mean_Variance_Estimator___Gamma_Initial_Variance: libc::c_float,
    pub m_Mean_Variance_Estimator__Gamma_Mean: libc::c_float,
    pub m_Mean_Variance_Estimator__Gamma_Variance: libc::c_float,
    pub m_Mean_Variance_Estimator___Uptime_Gamma: libc::c_float,
    pub m_Mean_Variance_Estimator___Uptime_Gating: libc::c_float,
    pub m_Mean_Variance_Estimator___Gating_Duration_Minutes: libc::c_float,
    pub m_Mean_Variance_Estimator___Sigmoid__K: libc::c_float,
    pub m_Mean_Variance_Estimator___Sigmoid__X0: libc::c_float,
    pub m_Mox_Model__Sraw_Std: libc::c_float,
    pub m_Mox_Model__Sraw_Mean: libc::c_float,
    pub m_Sigmoid_Scaled__K: libc::c_float,
    pub m_Sigmoid_Scaled__X0: libc::c_float,
    pub m_Sigmoid_Scaled__Offset_Default: libc::c_float,
    pub m_Adaptive_Lowpass__A1: libc::c_float,
    pub m_Adaptive_Lowpass__A2: libc::c_float,
    pub m_Adaptive_Lowpass___Initialized: bool,
    pub m_Adaptive_Lowpass___X1: libc::c_float,
    pub m_Adaptive_Lowpass___X2: libc::c_float,
    pub m_Adaptive_Lowpass___X3: libc::c_float,
}
#[no_mangle]
pub unsafe extern "C" fn GasIndexAlgorithm_init_with_sampling_interval(
    mut params: *mut GasIndexAlgorithmParams,
    mut algorithm_type: int32_t,
    mut sampling_interval: libc::c_float,
) {
    (*params).mAlgorithm_Type = algorithm_type;
    (*params).mSamplingInterval = sampling_interval;
    if algorithm_type == 1 as libc::c_int {
        (*params).mIndex_Offset = 1.0f32;
        (*params).mSraw_Minimum = 10000 as libc::c_int;
        (*params).mGating_Max_Duration_Minutes = 60.0f32 * 12.0f32;
        (*params).mInit_Duration_Mean = 3600.0f32 * 4.75f32;
        (*params).mInit_Duration_Variance = 3600.0f32 * 5.70f32;
        (*params).mGating_Threshold = 30.0f32;
    } else {
        (*params).mIndex_Offset = 100.0f32;
        (*params).mSraw_Minimum = 20000 as libc::c_int;
        (*params).mGating_Max_Duration_Minutes = 60.0f32 * 3.0f32;
        (*params).mInit_Duration_Mean = 3600.0f32 * 0.75f32;
        (*params).mInit_Duration_Variance = 3600.0f32 * 1.45f32;
        (*params).mGating_Threshold = 340.0f32;
    }
    (*params).mIndex_Gain = 230.0f32;
    (*params).mTau_Mean_Hours = 12.0f32;
    (*params).mTau_Variance_Hours = 12.0f32;
    (*params).mSraw_Std_Initial = 50.0f32;
    GasIndexAlgorithm_reset(params);
}
#[no_mangle]
pub unsafe extern "C" fn GasIndexAlgorithm_init(
    mut params: *mut GasIndexAlgorithmParams,
    mut algorithm_type: int32_t,
) {
    GasIndexAlgorithm_init_with_sampling_interval(params, algorithm_type, 1.0f32);
}
#[no_mangle]
pub unsafe extern "C" fn GasIndexAlgorithm_reset(
    mut params: *mut GasIndexAlgorithmParams,
) {
    (*params).mUptime = 0.0f32;
    (*params).mSraw = 0.0f32;
    (*params).mGas_Index = 0 as libc::c_int as libc::c_float;
    GasIndexAlgorithm__init_instances(params);
}
unsafe extern "C" fn GasIndexAlgorithm__init_instances(
    mut params: *mut GasIndexAlgorithmParams,
) {
    GasIndexAlgorithm__mean_variance_estimator__set_parameters(params);
    GasIndexAlgorithm__mox_model__set_parameters(
        params,
        GasIndexAlgorithm__mean_variance_estimator__get_std(params),
        GasIndexAlgorithm__mean_variance_estimator__get_mean(params),
    );
    if (*params).mAlgorithm_Type == 1 as libc::c_int {
        GasIndexAlgorithm__sigmoid_scaled__set_parameters(
            params,
            614.0f32,
            -0.0101f32,
            1.0f32,
        );
    } else {
        GasIndexAlgorithm__sigmoid_scaled__set_parameters(
            params,
            213.0f32,
            -0.0065f32,
            100.0f32,
        );
    }
    GasIndexAlgorithm__adaptive_lowpass__set_parameters(params);
}
#[no_mangle]
pub unsafe extern "C" fn GasIndexAlgorithm_get_sampling_interval(
    mut params: *const GasIndexAlgorithmParams,
    mut sampling_interval: *mut libc::c_float,
) {
    *sampling_interval = (*params).mSamplingInterval;
}
#[no_mangle]
pub unsafe extern "C" fn GasIndexAlgorithm_get_states(
    mut params: *const GasIndexAlgorithmParams,
    mut state0: *mut libc::c_float,
    mut state1: *mut libc::c_float,
) {
    *state0 = GasIndexAlgorithm__mean_variance_estimator__get_mean(params);
    *state1 = GasIndexAlgorithm__mean_variance_estimator__get_std(params);
}
#[no_mangle]
pub unsafe extern "C" fn GasIndexAlgorithm_set_states(
    mut params: *mut GasIndexAlgorithmParams,
    mut state0: libc::c_float,
    mut state1: libc::c_float,
) {
    GasIndexAlgorithm__mean_variance_estimator__set_states(
        params,
        state0,
        state1,
        3.0f32 * 3600.0f32,
    );
    GasIndexAlgorithm__mox_model__set_parameters(
        params,
        GasIndexAlgorithm__mean_variance_estimator__get_std(params),
        GasIndexAlgorithm__mean_variance_estimator__get_mean(params),
    );
    (*params).mSraw = state0;
}
#[no_mangle]
pub unsafe extern "C" fn GasIndexAlgorithm_set_tuning_parameters(
    mut params: *mut GasIndexAlgorithmParams,
    mut index_offset: int32_t,
    mut learning_time_offset_hours: int32_t,
    mut learning_time_gain_hours: int32_t,
    mut gating_max_duration_minutes: int32_t,
    mut std_initial: int32_t,
    mut gain_factor: int32_t,
) {
    (*params).mIndex_Offset = index_offset as libc::c_float;
    (*params).mTau_Mean_Hours = learning_time_offset_hours as libc::c_float;
    (*params).mTau_Variance_Hours = learning_time_gain_hours as libc::c_float;
    (*params)
        .mGating_Max_Duration_Minutes = gating_max_duration_minutes as libc::c_float;
    (*params).mSraw_Std_Initial = std_initial as libc::c_float;
    (*params).mIndex_Gain = gain_factor as libc::c_float;
    GasIndexAlgorithm__init_instances(params);
}
#[no_mangle]
pub unsafe extern "C" fn GasIndexAlgorithm_get_tuning_parameters(
    mut params: *const GasIndexAlgorithmParams,
    mut index_offset: *mut int32_t,
    mut learning_time_offset_hours: *mut int32_t,
    mut learning_time_gain_hours: *mut int32_t,
    mut gating_max_duration_minutes: *mut int32_t,
    mut std_initial: *mut int32_t,
    mut gain_factor: *mut int32_t,
) {
    *index_offset = (*params).mIndex_Offset as int32_t;
    *learning_time_offset_hours = (*params).mTau_Mean_Hours as int32_t;
    *learning_time_gain_hours = (*params).mTau_Variance_Hours as int32_t;
    *gating_max_duration_minutes = (*params).mGating_Max_Duration_Minutes as int32_t;
    *std_initial = (*params).mSraw_Std_Initial as int32_t;
    *gain_factor = (*params).mIndex_Gain as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn GasIndexAlgorithm_process(
    mut params: *mut GasIndexAlgorithmParams,
    mut sraw: int32_t,
    mut gas_index: *mut int32_t,
) {
    if (*params).mUptime <= 45.0f32 {
        (*params).mUptime = (*params).mUptime + (*params).mSamplingInterval;
    } else {
        if sraw > 0 as libc::c_int && sraw < 65000 as libc::c_int {
            if sraw < (*params).mSraw_Minimum + 1 as libc::c_int {
                sraw = (*params).mSraw_Minimum + 1 as libc::c_int;
            } else if sraw > (*params).mSraw_Minimum + 32767 as libc::c_int {
                sraw = (*params).mSraw_Minimum + 32767 as libc::c_int;
            }
            (*params).mSraw = (sraw - (*params).mSraw_Minimum) as libc::c_float;
        }
        if (*params).mAlgorithm_Type == 0 as libc::c_int
            || GasIndexAlgorithm__mean_variance_estimator__is_initialized(params)
                as libc::c_int != 0
        {
            (*params)
                .mGas_Index = GasIndexAlgorithm__mox_model__process(
                params,
                (*params).mSraw,
            );
            (*params)
                .mGas_Index = GasIndexAlgorithm__sigmoid_scaled__process(
                params,
                (*params).mGas_Index,
            );
        } else {
            (*params).mGas_Index = (*params).mIndex_Offset;
        }
        (*params)
            .mGas_Index = GasIndexAlgorithm__adaptive_lowpass__process(
            params,
            (*params).mGas_Index,
        );
        if (*params).mGas_Index < 0.5f32 {
            (*params).mGas_Index = 0.5f32;
        }
        if (*params).mSraw > 0.0f32 {
            GasIndexAlgorithm__mean_variance_estimator__process(params, (*params).mSraw);
            GasIndexAlgorithm__mox_model__set_parameters(
                params,
                GasIndexAlgorithm__mean_variance_estimator__get_std(params),
                GasIndexAlgorithm__mean_variance_estimator__get_mean(params),
            );
        }
    }
    *gas_index = ((*params).mGas_Index + 0.5f32) as int32_t;
}
unsafe extern "C" fn GasIndexAlgorithm__mean_variance_estimator__set_parameters(
    mut params: *mut GasIndexAlgorithmParams,
) {
    (*params).m_Mean_Variance_Estimator___Initialized = 0 as libc::c_int != 0;
    (*params).m_Mean_Variance_Estimator___Mean = 0.0f32;
    (*params).m_Mean_Variance_Estimator___Sraw_Offset = 0.0f32;
    (*params).m_Mean_Variance_Estimator___Std = (*params).mSraw_Std_Initial;
    (*params)
        .m_Mean_Variance_Estimator___Gamma_Mean = 8.0f32 * 64.0f32
        * ((*params).mSamplingInterval / 3600.0f32)
        / ((*params).mTau_Mean_Hours + (*params).mSamplingInterval / 3600.0f32);
    (*params)
        .m_Mean_Variance_Estimator___Gamma_Variance = 64.0f32
        * ((*params).mSamplingInterval / 3600.0f32)
        / ((*params).mTau_Variance_Hours + (*params).mSamplingInterval / 3600.0f32);
    if (*params).mAlgorithm_Type == 1 as libc::c_int {
        (*params)
            .m_Mean_Variance_Estimator___Gamma_Initial_Mean = 8.0f32 * 64.0f32
            * (*params).mSamplingInterval / (1200.0f32 + (*params).mSamplingInterval);
    } else {
        (*params)
            .m_Mean_Variance_Estimator___Gamma_Initial_Mean = 8.0f32 * 64.0f32
            * (*params).mSamplingInterval / (20.0f32 + (*params).mSamplingInterval);
    }
    (*params)
        .m_Mean_Variance_Estimator___Gamma_Initial_Variance = 64.0f32
        * (*params).mSamplingInterval / (2500.0f32 + (*params).mSamplingInterval);
    (*params).m_Mean_Variance_Estimator__Gamma_Mean = 0.0f32;
    (*params).m_Mean_Variance_Estimator__Gamma_Variance = 0.0f32;
    (*params).m_Mean_Variance_Estimator___Uptime_Gamma = 0.0f32;
    (*params).m_Mean_Variance_Estimator___Uptime_Gating = 0.0f32;
    (*params).m_Mean_Variance_Estimator___Gating_Duration_Minutes = 0.0f32;
}
unsafe extern "C" fn GasIndexAlgorithm__mean_variance_estimator__set_states(
    mut params: *mut GasIndexAlgorithmParams,
    mut mean: libc::c_float,
    mut std: libc::c_float,
    mut uptime_gamma: libc::c_float,
) {
    (*params).m_Mean_Variance_Estimator___Mean = mean;
    (*params).m_Mean_Variance_Estimator___Std = std;
    (*params).m_Mean_Variance_Estimator___Uptime_Gamma = uptime_gamma;
    (*params).m_Mean_Variance_Estimator___Initialized = 1 as libc::c_int != 0;
}
unsafe extern "C" fn GasIndexAlgorithm__mean_variance_estimator__get_std(
    mut params: *const GasIndexAlgorithmParams,
) -> libc::c_float {
    return (*params).m_Mean_Variance_Estimator___Std;
}
unsafe extern "C" fn GasIndexAlgorithm__mean_variance_estimator__get_mean(
    mut params: *const GasIndexAlgorithmParams,
) -> libc::c_float {
    return (*params).m_Mean_Variance_Estimator___Mean
        + (*params).m_Mean_Variance_Estimator___Sraw_Offset;
}
unsafe extern "C" fn GasIndexAlgorithm__mean_variance_estimator__is_initialized(
    mut params: *mut GasIndexAlgorithmParams,
) -> bool {
    return (*params).m_Mean_Variance_Estimator___Initialized;
}
unsafe extern "C" fn GasIndexAlgorithm__mean_variance_estimator___calculate_gamma(
    mut params: *mut GasIndexAlgorithmParams,
) {
    let mut uptime_limit: libc::c_float = 0.;
    let mut sigmoid_gamma_mean: libc::c_float = 0.;
    let mut gamma_mean: libc::c_float = 0.;
    let mut gating_threshold_mean: libc::c_float = 0.;
    let mut sigmoid_gating_mean: libc::c_float = 0.;
    let mut sigmoid_gamma_variance: libc::c_float = 0.;
    let mut gamma_variance: libc::c_float = 0.;
    let mut gating_threshold_variance: libc::c_float = 0.;
    let mut sigmoid_gating_variance: libc::c_float = 0.;
    uptime_limit = 32767.0f32 - (*params).mSamplingInterval;
    if (*params).m_Mean_Variance_Estimator___Uptime_Gamma < uptime_limit {
        (*params)
            .m_Mean_Variance_Estimator___Uptime_Gamma = (*params)
            .m_Mean_Variance_Estimator___Uptime_Gamma + (*params).mSamplingInterval;
    }
    if (*params).m_Mean_Variance_Estimator___Uptime_Gating < uptime_limit {
        (*params)
            .m_Mean_Variance_Estimator___Uptime_Gating = (*params)
            .m_Mean_Variance_Estimator___Uptime_Gating + (*params).mSamplingInterval;
    }
    GasIndexAlgorithm__mean_variance_estimator___sigmoid__set_parameters(
        params,
        (*params).mInit_Duration_Mean,
        0.01f32,
    );
    sigmoid_gamma_mean = GasIndexAlgorithm__mean_variance_estimator___sigmoid__process(
        params,
        (*params).m_Mean_Variance_Estimator___Uptime_Gamma,
    );
    gamma_mean = (*params).m_Mean_Variance_Estimator___Gamma_Mean
        + ((*params).m_Mean_Variance_Estimator___Gamma_Initial_Mean
            - (*params).m_Mean_Variance_Estimator___Gamma_Mean) * sigmoid_gamma_mean;
    gating_threshold_mean = (*params).mGating_Threshold
        + (510.0f32 - (*params).mGating_Threshold)
            * GasIndexAlgorithm__mean_variance_estimator___sigmoid__process(
                params,
                (*params).m_Mean_Variance_Estimator___Uptime_Gating,
            );
    GasIndexAlgorithm__mean_variance_estimator___sigmoid__set_parameters(
        params,
        gating_threshold_mean,
        0.09f32,
    );
    sigmoid_gating_mean = GasIndexAlgorithm__mean_variance_estimator___sigmoid__process(
        params,
        (*params).mGas_Index,
    );
    (*params).m_Mean_Variance_Estimator__Gamma_Mean = sigmoid_gating_mean * gamma_mean;
    GasIndexAlgorithm__mean_variance_estimator___sigmoid__set_parameters(
        params,
        (*params).mInit_Duration_Variance,
        0.01f32,
    );
    sigmoid_gamma_variance = GasIndexAlgorithm__mean_variance_estimator___sigmoid__process(
        params,
        (*params).m_Mean_Variance_Estimator___Uptime_Gamma,
    );
    gamma_variance = (*params).m_Mean_Variance_Estimator___Gamma_Variance
        + ((*params).m_Mean_Variance_Estimator___Gamma_Initial_Variance
            - (*params).m_Mean_Variance_Estimator___Gamma_Variance)
            * (sigmoid_gamma_variance - sigmoid_gamma_mean);
    gating_threshold_variance = (*params).mGating_Threshold
        + (510.0f32 - (*params).mGating_Threshold)
            * GasIndexAlgorithm__mean_variance_estimator___sigmoid__process(
                params,
                (*params).m_Mean_Variance_Estimator___Uptime_Gating,
            );
    GasIndexAlgorithm__mean_variance_estimator___sigmoid__set_parameters(
        params,
        gating_threshold_variance,
        0.09f32,
    );
    sigmoid_gating_variance = GasIndexAlgorithm__mean_variance_estimator___sigmoid__process(
        params,
        (*params).mGas_Index,
    );
    (*params)
        .m_Mean_Variance_Estimator__Gamma_Variance = sigmoid_gating_variance
        * gamma_variance;
    (*params)
        .m_Mean_Variance_Estimator___Gating_Duration_Minutes = (*params)
        .m_Mean_Variance_Estimator___Gating_Duration_Minutes
        + (*params).mSamplingInterval / 60.0f32
            * ((1.0f32 - sigmoid_gating_mean) * (1.0f32 + 0.3f32) - 0.3f32);
    if (*params).m_Mean_Variance_Estimator___Gating_Duration_Minutes < 0.0f32 {
        (*params).m_Mean_Variance_Estimator___Gating_Duration_Minutes = 0.0f32;
    }
    if (*params).m_Mean_Variance_Estimator___Gating_Duration_Minutes
        > (*params).mGating_Max_Duration_Minutes
    {
        (*params).m_Mean_Variance_Estimator___Uptime_Gating = 0.0f32;
    }
}
unsafe extern "C" fn GasIndexAlgorithm__mean_variance_estimator__process(
    mut params: *mut GasIndexAlgorithmParams,
    mut sraw: libc::c_float,
) {
    let mut delta_sgp: libc::c_float = 0.;
    let mut c: libc::c_float = 0.;
    let mut additional_scaling: libc::c_float = 0.;
    if (*params).m_Mean_Variance_Estimator___Initialized as libc::c_int
        == 0 as libc::c_int
    {
        (*params).m_Mean_Variance_Estimator___Initialized = 1 as libc::c_int != 0;
        (*params).m_Mean_Variance_Estimator___Sraw_Offset = sraw;
        (*params).m_Mean_Variance_Estimator___Mean = 0.0f32;
    } else {
        if (*params).m_Mean_Variance_Estimator___Mean >= 100.0f32
            || (*params).m_Mean_Variance_Estimator___Mean <= -100.0f32
        {
            (*params)
                .m_Mean_Variance_Estimator___Sraw_Offset = (*params)
                .m_Mean_Variance_Estimator___Sraw_Offset
                + (*params).m_Mean_Variance_Estimator___Mean;
            (*params).m_Mean_Variance_Estimator___Mean = 0.0f32;
        }
        sraw = sraw - (*params).m_Mean_Variance_Estimator___Sraw_Offset;
        GasIndexAlgorithm__mean_variance_estimator___calculate_gamma(params);
        delta_sgp = (sraw - (*params).m_Mean_Variance_Estimator___Mean) / 64.0f32;
        if delta_sgp < 0.0f32 {
            c = (*params).m_Mean_Variance_Estimator___Std - delta_sgp;
        } else {
            c = (*params).m_Mean_Variance_Estimator___Std + delta_sgp;
        }
        additional_scaling = 1.0f32;
        if c > 1440.0f32 {
            additional_scaling = c / 1440.0f32 * (c / 1440.0f32);
        }
        (*params)
            .m_Mean_Variance_Estimator___Std = sqrtf(
            additional_scaling
                * (64.0f32 - (*params).m_Mean_Variance_Estimator__Gamma_Variance),
        )
            * sqrtf(
                (*params).m_Mean_Variance_Estimator___Std
                    * ((*params).m_Mean_Variance_Estimator___Std
                        / (64.0f32 * additional_scaling))
                    + (*params).m_Mean_Variance_Estimator__Gamma_Variance * delta_sgp
                        / additional_scaling * delta_sgp,
            );
        (*params)
            .m_Mean_Variance_Estimator___Mean = (*params)
            .m_Mean_Variance_Estimator___Mean
            + (*params).m_Mean_Variance_Estimator__Gamma_Mean * delta_sgp / 8.0f32;
    };
}
unsafe extern "C" fn GasIndexAlgorithm__mean_variance_estimator___sigmoid__set_parameters(
    mut params: *mut GasIndexAlgorithmParams,
    mut X0: libc::c_float,
    mut K: libc::c_float,
) {
    (*params).m_Mean_Variance_Estimator___Sigmoid__K = K;
    (*params).m_Mean_Variance_Estimator___Sigmoid__X0 = X0;
}
unsafe extern "C" fn GasIndexAlgorithm__mean_variance_estimator___sigmoid__process(
    mut params: *mut GasIndexAlgorithmParams,
    mut sample: libc::c_float,
) -> libc::c_float {
    let mut x: libc::c_float = 0.;
    x = (*params).m_Mean_Variance_Estimator___Sigmoid__K
        * (sample - (*params).m_Mean_Variance_Estimator___Sigmoid__X0);
    if x < -50.0f32 {
        return 1.0f32
    } else if x > 50.0f32 {
        return 0.0f32
    } else {
        return 1.0f32 / (1.0f32 + expf(x))
    };
}
unsafe extern "C" fn GasIndexAlgorithm__mox_model__set_parameters(
    mut params: *mut GasIndexAlgorithmParams,
    mut SRAW_STD: libc::c_float,
    mut SRAW_MEAN: libc::c_float,
) {
    (*params).m_Mox_Model__Sraw_Std = SRAW_STD;
    (*params).m_Mox_Model__Sraw_Mean = SRAW_MEAN;
}
unsafe extern "C" fn GasIndexAlgorithm__mox_model__process(
    mut params: *mut GasIndexAlgorithmParams,
    mut sraw: libc::c_float,
) -> libc::c_float {
    if (*params).mAlgorithm_Type == 1 as libc::c_int {
        return (sraw - (*params).m_Mox_Model__Sraw_Mean) / 2000.0f32
            * (*params).mIndex_Gain
    } else {
        return (sraw - (*params).m_Mox_Model__Sraw_Mean)
            / (-1.0f32 * ((*params).m_Mox_Model__Sraw_Std + 220.0f32))
            * (*params).mIndex_Gain
    };
}
unsafe extern "C" fn GasIndexAlgorithm__sigmoid_scaled__set_parameters(
    mut params: *mut GasIndexAlgorithmParams,
    mut X0: libc::c_float,
    mut K: libc::c_float,
    mut offset_default: libc::c_float,
) {
    (*params).m_Sigmoid_Scaled__K = K;
    (*params).m_Sigmoid_Scaled__X0 = X0;
    (*params).m_Sigmoid_Scaled__Offset_Default = offset_default;
}
unsafe extern "C" fn GasIndexAlgorithm__sigmoid_scaled__process(
    mut params: *mut GasIndexAlgorithmParams,
    mut sample: libc::c_float,
) -> libc::c_float {
    let mut x: libc::c_float = 0.;
    let mut shift: libc::c_float = 0.;
    x = (*params).m_Sigmoid_Scaled__K * (sample - (*params).m_Sigmoid_Scaled__X0);
    if x < -50.0f32 {
        return 500.0f32
    } else if x > 50.0f32 {
        return 0.0f32
    } else if sample >= 0.0f32 {
        if (*params).m_Sigmoid_Scaled__Offset_Default == 1.0f32 {
            shift = 500.0f32 / 499.0f32 * (1.0f32 - (*params).mIndex_Offset);
        } else {
            shift = (500.0f32 - 5.0f32 * (*params).mIndex_Offset) / 4.0f32;
        }
        return (500.0f32 + shift) / (1.0f32 + expf(x)) - shift;
    } else {
        return (*params).mIndex_Offset / (*params).m_Sigmoid_Scaled__Offset_Default
            * (500.0f32 / (1.0f32 + expf(x)))
    };
}
unsafe extern "C" fn GasIndexAlgorithm__adaptive_lowpass__set_parameters(
    mut params: *mut GasIndexAlgorithmParams,
) {
    (*params)
        .m_Adaptive_Lowpass__A1 = (*params).mSamplingInterval
        / (20.0f32 + (*params).mSamplingInterval);
    (*params)
        .m_Adaptive_Lowpass__A2 = (*params).mSamplingInterval
        / (500.0f32 + (*params).mSamplingInterval);
    (*params).m_Adaptive_Lowpass___Initialized = 0 as libc::c_int != 0;
}
unsafe extern "C" fn GasIndexAlgorithm__adaptive_lowpass__process(
    mut params: *mut GasIndexAlgorithmParams,
    mut sample: libc::c_float,
) -> libc::c_float {
    let mut abs_delta: libc::c_float = 0.;
    let mut F1: libc::c_float = 0.;
    let mut tau_a: libc::c_float = 0.;
    let mut a3: libc::c_float = 0.;
    if (*params).m_Adaptive_Lowpass___Initialized as libc::c_int == 0 as libc::c_int {
        (*params).m_Adaptive_Lowpass___X1 = sample;
        (*params).m_Adaptive_Lowpass___X2 = sample;
        (*params).m_Adaptive_Lowpass___X3 = sample;
        (*params).m_Adaptive_Lowpass___Initialized = 1 as libc::c_int != 0;
    }
    (*params)
        .m_Adaptive_Lowpass___X1 = (1.0f32 - (*params).m_Adaptive_Lowpass__A1)
        * (*params).m_Adaptive_Lowpass___X1 + (*params).m_Adaptive_Lowpass__A1 * sample;
    (*params)
        .m_Adaptive_Lowpass___X2 = (1.0f32 - (*params).m_Adaptive_Lowpass__A2)
        * (*params).m_Adaptive_Lowpass___X2 + (*params).m_Adaptive_Lowpass__A2 * sample;
    abs_delta = (*params).m_Adaptive_Lowpass___X1 - (*params).m_Adaptive_Lowpass___X2;
    if abs_delta < 0.0f32 {
        abs_delta = -1.0f32 * abs_delta;
    }
    F1 = expf(-0.2f32 * abs_delta);
    tau_a = (500.0f32 - 20.0f32) * F1 + 20.0f32;
    a3 = (*params).mSamplingInterval / ((*params).mSamplingInterval + tau_a);
    (*params)
        .m_Adaptive_Lowpass___X3 = (1.0f32 - a3) * (*params).m_Adaptive_Lowpass___X3
        + a3 * sample;
    return (*params).m_Adaptive_Lowpass___X3;
}
