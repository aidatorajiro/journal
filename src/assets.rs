pub mod loader {
    //! Asset Loader Extension for Bevy
    
    use bevy::{reflect::TypeUuid, asset::*};

    #[derive(Debug, TypeUuid)]
    #[uuid = "AAF83272-ED10-456A-B953-0B6E3F7AB9E4"]
    pub struct JournalData {
    }

    #[derive(Default)]
    pub struct JournalDataLoader;

    impl AssetLoader for JournalDataLoader {
        fn load<'a>(
            &'a self,
            bytes: &'a [u8],
            load_context: &'a mut LoadContext,
        ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
            Box::pin(async move {
                println!("Journal Data Loaded");
                let custom_asset = JournalData {};
                load_context.set_default_asset(LoadedAsset::new(custom_asset));
                Ok(())
            })
        }

        fn extensions(&self) -> &[&str] {
            &["journal"]
        }
    }

}