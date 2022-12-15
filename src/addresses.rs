use ethers::types::Address;

pub struct Conts {
    pub iAdd: Address,
}

pub fn add() -> Conts {
    Conts {
        iAdd: "0xfc6Be3956527C0546b61Ff47af1Dc00B6a3A7ED6"
            .parse::<Address>()
            .expect("fail"),
    }
}
