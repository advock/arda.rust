use ethers::types::Address;

pub struct Conts {
    pub iAdd: Address,
}

pub fn add() -> Conts {
    Conts {
        iAdd: "0xD533a949740bb3306d119CC777fa900bA034cd52"
            .parse::<Address>()
            .expect("fail"),
    }
}
