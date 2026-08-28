//! Process-to-port mapping
//!
//! Collects local bound TCP/UDP ports and groups them by PID. This is more
//! expensive than a sysinfo snapshot, so callers should invoke it only when
//! a port search is active.

use listeners::{Protocol, SocketState};
use std::collections::HashMap;

/// Returns local bound ports keyed by process ID.
///
/// TCP sockets are included only in LISTEN state. UDP sockets are included
/// when they have a real (non-zero) local port. Collection failures return
/// an empty map so process listing still works.
pub fn collect_ports_by_pid() -> HashMap<u32, Vec<u16>> {
    let mut ports_by_pid: HashMap<u32, Vec<u16>> = HashMap::new();

    let Ok(sockets) = listeners::get_all() else {
        return ports_by_pid;
    };

    for listener in sockets {
        let port = listener.socket.port();
        if !should_index_socket(listener.protocol, listener.state, port) {
            continue;
        }

        let ports = ports_by_pid.entry(listener.process.pid).or_default();
        if !ports.contains(&port) {
            ports.push(port);
        }
    }

    for ports in ports_by_pid.values_mut() {
        ports.sort_unstable();
    }

    ports_by_pid
}

/// Whether a socket should be searchable from the process search field.
fn should_index_socket(protocol: Protocol, state: SocketState, port: u16) -> bool {
    if port == 0 {
        return false;
    }

    match protocol {
        Protocol::TCP => state == SocketState::Listen,
        Protocol::UDP => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skips_unspecified_port() {
        assert!(!should_index_socket(Protocol::TCP, SocketState::Listen, 0));
        assert!(!should_index_socket(Protocol::UDP, SocketState::Unknown, 0));
    }

    #[test]
    fn indexes_tcp_listen_only() {
        assert!(should_index_socket(
            Protocol::TCP,
            SocketState::Listen,
            3000
        ));
        assert!(!should_index_socket(
            Protocol::TCP,
            SocketState::Established,
            3000
        ));
    }

    #[test]
    fn indexes_udp_with_real_port() {
        assert!(should_index_socket(Protocol::UDP, SocketState::Unknown, 53));
    }
}
