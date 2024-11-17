// implementing elgamle encryprion algorithm
// Alice------------Bob
// Bob share his public key to alice, alice then encrypt message with that key to bob, bob can then now decrypt the message
// Bob chooses a relatively large prime number P and g, which are publicly known
// Bob select a private key from a set of prime numbers {2,3 ......p-2}
// Bob computes his public key, using g^pr modP, where pr is his private key
// Alice compute an emphemeral key  choosing a ny random prime number then doing g^i modP
// Alice compute the masking key by using bob pubKey with the random number he selected B^i modp
// Alice then encrypt the message Y= XKm modp, he then send Y(encrypted message) and Emphemeral Key to Bod
// Bob then compute his masking key by doing ke^pr mod P he can then now decrypt the message by doing X=Ykm^-1 mod P.


fn main() {

    fn g_mod_p( g: usize, pr: usize,  p: usize, ) -> usize {
        let mut result = 1;
        let g_mod_p = g % p;
        for _ in 0..pr {
            result = (result * g_mod_p) % p;
        }
        return result;
    }

  
    fn mod_inverse(a: usize, m: usize) -> Option<usize> {
        let mut t: isize = 0;
        let mut newt: isize = 1;
        let mut r: usize = m;
        let mut newr: usize = a;

        while newr != 0 {
            let quotient = r / newr;
            let old_t = t;
            t = newt;
            newt = old_t - quotient as isize * newt; // Ensure correct arithmetic
            let old_r = r;
            r = newr;
            newr = old_r - quotient as usize * newr; // Ensure correct arithmetic
        }

        if r > 1 {
            return None; // a and m are not coprime
        }

        if t < 0 {
            t += m as isize;
        }

        Some((t % m as isize) as usize) // Return t modulo m
    }

    // bob computes his pubkey
    let bob_k_pr: u64 = 31u64;
    let p= 23u64;
    let g =29;
    let bob_pub_key = g_mod_p(g, bob_k_pr as usize, p as usize);
    println!("{}", bob_pub_key);

    // comptute the emphemeral key for Alice
     let i = 5;
     let alice_emphemeral_key = g_mod_p(g, i, p as usize);
     println!("{}", alice_emphemeral_key);

    //  compute the masking key for Alice
    let alice_masking_key = g_mod_p(bob_pub_key, i, p as usize);
    println!("{}", alice_masking_key);

    // So, now alice can now encrypt her message  using the masking_key_generated.
    let message:usize = 5 % p as usize;
    println!("{}", message);
   let alice_encrypted_message = (message*alice_masking_key) % p as usize;
   println!("{}", alice_encrypted_message);
    
    // Bob can now compute his masking key and use it to decrypt alice message

    let bob_masking_key = g_mod_p(alice_emphemeral_key, bob_k_pr as usize, p as usize);
    println!("{}", bob_masking_key);

    // Bob can now decrypt alice mesage using the masking_she generated


    if let Some(bob_masking_key_inv) = mod_inverse(bob_masking_key, p as usize) {
        let bob_decrypted_message = (alice_encrypted_message * bob_masking_key_inv  ) % p as usize;
        println!("Bob's decrypted message: {}", bob_decrypted_message);
    } else {
        println!("Failed to compute the modular inverse for the masking key.");
    }




}