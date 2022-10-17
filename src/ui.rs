mod inner {
    //! inner mechanisms for UI.
    use bevy::prelude::*;

    use crate::typedef::component::{MainCamera2D, MainCamera3D};

    pub fn use_2d_camera (mut commands: Commands, q_2d: Query<Entity, With<MainCamera2D>>, q_3d: Query<Entity, With<MainCamera3D>>) {
        for x in q_3d.iter() {
            commands.entity(x).despawn_recursive()
        }
        if q_2d.is_empty() {
            commands.spawn_bundle(Camera2dBundle::default()).insert(MainCamera2D {});
        }
    }

    pub fn use_3d_camera (mut commands: Commands, q_2d: Query<Entity, With<MainCamera2D>>, q_3d: Query<Entity, With<MainCamera3D>>) {
        for x in q_2d.iter() {
            commands.entity(x).despawn_recursive()
        }
        if q_3d.is_empty() {
            commands.spawn_bundle(Camera3dBundle::default()).insert(MainCamera3D {});
        }
    }
}

pub mod top {
    //! UI definitions for toppage
    use bevy::{prelude::*, ui::FocusPolicy};

    use crate::{constants::style::*, typedef::{component::*, event::*, state::*}};

    use super::inner::*;

    /// Enter systems on the top page.
    pub fn top_systems_enter() -> SystemSet {
        return SystemSet::on_enter(AppState::Top).with_system(use_2d_camera).with_system(top_enter);
    }

    /// Update systems on the top page.
    pub fn top_systems_update() -> SystemSet {
        return SystemSet::on_update(AppState::Top).with_system(top_button_update_system);
    }

    /// Exit systems on the top page.
    pub fn top_systems_exit() -> SystemSet {
        return SystemSet::on_exit(AppState::Top).with_system(top_exit);
    }

    fn top_exit (mut q: Query<Entity, With<TopPageContents>>, mut com: Commands) {
        for i in q.iter() {
            com.entity(i).despawn_recursive();
        }
    }

    fn top_enter(mut commands: Commands, asset_server: Res<AssetServer>) {
        
        commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_wrap: FlexWrap::Wrap,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            let tags = vec![TopPageButton::Explore, TopPageButton::Linear, TopPageButton::NewPage, TopPageButton::Migrate];
            for tag in tags {
                let btnstyle = Style {
                    size: Size::new(Val::Percent(50.0), Val::Percent(50.0)),
                    position: UiRect::all(Val::Auto),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                };
            
                let txtstyle = TextStyle {
                    font: asset_server.load("NotoSansJP-Bold.otf"),
                    font_size: 40.0,
                    color: TOPBTN_TEXT_COLOR,
                };
                
                parent
                .spawn_bundle(ButtonBundle {
                    style: btnstyle.clone(),
                    color: TOPBTN_NORMAL.clone().into(),
                    ..default()
                })
                .with_children(|parent| {

                    let image = match tag {
                        TopPageButton::NewPage => asset_server.load("newpage.png").into(),
                        TopPageButton::Explore => asset_server.load("explore.png").into(),
                        TopPageButton::Linear => asset_server.load("linear.png").into(),
                        TopPageButton::Migrate => asset_server.load("migrate.png").into(),
                    };

                    parent.spawn_bundle(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                            position: UiRect{ left: Val::Percent(0.0), top: Val::Percent(0.0), ..default() },
                            position_type: PositionType::Absolute,
                            
                            ..default()
                        },
                        color: TOPBTN_IMG_OVERLAY.into(),
                        focus_policy: FocusPolicy::Pass,
                        image,
                        ..default()
                    });

                    parent.spawn_bundle(TextBundle::from_section(
                        "", 
                        txtstyle.clone()
                    ));
                }).insert(tag);
            }
        }).insert(TopPageContents {});
    }

    fn top_button_update_system(
        mut interaction_query: Query<
            (&Interaction, &Children, &TopPageButton, Entity),
            (Changed<Interaction>, With<TopPageButton>),
        >,
        mut text_query: Query<&mut Text>,
        mut app_state: ResMut<State<AppState>>,
        mut color_query: Query<&mut UiColor>
    ) {
        for (interaction, children, toppage, ent) in &mut interaction_query {
            let mut text = text_query.get_mut(children[1]).unwrap();

            let label = match toppage {
                TopPageButton::NewPage => ("NewPage"),
                TopPageButton::Explore => ("Explore"),
                TopPageButton::Linear => ("Linear"),
                TopPageButton::Migrate => ("Migrate"),
            };
            let st = match toppage {
                TopPageButton::NewPage => AppState::NewPage,
                TopPageButton::Explore => AppState::Explore,
                TopPageButton::Linear => AppState::Linear,
                TopPageButton::Migrate => AppState::Mitigate,
            };
    
            text.sections[0].value = label.to_string();
    
            let mut color = color_query.get_mut(ent).unwrap();
            match *interaction {
                Interaction::Clicked => {
                    app_state.set(st);
                }
                Interaction::Hovered => {
                    *color = TOPBTN_HOVER.into();
                }
                Interaction::None => {
                    *color = TOPBTN_NORMAL.into();
                }
            }

            let mut im = color_query.get_mut(children[0]).unwrap();
        }
    }
}

pub mod newpage {
    //! UI defenitions for newpage
    use bevy::prelude::*;

    use crate::{typedef::{state::AppState, component::NewPageContents}, constants::style::ICON_BACKGROUND_COLOR};

    use super::inner::*;

    pub fn newpage_systems_enter () -> SystemSet {
        return SystemSet::on_enter(AppState::NewPage).with_system(use_2d_camera).with_system(newpage_enter);
    }

    pub fn newpage_systems_exit () -> SystemSet {
        return SystemSet::on_exit(AppState::NewPage).with_system(newpage_exit);
    }

    pub fn newpage_systems_update () -> SystemSet {
        return SystemSet::on_update(AppState::NewPage).with_system(newpage_update);
    }

    fn newpage_enter (mut com: Commands) {
        com.spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                // auto position
                position: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            color: ICON_BACKGROUND_COLOR.into(),
            ..default()
        }).insert(NewPageContents {});
    }

    fn newpage_exit (mut q: Query<Entity, With<NewPageContents>>, mut com: Commands) {
        for i in q.iter() {
            com.entity(i).despawn_recursive();
        }
    }

    fn newpage_update () {

    }
}

pub mod explore {
    //! UI definitions for explore
    use bevy::prelude::*;
    use crate::typedef::{state::AppState, component::Fragment};

    use super::inner::use_3d_camera;

    pub fn explore_systems_enter () -> SystemSet {
        return SystemSet::on_enter(AppState::Explore).with_system(use_3d_camera).with_system(explore_enter);
    }

    pub fn explore_systems_exit () -> SystemSet {
        return SystemSet::on_enter(AppState::Explore).with_system(explore_exit);
    }

    pub fn explore_systems_update () -> SystemSet {
        return SystemSet::on_enter(AppState::Explore).with_system(explore_update);
    }

    fn explore_enter (q: Query<&Fragment>) {
        
    }

    fn explore_exit () {

    }

    fn explore_update () {

    }
}