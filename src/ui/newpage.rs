
//! UI defenitions for newpage
use bevy::{prelude::*, ui::FocusPolicy};
use bevy_egui::EguiContext;
use serde::__private::de;

use crate::{typedef::{state::AppState, component::{NewPageContents, NewPageButton, FragmentContents}, event::{JumpToTop, JumpToNewPage}, resource::{GameState, NewPageState, FragmentClone}}, constants::style::*};

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
            ..default()
        });
        if let Some(entry_id) = ev.entry_id {
            // TODO write initial "syncing" code here ("pull" from the database to entry_clone)
        }
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

        let tags = [NewPageButton::Return, NewPageButton::Save, NewPageButton::AddTexts];

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
                    NewPageButton::Save => asset_server.load("newpage.png").into(),
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
                        NewPageButton::Save => "Save",
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
    mut global: ResMut<GameState>
) {
    let mut newpage_state = global.newpage_state.as_mut().unwrap();
    for (inter, child, btn_attr, mut color) in interaction_query.iter_mut() {
        match *inter {
            Interaction::Clicked => {
                *color = NEWPAGE_CLICK.into();
                match btn_attr {
                    NewPageButton::Return => {
                        ev_top.send(JumpToTop {})
                    },
                    NewPageButton::AddTexts => {
                        newpage_state.entry_clone.push(FragmentClone::Modified {
                            contents: FragmentContents::TextData { data: "".to_string() }
                        });
                    },
                    NewPageButton::Save => {
                        
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
        for fc in newpage_state.entry_clone.iter_mut() {
            match fc {
                FragmentClone::NotModified { fragment_id } => {
                    // TODO: copy existing fragment and do something like watching
                },
                FragmentClone::Modified { contents } => {
                    match contents {
                        FragmentContents::TextData { data } => {
                            ui.text_edit_multiline(data);
                        },
                        FragmentContents::Code { data, language } => {},
                        FragmentContents::URL { data } => {},
                        FragmentContents::Image { data } => {},
                    }
                },
            }
        }
    });
}

