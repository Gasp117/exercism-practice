pub fn brackets_are_balanced(string: &str) -> bool {
    let br = string.as_bytes().iter().try_fold(vec![], |mut br, c| {
        match c {
            b'{' | b'[' | b'(' => br.push(c),
            b'}' => {
                if br.pop() != Some(&b'{') {
                    return Err(());
                }
            }
            b')' => {
                if br.pop() != Some(&b'(') {
                    return Err(());
                }
            }
            b']' => {
                if br.pop() != Some(&b'[') {
                    return Err(());
                }
            }
            _ => (),
        }
        Ok(br)
    });
    br.is_ok() && br.unwrap().is_empty()
}
