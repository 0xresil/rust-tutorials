
use borsh::{BorshDeserialize, BorshSerialize};
use std::rc::Rc;
use std::cell::RefCell;
use arr_macro::arr;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Record {
    pub nft_address: String, // < 50
    pub owner: String,       // < 50
    pub register_date: String // < 50
}
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct AccountInfo {
    pub data: Vec<u8>
}

fn main() {

    const RECORD_MAX_SIZE: usize = 44 + 44 + 12 + 4 * 3;

    let mut account_info: Vec<u8> = Vec::new();
    let data1 = String::from("4h8pBJZDp4MSzXH675AgCSMCnxEDZnjkG5DW31ySCdyp");
    let data2 = String::from("Ch4vjb7joSG3RTQc24Z91VJyQXN5WqncvXoXpHEhqEoX");
    let data1_bytes: &[u8] = data1.as_bytes();
    
    let new_owner_address = String::from("Ch4vjb7joSG3RTQc24Z91VJyQXN5WqncvXoXpHEhqEoX");
    let new_nft_address = String::from_utf8(data1_bytes.to_vec()).unwrap();
    
    let new_record: Record = Record {
        nft_address: new_nft_address,
        owner: new_owner_address,
        register_date: String::from("ABCABCABCABC")
    };

    let mut buffer: Vec<u8> = Vec::new();

    new_record.serialize(&mut buffer).unwrap();
    account_info.append(&mut buffer);

    new_record.serialize(&mut buffer).unwrap();
    account_info.append(&mut buffer);

    //println!("{:?}", account_info);

    let dst: Vec<Vec<u8>> = account_info.chunks(RECORD_MAX_SIZE).map(|s| s.into()).collect();
    
    //println!("{} {}", dst.len(), account_info.len());
    let one_record : Record = Record::try_from_slice(dst[0].as_slice()).unwrap();

    //println!("{:?}", one_record);

    let data: [u8; 5] = [4, 6, 7, 4, 3];
    let mut data_vec: Vec<u8> = vec![];
    data_vec.append(&mut data.to_vec());
    data_vec.append(&mut account_info);
    //println!("{:?}", data_vec);

    let mut ref_arr: [u8; 500] = arr![0; 500];
    let account_data : Rc<RefCell<&mut [u8]>> = Rc::new(RefCell::new(&mut ref_arr));
    let x = data_vec.serialize(&mut &mut account_data.borrow_mut()[..]);
/*    println!("data_vec {:?}", data_vec);
    println!("{:?}", account_data);

    let data_arr = account_data.borrow().to_vec();
    let mut split_pos = 0;
    let mut flag = 0;
    for i in 0..data_arr.len() {
        //println!("arr {:?}", data_arr.get(i));
        if (data_arr.get(data_arr.len() - i - 1) != Some(&0)) {
            flag = 1;
        } else {
            split_pos = data_arr.len() - i - 1;
        }
        if flag == 1 {
            break;
        }
    }
    println!("split_pos {}", split_pos);

    let print_arr: &[u8] = &data_arr[..split_pos];

    println!("split {:?}", print_arr);
*/
    let mut dst_vec : Vec<u8> = vec![];
    let mut info = AccountInfo::deserialize(&mut &account_data.borrow()[..]);
    println!("info {:?}", info);
    //let bytes: Vec<u8> = nftAddress.to_vec();
    //let ascii: Ascii = 	Ascii::from_bytes(bytes).unwrap();

    //println!("{:?}", nftAddress.to_vec());

    //let addr = String::from_utf8(nftAddress.to_vec()).unwrap();
    //println!("{}", addr);

    
    // let ascii: Ascii = Ascii::from_bytes(bytes).unwrap();	//	We	know	these	chosen	bytes	are	ok. //	This	call	is	zero-cost:	no	allocation,	copies,	or	scans.
    //    let	string = String::from(ascii);
    
}

