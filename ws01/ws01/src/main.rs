mod room;

use crate::room::Room;

fn main() {
    let r1 = Room{
        name: "Mom's basement".to_owned(),
        cap: 129,
        mystic_quota: Some(42),
    };


}
