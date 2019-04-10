/*! New peer message struct.
*/

use nom::{be_u16, be_u32};

use crate::toxcore::binary_io::*;
use crate::toxcore::crypto_core::*;

/** New peer is a struct that holds info to send new peer message to a group chat.

Tell everyone about a new peer in the chat.
The peer who invited joining peer sends this packet to warn everyone that there is a new peer.

Serialized form:

Length    | Content
--------- | ------
`1`       | `0x63`
`2`       | `group number`
`2`       | `peer number`
`4`       | `message number`
`1`       | `0x10`
`2`       | `peer number`
`32`      | Long term PK
`32`      | DHT PK

*/
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NewPeer {
    group_number: u16,
    peer_number: u16,
    message_number: u32,
    new_peer_number: u16,
    long_term_pk: PublicKey,
    dht_pk: PublicKey,
}

impl FromBytes for NewPeer {
    named!(from_bytes<NewPeer>, do_parse!(
        tag!("\x63") >>
        group_number: be_u16 >>
        peer_number: be_u16 >>
        message_number: be_u32 >>
        tag!("\x10") >>
        new_peer_number: be_u16 >>
        long_term_pk: call!(PublicKey::from_bytes) >>
        dht_pk: call!(PublicKey::from_bytes) >>
        (NewPeer {
            group_number,
            peer_number,
            message_number,
            new_peer_number,
            long_term_pk,
            dht_pk,
        })
    ));
}

impl ToBytes for NewPeer {
    fn to_bytes<'a>(&self, buf: (&'a mut [u8], usize)) -> Result<(&'a mut [u8], usize), GenError> {
        do_gen!(buf,
            gen_be_u8!(0x63) >>
            gen_be_u16!(self.group_number) >>
            gen_be_u16!(self.peer_number) >>
            gen_be_u32!(self.message_number) >>
            gen_be_u8!(0x10) >>
            gen_be_u16!(self.new_peer_number) >>
            gen_slice!(self.long_term_pk.as_ref()) >>
            gen_slice!(self.dht_pk.as_ref())
        )
    }
}

impl NewPeer {
    /// Create new NewPeer object.
    pub fn new(group_number: u16, peer_number: u16, message_number: u32, new_peer_number: u16, long_term_pk: PublicKey, dht_pk: PublicKey) -> Self {
        NewPeer {
            group_number,
            peer_number,
            message_number,
            new_peer_number,
            long_term_pk,
            dht_pk,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    encode_decode_test!(
        new_peer_encode_decode,
        NewPeer::new(1, 2, 3, 4, gen_keypair().0, gen_keypair().0)
    );
}
