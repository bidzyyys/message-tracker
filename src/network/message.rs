// Message is received from peers in a p2p network.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    pub id: String,
    pub peer_id: String,
    pub data: Vec<u8>,
}
