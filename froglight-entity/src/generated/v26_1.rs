use froglight_common::version::V26_1;

#[expect(clippy::wildcard_imports, reason = "Generated code")]
use crate::generated::component::*;

/// TODO: Delete Me
struct EntityPlaceholder;

generate! {
    @version V26_1,
    datatypes: {
        Byte(u8) = 0
    },
    EntityPlaceholder => { ident: "froglight:placeholder", global: 0,
        components: [ Placeholder = 0 ]
    }
}
