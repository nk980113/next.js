use turbopack::transition::{Transition, TransitionVc};
use turbopack_core::{asset::AssetVc, virtual_asset::VirtualAssetVc};

use crate::embed_js::next_js_file;

#[turbo_tasks::value(shared)]
pub struct NextServerToClientTransition {}

#[turbo_tasks::value_impl]
impl Transition for NextServerToClientTransition {
    #[turbo_tasks::function]
    fn process_source(&self, asset: AssetVc) -> AssetVc {
        VirtualAssetVc::new(
            asset.path().join("client-proxy.tsx"),
            next_js_file("entry/app/server-to-client.tsx").into(),
        )
        .into()
    }
}
