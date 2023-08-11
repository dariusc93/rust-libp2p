use crate::ListenerId;
use libp2p_core::{multiaddr::Protocol, Multiaddr};

#[derive(Debug)]
pub struct ListenOpts {
    id: ListenerId,
    address: Multiaddr,
    external: bool,
}

impl ListenOpts {
    pub fn new(address: Multiaddr) -> ListenOpts {
        ListenOpts {
            id: ListenerId::next(),
            address,
            external: false,
        }
    }

    /// Get the [`ListenerId`] of this listen attempt
    pub fn listener_id(&self) -> ListenerId {
        self.id
    }

    /// Get the [`Multiaddr`] that is being listened on
    pub fn address(&self) -> &Multiaddr {
        &self.address
    }

    /// Determine if address is an external address
    pub fn external(&self) -> bool {
        self.external
    }

    /// Inform swarm to emit an event 
    pub fn set_as_external(&mut self) {
        self.external = true;
    }
}

impl From<Multiaddr> for ListenOpts {
    fn from(addr: Multiaddr) -> Self {
        ListenOpts::new(addr)
    }
}

impl From<Protocol<'_>> for ListenOpts {
    fn from(protocol: Protocol<'_>) -> Self {
        ListenOpts::new(protocol.into())
    }
}
