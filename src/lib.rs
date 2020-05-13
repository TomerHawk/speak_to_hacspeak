#![crate_type = "staticlib"]

use hacspec::prelude::*;
use hacspec_examples::aes_gcm::aes::*;
use libc::c_uchar;
use libc::uint16_t;

fn aes_256_enc_dec_teso(m: &ByteSeq, key: Key256, iv: Nonce, ctr: U32, ctxt: Option<&ByteSeq>) {
    let c = aes256_encrypt(key, iv, ctr, m);
    let m_dec = aes256_decrypt(key, iv, ctr, &c);
    assert_bytes_eq!(m, m_dec);
    if ctxt.is_some() {
        assert_bytes_eq!(c, ctxt.unwrap());
    }
    println!("Done!message : {:?}",m );
}
fn aes256_encryption(m: &ByteSeq, key: Key256, iv: Nonce, ctr: U32) -> ByteSeq{
    let c = aes256_encrypt(key, iv, ctr, m);
    println!("Done!cipher : {:?}",c );
    return c;
}

fn aes_256_decryption(c: &ByteSeq, key: Key256, iv: Nonce, ctr: U32){
    let m = aes256_decrypt(key, iv, ctr, c);
    println!("Done!message : {:?}",m );
}


#[no_mangle]
pub extern fn teso_aes256_1(m:[u8;32],k:[u8;32]) -> [u16;32] {


    let msg = ByteSeq::from_public_slice(
        &m
    );
    let key = Key256::from_public_slice(
        &k
    );
    let nonce = Nonce::from_public_slice(&[
        0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb
    ]);
    let ctr = U32(0xfcfdfeff);
    /*let ctxt = ByteSeq::from_public_slice(&[
        0x60, 0x1e, 0xc3, 0x13, 0x77, 0x57, 0x89, 0xa5, 0xb7, 0xa7, 0xf5, 0x04, 0xbb, 0xf3, 0xd2,
        0x28
    ]);*/

    //aes_256_enc_dec_teso(&msg, key, nonce, ctr, None /*, Some(&ctxt)*/);
    let c = aes256_encryption(&msg, key, nonce, ctr);
    let mut cipher:[u16;32] = [0;32];
    for i in 0..32{
        cipher[i] = u16::from(c[i]);
    }
    println!("{:?} is {:?}",c,cipher);
    return cipher;

}
