//! UI definitions for newpage

// TODO: "split" functionality

use bevy::{prelude::*, ui::FocusPolicy};
use bevy_egui::EguiContext;
use egui::{TextEdit, Color32};

use crate::{typedef::{state::AppState, component::{NewPageContents, NewPageButton, FragmentContents, Fragment, EntityList}, event::{JumpToTop, JumpToNewPage, SyncFragments, SyncFragmentsDone}, resource::{NewPageState, FragmentClone}}, constants::style::*, utils::utils::{set_default_font, create_timestamp}, journalmanage::systems::handle_sync_fragments};

use super::inner::*;

pub fn newpage_systems_enter () -> SystemSet {
    return SystemSet::on_enter(AppState::NewPage).with_system(use_default_2d_camera).with_system(newpage_enter);
}

pub fn newpage_systems_exit () -> SystemSet {
    return SystemSet::on_exit(AppState::NewPage).with_system(newpage_exit);
}

pub fn newpage_systems_update () -> SystemSet {
    return SystemSet::on_update(AppState::NewPage).with_system(newpage_update).with_system(watch_sync_fragments_done.before(handle_sync_fragments)); // TODO: workaround with this weird hack
}

fn get_initial_state_with_ids (q_list: &Query<&EntityList>, entry_ids: Vec<Entity>) -> NewPageState {
    let entry_clone = entry_ids.iter().map(|entry| {
        match q_list.get(entry.clone()) {
            Ok(lst) => lst.entities.iter()
                .map(|x| FragmentClone::NotModified { fragment_id: x.clone() })
                .collect::<Vec<_>>(),
            Err(_) => vec![],
        }
    }).collect::<Vec<_>>().concat();

    NewPageState {
        page_entry_ids: entry_ids,
        entry_clone
    }
}

fn watch_sync_fragments_done (mut ev_sync: EventReader<SyncFragmentsDone>, q_list: Query<&EntityList>, mut page: ResMut<NewPageState>) {
    for ev in ev_sync.iter() {
        *page = get_initial_state_with_ids(&q_list, vec![ev.entry_id]);
    }
}

fn newpage_enter (
    mut com: Commands,
    mut ev_newpage: EventReader<JumpToNewPage>,
    asset_server: Res<AssetServer>,
    q_list: Query<&EntityList>
) {
    for ev in ev_newpage.iter() {
        com.insert_resource::<NewPageState>(get_initial_state_with_ids(&q_list, ev.entry_ids.clone()));
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

fn newpage_exit (q: Query<Entity, With<NewPageContents>>, mut com: Commands) {
    com.remove_resource::<NewPageState>();

    for i in q.iter() {
        com.entity(i).despawn_recursive();
    }
}

fn newpage_update (
    mut egui_ctx: ResMut<EguiContext>,
    window: Res<Windows>,
    mut interaction_query: Query<
        (&Interaction, &NewPageButton, &mut UiColor),
        Changed<Interaction>,
    >,
    mut ev_top: EventWriter<JumpToTop>,
    mut newpage_state: ResMut<NewPageState>,
    q_fragment: Query<&Fragment>,
    mut initialized: Local<bool>,
    mut inject_pos: Local<Option<usize>>,
    mut ev_sync: EventWriter<SyncFragments>
) {
    for (inter, btn_attr, mut color) in interaction_query.iter_mut() {
        match *inter {
            Interaction::Clicked => {
                *color = NEWPAGE_CLICK.into();
                match btn_attr {
                    NewPageButton::Return => {
                        ev_top.send(JumpToTop {})
                    },
                    NewPageButton::AddTexts => {
                        newpage_state.entry_clone.push(FragmentClone::Modified {
                            fragment: Fragment { timestamp: create_timestamp(), contents: FragmentContents::TextData { data: "".to_string() } },
                            original_id: None
                        });
                    },
                    NewPageButton::Save => {
                        ev_sync.send(SyncFragments {
                            entry_clone: newpage_state.entry_clone.clone(),
                            original_entries: newpage_state.page_entry_ids.clone(),
                        })
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
    egui::ScrollArea::vertical().always_show_scroll(true).show(ui, |ui| {
        if *initialized == false {
            set_default_font(ui);
            *initialized = true;
        }

        let prev_inject_pos = inject_pos.clone();
        *inject_pos = None;

        for (i, fc) in newpage_state.entry_clone.iter_mut().enumerate() {
            let mut fragment_overwrite = None;

            match fc {
                // For data that is not cloned yet (thus have not been changed yet)
                FragmentClone::NotModified { fragment_id } => {
                    let f = q_fragment.get(*fragment_id).unwrap();
                    ui.label(format!("Fragment ID {:?}", fragment_id));
                    match &f.contents {
                        FragmentContents::TextData { data } => {
                            let mut data_cloned = data.clone();
                            let mut size = ui.available_size();
                            size.y = 40.0;
                            let edit = ui.add_sized(size, TextEdit::multiline(&mut data_cloned).margin(egui::Vec2{x:9.0, y:6.0}));
                            if edit.changed() {
                                fragment_overwrite = Some((FragmentContents::TextData { data: data_cloned }, fragment_id));
                            }
                        },
                        FragmentContents::Code { data, language } => {
                            // TODO handle code
                        },
                        FragmentContents::URL { data } => {
                            // TODO handle url
                        },
                        FragmentContents::Image { data } => {
                            // TODO handle image
                        },
                    };
                },
                // For cloned data (thus have been already changed, desyncing from the master database)
                FragmentClone::Modified { fragment, original_id } => {
                    ui.label(if let Some(oid) = original_id { format!("Modified from ID {:?}", oid) } else { "New Fragment".into() });
                    match &mut fragment.contents {
                        FragmentContents::TextData { data } => {
                            let mut size = ui.available_size();
                            size.y = 40.0;
                            let widget = TextEdit::multiline(data).margin(egui::Vec2{x:9.0, y:6.0}).text_color(Color32::from_rgb(230, 134, 156));
                            let edit = ui.add_sized(size, widget);
                            if let Some(pip) = prev_inject_pos {
                                if i == pip + 1 {
                                    edit.request_focus();
                                }
                            }
                            if data.ends_with("\n\n") {
                                let mut x = data.chars();
                                x.nth_back(1);
                                *data = x.as_str().to_string();

                                *inject_pos = Some(i);
                            }
                        },
                        FragmentContents::Code { data, language } => {
                            // TODO handle code
                        },
                        FragmentContents::URL { data } => {
                            // TODO handle url
                        },
                        FragmentContents::Image { data } => {
                            // TODO handle image
                        },
                    };
                },
            }

            // If some data is modified, clone it and put into entry_clone.
            if let Some((contents, oid)) = fragment_overwrite {
                *fc = FragmentClone::Modified { fragment: Fragment { timestamp: create_timestamp(), contents }, original_id: Some(oid.clone()) }
            }
        }

        // Add a new empty fragment at the position specified by `inject_pos`. `original_id` will be set to None, which means the fragment doesn't have a predecessor.
        if let Some(ip) = *inject_pos {
            newpage_state.entry_clone.insert(ip + 1, FragmentClone::Modified {
                fragment: Fragment { timestamp: create_timestamp(), contents: FragmentContents::TextData { data: "".to_string() } },
                original_id: None
            });
        }
    });
    });
}