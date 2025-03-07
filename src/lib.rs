// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
extern crate alloc;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{ alloy_primitives::{ U256, Address, U64 }, prelude::*, msg, block };

// Define some persistent storage using the Solidity ABI.
// `Counter` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct App{
        mapping(address => mapping(uint256 => Email)) system;
        mapping(address => uint256) len_;
        mapping(address => mapping(address => bool)) block_;
    }

    pub struct Email{
        address sender; //0x000000000000000000000000000000
       string message;//""
       uint64 time; //0
    }
}

/// Declare that `Counter` is a contract with the following external methods.
#[public]
impl App {
    pub fn send_email(&mut self, to: Address, message: String) {
        let s_e = self.block_.getter(to);
        let state = s_e.getter(msg::sender());

        if state.get() {
            return;
        }

        let l_n = self.len_.getter(to);
        let l_n_x = l_n.get();
        let mut inbox = self.system.setter(to);
        let mut email = inbox.setter(l_n.get());

        email.sender.set(msg::sender());
        email.message.set_str(message);
        email.time.set(U64::from(block::timestamp()));

        {
            let mut data = self.len_.setter(to);
            data.set(l_n_x + U256::from(1));
        }
    }

    pub fn get_len(&self) -> U256 {
        let l_n = self.len_.getter(msg::sender());
        l_n.get()
    }

    pub fn get_mail(&self, start: u128, end: u128) -> Vec<(Address, String, u64)> {
        let mut data: Vec<(Address, String, u64)> = (start..end)
            .map(|i| {
                let inbox = self.system.getter(msg::sender());
                let mail = inbox.getter(U256::from(i));
                let sender = mail.sender.get();
                let msg_ = mail.message.get_string();
                let time = mail.time.get().to::<u64>();
                (sender, msg_, time)
            })
            .collect();

        data
    }

    pub fn blocking(&mut self, block_ad: Address, status: bool) {
        let mut b_l = self.block_.setter(msg::sender());
        let mut state = b_l.setter(block_ad);
        state.set(status)
    }

    pub fn blocking_status(&self, b_s: Address) -> bool {
        let b_l = self.block_.getter(msg::sender());
        let state = b_l.getter(b_s);
        state.get()
    }
}
