pub mod btn {
    use bevy::prelude::*;

    use crate::{constants::style::*, typedef::component::*};

    pub fn top_buttons(commands: &mut Commands, asset_server: &Res<AssetServer>) {
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
            let tags = vec![TopPageButton::NewPage, TopPageButton::Explore, TopPageButton::Linear, TopPageButton::Migrate];
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
                    color: Color::rgb(0.9, 0.9, 0.9),
                };
                
                parent
                .spawn_bundle(ButtonBundle {
                    style: btnstyle.clone(),
                    color: TOPBTN_NORMAL.clone().into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "", 
                        txtstyle.clone()
                    ));
                }).insert(tag);
            }
        });
    }
}