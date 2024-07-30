// signal for 1.0 compatibility
extern "C" fn interface_version_8() -> () {}

// copy memory to/from host, so we can pass in/out Vec<u8>
extern "C" fn allocate(size: usize) -> u32;
extern "C" fn deallocate(pointer: u32);

// creates an initial state of a contract with a configuration send in the argument msg_ptr
extern "C" fn instantiate(env_ptr: u32, info_ptr: u32, msg_ptr: u32) -> u32;// signal for 1.0 compatibility
extern "C" fn interface_version_8() -> () {}

// copy memory to/from host, so we can pass in/out Vec<u8>
extern "C" fn allocate(size: usize) -> u32;
extern "C" fn deallocate(pointer: u32);

// creates an initial state of a contract with a configuration send in the argument msg_ptr
extern "C" fn instantiate(env_ptr: u32, info_ptr: u32, msg_ptr: u32) -> u32;