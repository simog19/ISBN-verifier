/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    //unimplemented!("Is {:?} a valid ISBN number?", isbn); //unimplemented ritorna never
    let mut t = 0;
    let mut w = 10;

    for ch in isbn.chars(){
        match ch {
            '-' => continue,
            'X' if w==1 => t += 10*1,
            '0'..='9' => t += ch.to_digit(10).unwrap()*w, //cosa fa unwrap???
            _ => return false
        }
        if w==0 { return false; }
        w-=1;
    }
    t % 11 == 0 && w == 0
}
