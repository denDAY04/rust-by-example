fn main() {
    let mut _mutable_integer = 7i32;

    {
        // Shadowing by immutable `_mutable_integer`
        let _mutable_integer = _mutable_integer;

        // Error! `_mutable_integer` is frozen in this scope
        // _mutable_integer = 50;
        // FIXME ^ Comment out this line

        // `_mutable_integer` goes out of scope
    }

    // Ok! `_mutable_integer` is not frozen in this scope
    _mutable_integer = 3;
	
	// custom test to check the inverse: immutable -> mutable scoped
	let _imutable_integer = 42i32;
	println!("Imutable integer: {}", _imutable_integer);
	{
		let mut _imutable_integer = _imutable_integer;
		_imutable_integer = -1;
		println!("Inner imutable integer: {}", _imutable_integer);
	}
	println!("Imutable integer: {}", _imutable_integer);
	// turns out binding an imutable to a mutable just introduces an shadowed variable
}
