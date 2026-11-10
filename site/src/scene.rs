//! L'assiette 3D, un petit jeu `paquets::ecs` : des ingrédients (tomates, œufs, fromage,
//! carottes, champignons, petits pois…) tombent du ciel dans une assiette, rebondissent,
//! roulent et s'empilent grâce à la physique d'Active. Toucher l'assiette en fait tomber
//! un de plus à cet endroit ; glisser fait tourner la caméra.

use std::f32::consts::{PI, TAU};

use paquets::ecs::prelude::*;

/// Un ingrédient tombé.
struct Falling;

/// Les éléments qui flottent autour de l'assiette (couverts, étincelles).
struct Floating {
    phase: f32,
    speed: f32,
    base: Vec3,
}

paquets::active::component!(Falling, Floating);

/// Une forme d'ingrédient : son maillage, sa couleur, et son échelle.
#[derive(Clone)]
struct Kind {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    scale: Vec3,
}

/// Les ingrédients possibles, et un générateur de nombres.
struct Pantry {
    kinds: Vec<Kind>,
    seed: u64,
    /// Le temps avant le prochain ingrédient.
    next_drop: f32,
}

impl Pantry {
    fn next(&mut self) -> u64 {
        // xorshift64* : assez pour choisir des formes et des positions.
        self.seed ^= self.seed >> 12;
        self.seed ^= self.seed << 25;
        self.seed ^= self.seed >> 27;
        self.seed.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// Un nombre entre -1 et 1.
    fn signed(&mut self) -> f32 {
        (self.next() % 2001) as f32 / 1000.0 - 1.0
    }
}

/// L'orbite de la caméra et le glissement en cours.
#[derive(Default)]
struct Orbit {
    angle: f32,
    dragged: f32,
    /// `prefers-reduced-motion` : rien ne bouge tout seul.
    still: bool,
    /// Un écran étroit (téléphone) : la caméra recule pour garder l'assiette entière.
    narrow: bool,
}

/// Le compteur affiché sous la scène.
struct Counter(paquets::active::State<usize>);

paquets::active::resource!(Pantry, Orbit, Counter);

const PLATE: f32 = 3.2;
const MAX_FALLING: usize = 36;

fn material(
    materials: &mut Assets<StandardMaterial>,
    color: Color,
    roughness: f32,
    metallic: f32,
) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        base_color: color,
        perceptual_roughness: roughness,
        metallic,
        ..default()
    })
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // L'assiette : un disque épais (le corps, qui porte les ingrédients), un fond plus
    // clair et un liseré doré.
    let porcelain = material(&mut materials, Color::srgb_u8(246, 242, 236), 0.25, 0.0);
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(PLATE, 0.24))),
        MeshMaterial3d(porcelain),
        Transform::from_xyz(0.0, -0.12, 0.0),
        RigidBody::Static,
        Friction(0.8),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Torus::new(PLATE - 0.12, PLATE + 0.06))),
        MeshMaterial3d(material(
            &mut materials,
            Color::srgb_u8(232, 186, 92),
            0.2,
            0.9,
        )),
        Transform::from_xyz(0.0, 0.02, 0.0),
        RigidBody::Static,
        Collider::cylinder(0.01, 0.01),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(PLATE * 0.62, 0.02))),
        MeshMaterial3d(material(
            &mut materials,
            Color::srgb_u8(255, 251, 246),
            0.5,
            0.0,
        )),
        Transform::from_xyz(0.0, 0.01, 0.0),
        RigidBody::Static,
        Collider::cylinder(0.01, 0.01),
    ));

    // Les ingrédients.
    let kinds = vec![
        // Tomate
        Kind {
            mesh: meshes.add(Sphere::new(0.42).mesh().uv(32, 18)),
            material: material(&mut materials, Color::srgb_u8(235, 64, 52), 0.3, 0.0),
            scale: Vec3::new(1.0, 0.86, 1.0),
        },
        // Œuf
        Kind {
            mesh: meshes.add(Sphere::new(0.3).mesh().uv(28, 16)),
            material: material(&mut materials, Color::srgb_u8(250, 236, 214), 0.45, 0.0),
            scale: Vec3::new(1.0, 1.3, 1.0),
        },
        // Fromage
        Kind {
            mesh: meshes.add(Cuboid::new(0.7, 0.42, 0.5)),
            material: material(&mut materials, Color::srgb_u8(255, 199, 44), 0.55, 0.0),
            scale: Vec3::ONE,
        },
        // Carotte
        Kind {
            mesh: meshes.add(Cone::new(0.2, 0.95)),
            material: material(&mut materials, Color::srgb_u8(255, 128, 31), 0.5, 0.0),
            scale: Vec3::ONE,
        },
        // Champignon
        Kind {
            mesh: meshes.add(Cylinder::new(0.32, 0.3)),
            material: material(&mut materials, Color::srgb_u8(196, 160, 128), 0.7, 0.0),
            scale: Vec3::ONE,
        },
        // Petit pois
        Kind {
            mesh: meshes.add(Sphere::new(0.16).mesh().ico(2)),
            material: material(&mut materials, Color::srgb_u8(110, 200, 80), 0.35, 0.0),
            scale: Vec3::ONE,
        },
        // Rondelle de concombre
        Kind {
            mesh: meshes.add(Cylinder::new(0.36, 0.1)),
            material: material(&mut materials, Color::srgb_u8(120, 190, 110), 0.4, 0.0),
            scale: Vec3::ONE,
        },
        // Oignon
        Kind {
            mesh: meshes.add(Torus::new(0.14, 0.38)),
            material: material(&mut materials, Color::srgb_u8(214, 140, 210), 0.3, 0.1),
            scale: Vec3::ONE,
        },
    ];

    // Des étincelles dorées qui flottent autour (sans physique).
    let spark = meshes.add(Sphere::new(0.07).mesh().ico(1));
    let glow = materials.add(StandardMaterial {
        base_color: Color::srgb_u8(255, 214, 120),
        emissive: LinearRgba::rgb(3.0, 2.0, 0.6),
        unlit: true,
        ..default()
    });
    for i in 0..14 {
        let a = i as f32 / 14.0 * TAU;
        let r = PLATE + 0.9 + (i % 3) as f32 * 0.5;
        let base = Vec3::new(r * a.cos(), 0.8 + (i % 4) as f32 * 0.7, r * a.sin());
        commands.spawn((
            Mesh3d(spark),
            MeshMaterial3d(glow),
            Transform::from_translation(base),
            RigidBody::Kinematic,
            Collider::sphere(0.001),
            Floating {
                phase: a,
                speed: 0.6 + (i % 5) as f32 * 0.15,
                base,
            },
        ));
    }

    // Les lumières : une chaude, une froide, et le soleil qui donne les ombres.
    commands.spawn((
        PointLight {
            color: Color::srgb_u8(255, 150, 90),
            intensity: 2_200_000.0,
            range: 30.0,
            ..default()
        },
        Transform::from_xyz(4.0, 7.0, 4.0),
    ));
    commands.spawn((
        PointLight {
            color: Color::srgb_u8(120, 200, 255),
            intensity: 1_100_000.0,
            range: 30.0,
            ..default()
        },
        Transform::from_xyz(-5.0, 4.0, -3.0),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 2_200.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(2.0, 8.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    // Le canvas est transparent : le fond de la page passe derrière.
    commands.spawn((
        Camera3d::default(),
        Camera {
            clear_color: ClearColorConfig::None,
            ..default()
        },
        Transform::from_xyz(0.0, 6.0, 10.0).looking_at(Vec3::new(0.0, 0.6, 0.0), Vec3::Y),
    ));
    commands.insert_resource(Pantry {
        kinds,
        seed: 0x9E37_79B9_7F4A_7C15,
        next_drop: 0.0,
    });
}

/// Fait tomber un ingrédient au-dessus de `at` (sur l'assiette).
fn drop_one(
    commands: &mut Commands,
    pantry: &mut Pantry,
    at: Vec3,
    falling: &Query<Entity, With<Falling>>,
) {
    // Trop d'ingrédients : le plus ancien s'en va.
    if falling.count() >= MAX_FALLING
        && let Some(oldest) = falling.iter().next()
    {
        commands.entity(oldest).despawn();
    }
    let index = (pantry.next() % pantry.kinds.len() as u64) as usize;
    let kind = pantry.kinds[index].clone();
    let turn = pantry.signed() * PI;
    let tilt = pantry.signed() * 0.6;
    commands.spawn((
        Mesh3d(kind.mesh),
        MeshMaterial3d(kind.material),
        Transform::from_xyz(at.x, 6.5 + pantry.signed().abs() * 1.5, at.z)
            .with_rotation(Quat::from_rotation_y(turn) * Quat::from_rotation_x(tilt))
            .with_scale(kind.scale),
        AngularVelocity(Vec3::new(tilt * 5.0, pantry.signed() * 2.0, -tilt * 4.0)),
        Restitution(0.45),
        Friction(0.6),
        Falling,
    ));
}

/// La pluie d'ingrédients.
fn rain(
    mut commands: Commands,
    time: Res<Time>,
    orbit: Res<Orbit>,
    mut pantry: ResMut<Pantry>,
    falling: Query<Entity, With<Falling>>,
) {
    if orbit.still && falling.count() >= 12 {
        return;
    }
    pantry.next_drop -= time.delta_secs();
    if pantry.next_drop > 0.0 {
        return;
    }
    // Vite au début (l'assiette se remplit), puis tranquillement.
    let n = falling.count();
    pantry.next_drop = if orbit.still {
        0.0
    } else if n < 14 {
        0.18
    } else {
        0.9
    };
    let r = pantry.signed().abs().sqrt() * (PLATE - 0.6);
    let a = pantry.signed() * PI;
    drop_one(
        &mut commands,
        &mut pantry,
        Vec3::new(r * a.cos(), 0.0, r * a.sin()),
        &falling,
    );
}

/// Les étincelles flottent.
fn float(time: Res<Time>, orbit: Res<Orbit>, mut sparks: Query<(&mut Transform, &Floating)>) {
    if orbit.still {
        return;
    }
    let t = time.elapsed_secs();
    for (mut transform, f) in &mut sparks {
        let a = f.phase + t * 0.25 * f.speed;
        let r = Vec3::new(f.base.x, 0.0, f.base.z).length();
        transform.translation = Vec3::new(
            r * a.cos(),
            f.base.y + 0.35 * (t * f.speed * 1.7 + f.phase).sin(),
            r * a.sin(),
        );
    }
}

/// Glisser fait tourner la caméra ; toucher sans glisser fait tomber un ingrédient.
#[allow(clippy::too_many_arguments)]
fn pointer(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
    time: Res<Time>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform, &mut Transform)>,
    mut orbit: ResMut<Orbit>,
    mut pantry: ResMut<Pantry>,
    falling: Query<Entity, With<Falling>>,
) {
    let (camera, at, mut transform) = camera.into_inner();
    if buttons.just_pressed(MouseButton::Left) {
        orbit.dragged = 0.0;
    }
    if buttons.pressed(MouseButton::Left) {
        orbit.dragged += motion.delta.length();
        orbit.angle -= motion.delta.x * 0.008;
    } else if !orbit.still {
        orbit.angle += 0.1 * time.delta_secs();
    }
    if buttons.just_released(MouseButton::Left) && orbit.dragged < 6.0 {
        let target = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(&at, cursor).ok())
            .and_then(|ray| {
                Some(ray.get_point(ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y))?))
            })
            .filter(|p| p.length() < PLATE);
        if let Some(point) = target {
            drop_one(&mut commands, &mut pantry, point, &falling);
        }
    }
    // Un écran en portrait est étroit : la caméra recule et monte un peu.
    orbit.narrow = window.resolution.width < window.resolution.height;
    let (distance, height) = if orbit.narrow {
        (13.5, 8.0)
    } else {
        (10.0, 6.0)
    };
    let eye = Vec3::new(
        distance * orbit.angle.sin(),
        height,
        distance * orbit.angle.cos(),
    );
    *transform = Transform::from_translation(eye).looking_at(Vec3::new(0.0, 0.6, 0.0), Vec3::Y);
}

/// Ce qui est tombé de l'assiette s'en va.
fn fallen(mut commands: Commands, falling: Query<(Entity, &Transform), With<Falling>>) {
    for (entity, transform) in &falling {
        if transform.translation.y < -8.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn count(counter: Res<Counter>, falling: Query<Entity, With<Falling>>) {
    let n = falling.count();
    if counter.0.is_alive() && counter.0.get() != n {
        counter.0.set(n);
    }
}

/// Le jeu, prêt à tourner (dans un canvas, ou dans un test avec `app.update()`).
pub fn app(counter: paquets::active::State<usize>, still: bool) -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .insert_resource(Orbit {
            still,
            ..Orbit::default()
        })
        .insert_resource(Counter(counter))
        .add_systems(Startup, setup)
        .add_systems(Update, (rain, float, pointer, fallen, count));
    app
}

/// Lance la scène dans le canvas ; elle s'arrête quand la poignée est lâchée.
#[cfg(target_arch = "wasm32")]
pub fn start(
    canvas: &paquets::active::Dom,
    counter: paquets::active::State<usize>,
) -> Option<Running> {
    use wasm_bindgen::JsCast;
    let running = match app(counter, crate::client::reduced_motion()).run_in(canvas) {
        Ok(running) => running,
        Err(error) => {
            web_sys::console::warn_1(&error.into());
            return None;
        }
    };
    // Le moteur prend tous les gestes du canvas (`touch-action: none`) : sur un téléphone,
    // on garde le défilement vertical de la page.
    if let Ok(el) = canvas.clone().dyn_into::<web_sys::HtmlElement>() {
        let _ = el.style().set_property("touch-action", "pan-y");
    }
    Some(running)
}
