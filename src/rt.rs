use crate::processor;

#[no_mangle]
extern "C" fn drone_self_reset() -> ! {
    processor::self_reset()
}
