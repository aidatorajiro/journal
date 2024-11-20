//! UI definitions for toppage
//! The app's start page.

use bevy::{prelude::*, ui::FocusPolicy};

use crate::{constants::style::*, typedef::{component::*, event::*, state::*}};

use super::inner::*;

pub fn top_exit (q: Query<Entity, With<TopPageContents>>, mut com: Commands) {
    for i in q.iter() {
        com.entity(i).despawn_recursive();
    }
}

pub fn top_enter(mut commands: Commands, asset_server: Res<AssetServer>) {
    
    commands
    .spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_wrap: FlexWrap::Wrap,
            ..default()
        },
        background_color: Color::NONE.into(),
        ..default()
    })
    .with_children(|parent| {
        let tags = [TopPageButton::Explore, TopPageButton::Linear, TopPageButton::NewPage, TopPageButton::Migrate];
        for tag in tags {
            let btnstyle = Style {
                width: Val::Percent(50.0),
                height: Val::Percent(50.0),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            };
            
            parent
            .spawn(ButtonBundle {
                style: btnstyle.clone(),
                background_color: TOPBTN_NORMAL.clone().into(),
                ..default()
            })
            .with_children(|parent| {

                let image = match tag {
                    TopPageButton::NewPage => asset_server.load("newpage.png").into(),
                    TopPageButton::Explore => asset_server.load("explore.png").into(),
                    TopPageButton::Linear => asset_server.load("linear.png").into(),
                    TopPageButton::Migrate => asset_server.load("migrate.png").into(),
                };

                parent.spawn(ImageBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        left: Val::Percent(0.0),
                        top: Val::Percent(0.0),
                        position_type: PositionType::Absolute,
                        
                        ..default()
                    },
                    background_color: TOPBTN_IMG_OVERLAY.into(),
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

                parent.spawn(TextBundle::from_section(
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

pub fn top_button_update_system(
    mut interaction_query: Query<
        (&Interaction, &TopPageButton, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut ev_newpage: EventWriter<JumpToNewPage>,
    mut ev_explore: EventWriter<JumpToExplore>,
    mut ev_linear: EventWriter<JumpToLinear>,
    mut ev_migrate: EventWriter<JumpToMigrate>
) {
    for (interaction, toppage, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => match toppage {
                TopPageButton::NewPage => {ev_newpage.send(JumpToNewPage::default());},
                TopPageButton::Explore => {ev_explore.send(JumpToExplore::default());},
                TopPageButton::Linear => {ev_linear.send(JumpToLinear::default());},
                TopPageButton::Migrate => {ev_migrate.send(JumpToMigrate::default());},
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