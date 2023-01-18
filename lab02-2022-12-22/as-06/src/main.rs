fn main() {
    let i = [1,2,0,0,0,3,6];
    let num = i.len();
    let mut o = [0;7];
    let (mut x,mut y,_z) = (0,0,0);
    loop {
        if x == num {
            break;
        }
        let z = i[x];
        if z == 0 {
            x += 1;
        }
        else {
            loop {
                if o[y] == 0 {
                    o[y] = z;
                    y += 1;
                    break;
                }
            }
            x += 1;
        }
    }
    print!("input = [{},{},{},{},{},{},{}]",i[0],i[1],i[2],i[3],i[4],i[5],i[6]);
    println!("");
    print!("output = [{},{},{},{},{},{},{}]",o[0],o[1],o[2],o[3],o[4],o[5],o[6]);
}