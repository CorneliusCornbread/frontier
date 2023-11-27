use bevy::{
    asset::AssetServer,
    core_pipeline::core_3d::Camera3dBundle,
    ecs::system::{Commands, Res},
    math::Vec3,
    pbr::{
        AmbientLight, CascadeShadowConfigBuilder, DirectionalLight, DirectionalLightBundle,
        EnvironmentMapLight,
    },
    render::color::Color,
    scene::SceneBundle,
    transform::components::Transform,
};

pub fn load_funny_gltf(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(25.0, 6.0, 0.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..Default::default()
    },));

    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..Default::default()
        },

        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..Default::default()
        }
        .into(),
        ..Default::default()
    });

    let my_gltf: bevy::prelude::Handle<bevy::prelude::Scene> =
        assets.load("test_platform.glb#Scene0");
    commands.spawn(SceneBundle {
        scene: my_gltf,
        ..Default::default()
    });
}
