/// Default tolerance to flatten a bezier into a polyline.
///
/// Effectively it's the distance from the polyline to bezier.
///
/// A lower value means:
/// - A closer fit to the bezier.
/// - More segments to the polyline
///
/// 10 mm is an acceptable architectural tolerance.
pub const FLATTEN_TOLERANCE: f32 = 0.010;

/// Accuracy of the bezier created by offset.
///
/// The unit is not specific but more detail is in the blog post:
/// <https://raphlinus.github.io/curves/2022/09/09/parallel-beziers.html>
///
/// A lower value means:
/// - A closer fit to the source bezier.
/// - More segments to the bezier
///
/// `1.0` seems to be good enough for our use but this may need to be revisted.
pub const OFFSET_ACCURACY: f32 = 1.0;

/// Accuracy used for length calculation.
///
/// Effectively the resulting length will be accurate to the value.
///
/// 1 mm is an acceptable accuracy for architectural lengths.
pub const LENGTH_ACCURACY: f32 = 0.001;

/// Tolerance used when flattening a bezier into a polyline in order to determine intersections.
///
/// Effectively the resulting intersection will be accurate to the value.
///
/// 10 mm is an acceptable architectural tolerance.
pub const INTERSECTION_TOLERANCE: f32 = 0.010;

/// Accuracy used when determining where to split a bezier so that it's close to the point of
/// intersection.
///
/// The unit is not specifically defined. A lower value is presumed to give a more accurate result.
///
/// 10 mm is an acceptable architectural tolerance.
pub const INTERSECTION_ACCURACY: f32 = 0.010;
