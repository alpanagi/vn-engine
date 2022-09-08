use bevy::asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};

use crate::domain::Script;

#[derive(Default)]
pub struct ScriptLoader;

impl AssetLoader for ScriptLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let contents = std::str::from_utf8(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(Script {
                content: contents.to_string(),
            }));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["vn"]
    }
}
