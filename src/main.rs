//! Main Program

pub mod constants;
pub mod typedef;
pub mod subwindow;
pub mod assets;

use std::fs::File;
use std::io::prelude::*;

use assets::assets::RawData;
use assets::assets::RawDataLoader;
use bevy_render::texture::CompressedImageFormats;
use bevy_render::texture::ImageType;
use subwindow::subwindow::*;
use typedef::component::*;
use typedef::state::*;
use freetype::Library;
use freetype::face::LoadFlag;

use bevy::{prelude::*, render::RenderApp};
use bevy_egui::EguiPlugin;
use bevy_render::{RenderStage};

/// Main function
fn main() {
    let mut app = App::new();
    
    app.init_resource::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_asset::<RawData>()
        .init_asset_loader::<RawDataLoader>()
        .add_startup_system(setup)
        .add_system(system_drag_and_drop)
        .add_system(system_swash_test)
        .add_system(subwindow_event)
        .add_system_set(subwindow_ui_set());
    
    let render_app = app.sub_app_mut(RenderApp);
    render_app.add_system_to_stage(RenderStage::Extract, subwindow_subapp_system);

    app.run();
}

/// setup function for bevy
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 2d camera
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle((SubWindow::default(), MemoField));

    commands.spawn()
        .insert(SwashText {})
        .insert(asset_server.load::<RawData, _>("Nabla-Regular-VariableFont_EDPT,EHLT.ttf"));
}

/// Event listener for file drag and drop event.
fn system_drag_and_drop(
    mut dnd_ev: EventReader<FileDragAndDrop>
) {
    for ev in dnd_ev.iter() {
        println!("{:?}", ev);
        match ev {
            FileDragAndDrop::DroppedFile { .. } => {}
            FileDragAndDrop::HoveredFile { .. } => {},
            FileDragAndDrop::HoveredFileCancelled { .. } => {},
        }
    }
}

fn system_swash_test (mut q: Query<&Handle<RawData>, With<SwashText>>, rawdata: Res<Assets<RawData>>, mut simplestate: Local<SimpleState>) {
    for s in q.iter_mut() {
        if let Some(x) = rawdata.get(s) {
            if (simplestate.switch) {continue;}

            /*let fontref = FontDataRef::new(x.data.as_slice())
                .unwrap()
                .get(0)
                .unwrap();*/

            /*let mut context = ShapeContext::new();
            let mut shaper = context.builder(fontref)
                .script(Script::Arabic)
                .direction(swash::shape::Direction::RightToLeft)
                .size(16.)
                .build();
            shaper.add_str("a quick brown fox?");*/
            
            /* 
            let mut context = ScaleContext::new();
            let mut scaler = context.builder(fontref)
                .size(300.)
                .build();
            let glyph_id = fontref.charmap().map('A');
            let image = Render::new(&[
                    Source::ColorOutline(0),
                    Source::ColorBitmap(StrikeWith::BestFit),
                    Source::Outline,
                ])
                // Select a subpixel format
                .format(Format::Subpixel)
                // Apply the fractional offset
                .offset(Vector::new(0., 0.))
                // Render the image
                .render(&mut scaler, glyph_id);*/

            /*let mut context = ScaleContext::new();
            let mut scaler = context
                .builder(fontref)
                .size(300.)
                .build();
            println!("{}", scaler.has_color_bitmaps());
            let glyph_id = fontref.charmap().map('A');
            let outline_opt = scaler.scale_color_outline(glyph_id);
                
            let outline = outline_opt.unwrap();

            let mut file = File::create("/Users/aidatorajiro/hello.bounds.txt").unwrap();
            file.write_all(format!("{:?}", outline.bounds()).as_bytes()).unwrap();
            
            let mut file = File::create("/Users/aidatorajiro/hello.points.txt").unwrap();
            file.write_all(format!("{:?}", outline.points()).as_bytes()).unwrap();

            let mut file = File::create("/Users/aidatorajiro/hello.verbs.txt").unwrap();
            file.write_all(format!("{:?}", outline.verbs()).as_bytes()).unwrap();

            println!("{}", outline.is_color());*/

            /*

            println!("{:?} {:?} {:?} {:?}", imvec.placement.width, imvec.placement.height, imvec.placement.left, imvec.placement.top);
            println!("{:?}", imvec.content);
            println!("{:?}", imvec.source);

            let mut file = File::create("/Users/aidatorajiro/hello.data").unwrap();
            file.write_all(imvec.data.as_slice()).unwrap();*/

            let lib = Library::init().unwrap();
            let face = lib.new_memory_face(x.data.to_vec(), 0).unwrap();
            face.set_char_size(100*100, 0, 72, 0).unwrap();
            face.load_char('A' as usize, LoadFlag::RENDER | LoadFlag::COLOR).unwrap();
            println!("{}", face.has_color());


            let glyph = face.glyph();

            let left = glyph.bitmap_left() as usize;
            let top = glyph.bitmap_left() as usize;
            let bitmap = glyph.bitmap();
            let width = bitmap.width() as usize;
            let rows = bitmap.rows() as usize;

            let mut file = File::create("/Users/aidatorajiro/hello.data").unwrap();
            file.write_all(bitmap.buffer()).unwrap();

            println!("{:?}", (left, top, width, rows));
            
            simplestate.switch = true;
        }
        
    }
}