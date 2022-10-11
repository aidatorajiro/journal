pub mod loader {
    use bevy::{reflect::TypeUuid, asset::*};

    #[derive(Debug, TypeUuid)]
    #[uuid = "26c8669a-8a70-4266-a4d4-64b33c0199d7"]
    pub struct RawData {
        pub data: Vec<u8>
    }

    #[derive(Default)]
    pub struct RawDataLoader;

    impl AssetLoader for RawDataLoader {
        fn load<'a>(
            &'a self,
            bytes: &'a [u8],
            load_context: &'a mut LoadContext,
        ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
            Box::pin(async move {
                println!("Rawdata Loaded");
                let custom_asset = RawData {data: bytes.to_vec()};
                load_context.set_default_asset(LoadedAsset::new(custom_asset));
                Ok(())
            })
        }

        fn extensions(&self) -> &[&str] {
            &["rawdata"]
        }
    }

}