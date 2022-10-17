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

pub mod manage {
    use bevy::prelude::*;

    use crate::typedef::{event::*, state::AppState};

    pub fn ui_manage_systems () -> bevy::prelude::SystemSet {
        return SystemSet::new().with_system(event_listener)
    }

    fn event_listener (
        mut app_state: ResMut<State<AppState>>,
        mut ev_newpage: EventReader<JumpToNewPage>,
        mut ev_explore: EventReader<JumpToExplore>,
        mut ev_linear: EventReader<JumpToLinear>,
        mut ev_migrate: EventReader<JumpToMigrate>,
        mut ev_top: EventReader<JumpToTop>,
    ) {
        for x in ev_newpage.iter() {
            app_state.set(AppState::NewPage).expect("couldn't change state");
        }
        for x in ev_explore.iter() {
            app_state.set(AppState::Explore).expect("couldn't change state");
        }
        for x in ev_linear.iter() {
            app_state.set(AppState::Linear).expect("couldn't change state");
        }
        for x in ev_migrate.iter() {
            app_state.set(AppState::Migrate).expect("couldn't change state");
        }
        for x in ev_top.iter() {
            app_state.set(AppState::Top).expect("couldn't change state");
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

    fn top_exit (q: Query<Entity, With<TopPageContents>>, mut com: Commands) {
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
            let tags = [TopPageButton::Explore, TopPageButton::Linear, TopPageButton::NewPage, TopPageButton::Migrate];
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

                    let label = match tag {
                        TopPageButton::NewPage => "NewPage",
                        TopPageButton::Explore => "Explore",
                        TopPageButton::Linear => "Linear",
                        TopPageButton::Migrate => "Migrate",
                    };

                    parent.spawn_bundle(TextBundle::from_section(
                        label.to_string(),
                        TextStyle {
                            font: asset_server.load("NotoSansJP-Bold.otf"),
                            font_size: 40.0,
                            color: TOPBTN_TEXT_COLOR,
                        }
                    ));
                }).insert(tag);
            }
        }).insert(TopPageContents {});
    }

    fn top_button_update_system(
        mut interaction_query: Query<
            (&Interaction, &Children, &TopPageButton, &mut UiColor),
            (Changed<Interaction>),
        >,
        mut text_query: Query<&mut Text>,
        mut ev_newpage: EventWriter<JumpToNewPage>,
        mut ev_explore: EventWriter<JumpToExplore>,
        mut ev_linear: EventWriter<JumpToLinear>,
        mut ev_migrate: EventWriter<JumpToMigrate>,
        com: Commands
    ) {
        for (interaction, children, toppage, mut color) in &mut interaction_query {
            let mut text = text_query.get_mut(children[1]).unwrap();
    
            match *interaction {
                Interaction::Clicked => match toppage {
                    TopPageButton::NewPage => ev_newpage.send(JumpToNewPage::default()),
                    TopPageButton::Explore => ev_explore.send(JumpToExplore::default()),
                    TopPageButton::Linear => ev_linear.send(JumpToLinear::default()),
                    TopPageButton::Migrate => ev_migrate.send(JumpToMigrate::default()),
                }
                Interaction::Hovered => {
                    *color = TOPBTN_HOVER.into();
                }
                Interaction::None => {
                    *color = TOPBTN_NORMAL.into();
                }
            }
        }
    }
}

pub mod newpage {
    //! UI defenitions for newpage
    use bevy::{prelude::*, ui::FocusPolicy};
    use bevy_egui::EguiContext;

    use crate::{typedef::{state::AppState, component::{NewPageContents, NewPageButton}, event::{JumpToTop, JumpToNewPage}, resource::{GameState, NewPageState}}, constants::style::*};

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

    fn newpage_enter (mut com: Commands, mut ev_newpage: EventReader<JumpToNewPage>, asset_server: Res<AssetServer>, mut global: ResMut<GameState>) {
        for ev in ev_newpage.iter() {
            global.newpage_state = Some(NewPageState {
                page_entry_id: ev.entry_id,
            });
        }
        com.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(20.0), Val::Percent(100.0)),
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::FlexEnd,
                align_content: AlignContent::FlexEnd,
                flex_wrap: FlexWrap::Wrap,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        }).with_children(|parent|{
            let base_w = 100.0;
            let base_h = 20.0;

            let tags = [NewPageButton::Return, NewPageButton::Edit, NewPageButton::AddTexts];

            for tag in tags {
                parent.spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Percent(base_w), Val::Percent(base_h)),
                        // auto position
                        position: UiRect::all(Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    color: NEWPAGE_NORMAL.into(),
                    ..default()
                }).with_children(|parent| {

                    let image = match tag {
                        NewPageButton::Return => asset_server.load("newpage.png").into(),
                        NewPageButton::AddTexts => asset_server.load("newpage.png").into(),
                        NewPageButton::Edit => asset_server.load("newpage.png").into(),
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
                        match tag {
                            NewPageButton::Return => "Return",
                            NewPageButton::AddTexts => "Add Texts",
                            NewPageButton::Edit => "Edit",
                        }, 
                        TextStyle {
                            font: asset_server.load("NotoSansJP-Bold.otf"),
                            font_size: 40.0,
                            color: TOPBTN_TEXT_COLOR,
                        }
                    ));
                }).insert(tag);
            }
        }).insert(NewPageContents {});
    }

    fn newpage_exit (q: Query<Entity, With<NewPageContents>>, mut com: Commands, mut global: ResMut<GameState>) {
        global.newpage_state = None;
        for i in q.iter() {
            com.entity(i).despawn_recursive();
        }
    }

    fn newpage_update (
        mut egui_ctx: ResMut<EguiContext>,
        window: Res<Windows>,
        mut interaction_query: Query<
            (&Interaction, &Children, &NewPageButton, &mut UiColor),
            (Changed<Interaction>),
        >,
        mut text_query: Query<&mut Text>,
        mut ev_top: EventWriter<JumpToTop>,
    ) {
        for (inter, child, btn_attr, mut color) in interaction_query.iter_mut() {
            match *inter {
                Interaction::Clicked => {
                    *color = NEWPAGE_CLICK.into();
                    match btn_attr {
                        NewPageButton::Return => {
                            ev_top.send(JumpToTop {})
                        },
                        NewPageButton::AddTexts => {
                            
                        },
                        NewPageButton::Edit => {

                        }
                    };
                },
                Interaction::Hovered => {
                    *color = NEWPAGE_HOVER.into();
                },
                Interaction::None => {
                    *color = NEWPAGE_NORMAL.into();
                },
            }
        }
        
        let w = window.get_primary().unwrap();
        egui::SidePanel::right("right_panel")
            .resizable(false)
            .min_width(w.width() * 0.8)
            .max_width(w.width() * 0.8)
        .show(egui_ctx.ctx_mut(), |ui| {
            
        });
    }


}

pub mod migrate {
    //! UI definitions for explore
    use bevy::prelude::*;
    use crate::typedef::{state::AppState};

    use super::inner::use_2d_camera;

    pub fn migrate_systems_enter () -> SystemSet {
        return SystemSet::on_enter(AppState::Migrate).with_system(use_2d_camera).with_system(migrate_enter);
    }

    pub fn migrate_systems_exit () -> SystemSet {
        return SystemSet::on_enter(AppState::Migrate).with_system(migrate_exit);
    }

    pub fn migrate_systems_update () -> SystemSet {
        return SystemSet::on_enter(AppState::Migrate).with_system(migrate_update);
    }

    fn migrate_enter () {
        
    }

    fn migrate_exit () {

    }

    fn migrate_update () {

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