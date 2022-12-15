use ethers::types::Address;

pub struct Conts {
    pub iAdd: Address,
}

pub fn add() -> Conts {
    Conts {
        iAdd: "0x6bee7818EA411269631cb9E6585703aF19D5e5a6"
            .parse::<Address>()
            .expect("fail"),
    }
}
