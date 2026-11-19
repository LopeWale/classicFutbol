//! Non-blocking native UDP datagram transport (`UdpSocketTransport`) for `RollbackSession` (Phase 4).

use crate::{NetMessage, RollbackTransport};
use std::io;
use std::net::{SocketAddr, UdpSocket};

pub struct UdpSocketTransport {
    socket: UdpSocket,
    peer_addr: SocketAddr,
    host_relay_addr: Option<SocketAddr>,
    recv_buf: [u8; 4096],
}

impl UdpSocketTransport {
    pub fn bind(
        local_addr: SocketAddr,
        peer_addr: SocketAddr,
        host_relay_addr: Option<SocketAddr>,
    ) -> io::Result<Self> {
        let socket = UdpSocket::bind(local_addr)?;
        socket.set_nonblocking(true)?;
        Ok(Self {
            socket,
            peer_addr,
            host_relay_addr,
            recv_buf: [0_u8; 4096],
        })
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.socket.local_addr()
    }
}

impl RollbackTransport for UdpSocketTransport {
    fn send_peer(&mut self, msg: NetMessage) {
        if let Ok(payload) = serde_json::to_vec(&msg) {
            let _ = self.socket.send_to(&payload, self.peer_addr);
        }
    }

    fn poll_peer(&mut self) -> Option<NetMessage> {
        loop {
            match self.socket.recv_from(&mut self.recv_buf) {
                Ok((len, _src)) => {
                    if let Ok(msg) = serde_json::from_slice::<NetMessage>(&self.recv_buf[..len]) {
                        return Some(msg);
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => return None,
                Err(_) => return None,
            }
        }
    }

    fn mirror_to_host(&mut self, msg: NetMessage) {
        if let Some(relay) = self.host_relay_addr {
            if let Ok(payload) = serde_json::to_vec(&msg) {
                let _ = self.socket.send_to(&payload, relay);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AdvanceOutcome, RollbackSession};
    use psfc_sim::input::{BTN_PASS, BTN_SKILL, BTN_SPRINT};
    use psfc_sim::{FixtureSnapshot, InputFrame, MatchSimulation, SideController, TeamSide};

    #[test]
    fn real_udp_loopback_peers_and_host_mirror_converge_bit_for_bit() {
        let mut snap = FixtureSnapshot::sample_five_a_side(4242);
        snap.rules.home_controller = SideController::Human;
        snap.rules.away_controller = SideController::Human;

        let s_home = UdpSocket::bind("127.0.0.1:0").unwrap();
        let s_away = UdpSocket::bind("127.0.0.1:0").unwrap();
        let s_relay = UdpSocket::bind("127.0.0.1:0").unwrap();
        s_relay.set_nonblocking(true).unwrap();

        let addr_home = s_home.local_addr().unwrap();
        let addr_away = s_away.local_addr().unwrap();
        let addr_relay = s_relay.local_addr().unwrap();
        drop(s_home);
        drop(s_away);

        let mut home_udp = UdpSocketTransport::bind(addr_home, addr_away, Some(addr_relay)).unwrap();
        let mut away_udp = UdpSocketTransport::bind(addr_away, addr_home, Some(addr_relay)).unwrap();

        let mut home_sess = RollbackSession::new(&snap, TeamSide::Home).unwrap();
        let mut away_sess = RollbackSession::new(&snap, TeamSide::Away).unwrap();
        let mut ref_sim = MatchSimulation::new(&snap).unwrap();

        let target_ticks = 90_u32;
        for t in 1..=target_ticks {
            let h_in = InputFrame {
                move_x: 90,
                move_y: ((t % 40) as i8) - 20,
                buttons: if t % 17 == 0 {
                    BTN_SKILL
                } else if t % 21 == 0 {
                    BTN_PASS
                } else {
                    BTN_SPRINT
                },
                ..InputFrame::IDLE
            };
            let a_in = InputFrame {
                move_x: -75,
                move_y: 30,
                buttons: if t % 19 == 0 { BTN_PASS } else { 0 },
                ..InputFrame::IDLE
            };
            ref_sim.tick(&[h_in, a_in]);

            let out_h = home_sess.step_frame(h_in, &mut home_udp).unwrap();
            let out_a = away_sess.step_frame(a_in, &mut away_udp).unwrap();
            assert!(!matches!(out_h, AdvanceOutcome::DesyncDetected { .. }));
            assert!(!matches!(out_a, AdvanceOutcome::DesyncDetected { .. }));
        }

        // Reconcile final in-flight UDP datagrams
        for _ in 0..8 {
            home_sess.poll_and_reconcile(&mut home_udp).unwrap();
            away_sess.poll_and_reconcile(&mut away_udp).unwrap();
        }

        assert_eq!(home_sess.sim().state_hash(), ref_sim.state_hash());
        assert_eq!(away_sess.sim().state_hash(), ref_sim.state_hash());

        // Confirm host mirror socket received authoritative input batches
        let mut relay_packets = 0_usize;
        let mut buf = [0_u8; 4096];
        while s_relay.recv_from(&mut buf).is_ok() {
            relay_packets += 1;
        }
        assert!(relay_packets > 0, "expected mirrored UDP packets at host relay");
    }
}
