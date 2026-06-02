struct Address {
    id: u64,
    line1: String,
    line2: String,
    line3: String,
    house_num: String,
    city: String,
    state: String,
    country: String,
    pincode: String,
}

struct User {
    id: u64,
    fname: String,
    lname: String,
    email: String,
    address: Address,
}


trait OSAddress {
    fn new(&self) ->
}
