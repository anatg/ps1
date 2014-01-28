
use std::os;
use std::io::File;

fn main() {
    let args: ~[~str] = os::args();
    if args.len() != 3 {
        println!("Usage: {:s} <inputfile1> <inputfile2>", args[0]); 
    } else {
        let share1_file = &args[1];
        let share2_file = &args[2];
        let path1 = Path::new(share1_file.clone());
        let path2 = Path::new(share2_file.clone());
        let msg_file1 = File::open(&path1);
        let msg_file2 = File::open(&path2);
        

        match (msg_file1, msg_file2) {
            (Some(mut share1), Some(mut share2)) => {
                let msg_bytes1: ~[u8] = share1.read_to_end();
                let msg_bytes2: ~[u8] = share2.read_to_end();
                print!("{:s}", std::str::from_utf8_owned(
                    xor(msg_bytes1, msg_bytes2)));              
            } ,
            (_,_) => fail!("Error opening message files")
        }
    }
}

fn xor(a: &[u8], b: &[u8]) -> ~[u8] {
    let mut ret = ~[];
    for i in range(0, a.len()) {
	   ret.push(a[i] ^ b[i]);
    }
    ret
}
