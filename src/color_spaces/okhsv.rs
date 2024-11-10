// use crate::F;

// struct Lab {
//     L: F,
//     a: F,
//     b: F,
// }
// struct RGB {
//     r: F,
//     g: F,
//     b: F,
// }
// struct HSV {
//     h: F,
//     s: F,
//     v: F,
// }
// struct HSL {
//     h: F,
//     s: F,
//     l: F,
// }
// struct LC {
//     L: F,
//     C: F,
// }

// // Alternative representation of (L_cusp, C_cusp)
// // Encoded so S = C_cusp/L_cusp and T = C_cusp/(1-L_cusp)
// // The maximum value for C in the triangle is then found as fmin(S*L, T*(1-L)), for a given L
// struct ST {
//     s: F,
//     t: F,
// }

// // Finds the maximum saturation possible for a given hue that fits in sRGB
// // Saturation here is defined as S = C/L
// // a and b must be normalized so a^2 + b^2 == 1
// fn compute_max_saturation(a: F, b: F) -> F {
//     // Max saturation will be when one of r, g or b goes below zero.

//     // Select different coefficients depending on which component goes below zero first
//     let (k0, k1, k2, k3, k4, wl, wm, ws);

//     if -1.88170328 * a - 0.80936493 * b > 1.0 {
//         // Red component
//         k0 = 1.19086277;
//         k1 = 1.76576728;
//         k2 = 0.59662641;
//         k3 = 0.75515197;
//         k4 = 0.56771245;
//         wl = 4.0767416621;
//         wm = -3.3077115913;
//         ws = 0.2309699292;
//     } else if 1.81444104 * a - 1.19445276 * b > 1.0 {
//         // Green component
//         k0 = 0.73956515;
//         k1 = -0.45954404;
//         k2 = 0.08285427;
//         k3 = 0.12541070;
//         k4 = 0.14503204;
//         wl = -1.2684380046;
//         wm = 2.6097574011;
//         ws = -0.3413193965;
//     } else {
//         // Blue component
//         k0 = 1.35733652;
//         k1 = -0.00915799;
//         k2 = -1.15130210;
//         k3 = -0.50559606;
//         k4 = 0.00692167;
//         wl = -0.0041960863;
//         wm = -0.7034186147;
//         ws = 1.7076147010;
//     }

//     // Approximate max saturation using a polynomial:
//     let S = k0 + k1 * a + k2 * b + k3 * a * a + k4 * a * b;

//     // Do one step Halley's method to get closer
//     // this gives an error less than 10e6, except for some blue hues where the dS/dh is close to infinite
//     // this should be sufficient for most applications, otherwise do two/three steps

//     let k_l = 0.3963377774 * a + 0.2158037573 * b;
//     let k_m = -0.1055613458 * a - 0.0638541728 * b;
//     let k_s = -0.0894841775 * a - 1.2914855480 * b;

//     let l_ = 1.0 + S * k_l;
//     let m_ = 1.0 + S * k_m;
//     let s_ = 1.0 + S * k_s;

//     let l = l_ * l_ * l_;
//     let m = m_ * m_ * m_;
//     let s = s_ * s_ * s_;

//     let l_dS = 3.0 * k_l * l_ * l_;
//     let m_dS = 3.0 * k_m * m_ * m_;
//     let s_dS = 3.0 * k_s * s_ * s_;

//     let l_dS2 = 6.0 * k_l * k_l * l_;
//     let m_dS2 = 6.0 * k_m * k_m * m_;
//     let s_dS2 = 6.0 * k_s * k_s * s_;

//     let f = wl * l + wm * m + ws * s;
//     let f1 = wl * l_dS + wm * m_dS + ws * s_dS;
//     let f2 = wl * l_dS2 + wm * m_dS2 + ws * s_dS2;

//     S - f * f1 / (f1 * f1 - 0.5 * f * f2)
// }

// // finds L_cusp and C_cusp for a given hue
// // a and b must be normalized so a^2 + b^2 == 1
// fn  find_cusp( a:F ,  b:F)->LC {
// 	// First, find the maximum saturation (saturation S = C/L)
// 	 let S_cusp = compute_max_saturation(a, b);

// 	// Convert to linear sRGB to find the first point where at least one of r,g or b >= 1:
// 	let rgb_at_max = oklab_to_linear_srgb({ 1, S_cusp * a, S_cusp * b });
// 	float L_cusp = cbrtf(1.f / fmax(fmax(rgb_at_max.r, rgb_at_max.g), rgb_at_max.b));
// 	float C_cusp = L_cusp * S_cusp;

// 	return { L_cusp , C_cusp };
// }

// // ST to_ST(LC cusp)
// // {
// // 	float L = cusp.L;
// // 	float C = cusp.C;
// // 	return { C / L, C / (1 - L) };
// // }

// // float toe_inv(float x)
// // {
// // 	constexpr float k_1 = 0.206f;
// // 	constexpr float k_2 = 0.03f;
// // 	constexpr float k_3 = (1.f + k_1) / (1.f + k_2);
// // 	return (x * x + k_1 * x) / (k_3 * (x + k_2));
// // }

// // RGB okhsv_to_srgb(HSV hsv)
// // {
// // 	float h = hsv.h;
// // 	float s = hsv.s;
// // 	float v = hsv.v;

// // 	float a_ = cosf(2.f * pi * h);
// // 	float b_ = sinf(2.f * pi * h);

// // 	LC cusp = find_cusp(a_, b_);
// // 	ST ST_max = to_ST(cusp);
// // 	float S_max = ST_max.S;
// // 	float T_max = ST_max.T;
// // 	float S_0 = 0.5f;
// // 	float k = 1 - S_0 / S_max;

// // 	// first we compute L and V as if the gamut is a perfect triangle:

// // 	// L, C when v==1:
// // 	float L_v = 1     - s * S_0 / (S_0 + T_max - T_max * k * s);
// // 	float C_v = s * T_max * S_0 / (S_0 + T_max - T_max * k * s);

// // 	float L = v * L_v;
// // 	float C = v * C_v;

// // 	// then we compensate for both toe and the curved top part of the triangle:
// // 	float L_vt = toe_inv(L_v);
// // 	float C_vt = C_v * L_vt / L_v;

// // 	float L_new = toe_inv(L);
// // 	C = C * L_new / L;
// // 	L = L_new;

// // 	RGB rgb_scale = oklab_to_linear_srgb({ L_vt, a_ * C_vt, b_ * C_vt });
// // 	float scale_L = cbrtf(1.f / fmax(fmax(rgb_scale.r, rgb_scale.g), fmax(rgb_scale.b, 0.f)));

// // 	L = L * scale_L;
// // 	C = C * scale_L;

// // 	RGB rgb = oklab_to_linear_srgb({ L, C * a_, C * b_ });
// // 	return {
// // 		srgb_transfer_function(rgb.r),
// // 		srgb_transfer_function(rgb.g),
// // 		srgb_transfer_function(rgb.b),
// // 	};
// // }
