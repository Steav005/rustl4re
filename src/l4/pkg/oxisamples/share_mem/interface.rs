use l4re::mem::Dataspace;
iface! {
    trait Shm {
        const PROTOCOL_ID: i64 = 0x42;
        fn witter(&mut self, length: u32, ds: Cap<Dataspace>);
    }
}
