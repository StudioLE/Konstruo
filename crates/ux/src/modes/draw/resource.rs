use bevy::prelude::*;
use konstruo_beziers::*;
use konstruo_geometry::*;
use konstruo_paths::*;

#[derive(Resource)]
pub struct DrawMode {
    pub(super) origins: Vec<Vec3>,
    pub(super) handles: Vec<Vec3>,
    pub(super) path: Entity,
    pub(super) control: Entity,
    pub(super) line: Entity,
    pub(super) is_ready: bool,
}

impl DrawMode {
    /// Create a new [`DrawMode`] resource by spawning temporary [`Path`] entities.
    pub(super) fn new(factory: &mut PathFactory) -> Self {
        let spline = CubicBezierSpline::example_2();
        let path = Path::new(spline);
        let path_entity = factory.spawn_path(path.clone());
        factory
            .commands
            .entity(path_entity)
            .insert(Visibility::Hidden);
        for surface in PathSurface::default_surfaces() {
            factory.spawn_surface(surface, &path, path_entity);
        }
        let bundle =
            factory.control_bundle(ControlType::StartHandle, 0, Vec3::ZERO, Visibility::Hidden);
        let control = factory.commands.spawn(bundle).id();
        let line = vec![Vec3::ZERO, Vec3::ONE];
        let bundle = (
            PathControlLine::new(0, true),
            Mesh3d(factory.meshes.add(Polyline::new(line).to_mesh())),
            MeshMaterial3d(factory.materials.control_line.clone()),
            Visibility::Hidden,
        );
        let line = factory.commands.spawn(bundle).id();
        Self {
            origins: Vec::new(),
            handles: Vec::new(),
            path: path_entity,
            control,
            line,
            is_ready: false,
        }
    }
}
