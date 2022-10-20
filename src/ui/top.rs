//! UI definitions for toppage
use bevy::{prelude::*, ui::FocusPolicy};

use crate::{constants::style::*, typedef::{component::*, event::*, state::*}};

use super::inner::*;

/// Enter systems on the top page.
pub fn top_systems_enter() -> SystemSet {
    return SystemSet::on_enter(AppState::TopPage).with_system(use_2d_camera).with_system(top_enter);
}

/// Update systems on the top page.
pub fn top_systems_update() -> SystemSet {
    return SystemSet::on_update(AppState::TopPage).with_system(top_button_update_system);
}

/// Exit systems on the top page.
pub fn top_systems_exit() -> SystemSet {
    return SystemSet::on_exit(AppState::TopPage).with_system(top_exit);
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
        (&Interaction, &TopPageButton, &mut UiColor),
        Changed<Interaction>,
    >,
    mut ev_newpage: EventWriter<JumpToNewPage>,
    mut ev_explore: EventWriter<JumpToExplore>,
    mut ev_linear: EventWriter<JumpToLinear>,
    mut ev_migrate: EventWriter<JumpToMigrate>
) {
    for (interaction, toppage, mut color) in &mut interaction_query {
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