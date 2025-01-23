use kurbo::{BezPath, CubicBez};

pub(super) fn bezpath_to_cubics(path: BezPath) -> Vec<CubicBez> {
    path.segments().map(|segment| segment.to_cubic()).collect()
}
