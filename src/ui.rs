pub mod top {
    use bevy::prelude::*;

    use crate::{constants::style::*, typedef::{component::*, event::*, state::*}};

    pub fn top_systems() -> SystemSet {
        return SystemSet::on_update(AppState::Top).with_system(top_button_system);
    }

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

    fn top_button_system(
        mut interaction_query: Query<
            (&Interaction, &mut UiColor, &Children, &TopPageButton),
            (Changed<Interaction>, With<Button>),
        >,
        mut text_query: Query<&mut Text>,
        mut app_state: ResMut<State<AppState>>
    ) {
        for (interaction, mut color, children, toppage) in &mut interaction_query {
            let mut text = text_query.get_mut(children[0]).unwrap();
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
    
            match *interaction {
                Interaction::Clicked => {
                    *color = TOPBTN_PRESSED.into();
                    app_state.set(st);
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