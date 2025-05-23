use std::{
    hash::{Hash, Hasher},
    sync::RwLock,
};

use bytes::Bytes;
use time::OffsetDateTime;
use valence_protocol::{CompressionThreshold, PacketSide, PacketState, decode::PacketFrame};

pub struct PacketRegistry {
    packets: RwLock<Vec<Packet>>,
    receiver: flume::Receiver<Packet>,
    sender: flume::Sender<Packet>,
}

#[allow(unused)]
impl PacketRegistry {
    pub fn new() -> Self {
        let (sender, receiver) = flume::unbounded::<Packet>();

        Self {
            packets: RwLock::new(Vec::new()),
            receiver,
            sender,
        }
    }

    pub fn subscribe(&self) -> flume::Receiver<Packet> {
        self.receiver.clone()
    }

    pub fn register(&self, packet: Packet) {
        self.packets.write().unwrap().push(packet);
    }

    // register_all(takes an array of packets)
    pub fn register_all(&self, packets: &[Packet]) {
        self.packets.write().unwrap().extend_from_slice(packets);
    }

    fn get_specific_packet(&self, side: PacketSide, state: PacketState, packet_id: i32) -> Packet {
        let time = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());

        self.packets
            .read()
            .unwrap()
            .iter()
            .find(|packet| packet.id == packet_id && packet.side == side && packet.state == state)
            .unwrap_or(&Packet {
                side,
                state,
                id: packet_id,
                timestamp: Some(time),
                name: "Unknown Packet",
                data: None,
            })
            .clone()
    }

    pub async fn process(
        &self,
        side: PacketSide,
        state: PacketState,
        threshold: CompressionThreshold,
        packet: &PacketFrame,
    ) -> anyhow::Result<()> {
        let mut p = self.get_specific_packet(side, state, packet.id);

        let time = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());

        p.data = Some(packet.body.clone().freeze());
        p.timestamp = Some(time);

        // store in received_packets
        self.sender.send_async(p).await?;

        Ok(())
    }
}

impl Default for PacketRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Eq, serde::Serialize, serde::Deserialize)]
pub struct Packet {
    pub side: PacketSide,
    pub state: PacketState,
    pub id: i32,
    #[serde(skip)]
    pub timestamp: Option<OffsetDateTime>,
    #[serde(skip)]
    pub name: &'static str,
    /// Uncompressed packet data
    #[serde(skip)]
    pub data: Option<Bytes>,
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.side == other.side
            && self.state == other.state
            && self.id == other.id
            && self.data == other.data
    }
}

impl Hash for Packet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.side.hash(state);
        self.state.hash(state);
    }
}
