//! Asset loader and its data structures

pub mod loader {
    //! Asset Loader Extension for Bevy
    
    use bevy::{asset::*, reflect::Reflect, utils::BoxedFuture};
    use io::Reader;

    #[derive(Debug, Asset, Reflect)]
    pub struct RawData {
        pub data: Vec<u8>
    }

    #[derive(Default)]
    pub struct RawDataLoader;

    impl AssetLoader for RawDataLoader {
        fn load<'a>(
            &'a self,
            bytes: &'a mut Reader,
            settings: &'a Self::Settings,
            load_context: &'a mut LoadContext,
        ) -> BoxedFuture<'a, Result<RawData, anyhow::Error>> {
            Box::pin(async move {
                println!("Rawdata Loaded");
                let mut v: Vec<u8> = Vec::new();
                bytes.read_to_end(&mut v);
                let custom_asset = RawData {data: v};
                Ok(custom_asset)
            })
        }

        fn extensions(&self) -> &[&str] {
            &["rawdata"]
        }
        
        type Asset = RawData;
        
        type Settings = ();
        
        type Error = anyhow::Error;
    }

}