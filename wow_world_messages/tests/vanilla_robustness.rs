//! 1.12 packets that used to abort or panic the reader. Each is an error
//! now, or reads what is there.
#![cfg(all(feature = "vanilla", feature = "sync"))]

use std::alloc::{GlobalAlloc, Layout, System};

use wow_world_messages::vanilla::opcodes::{ClientOpcodeMessage, ServerOpcodeMessage};

/// The most one allocation may ask for here. A machine with less memory
/// than this one refuses less than it would, so a reservation from a count
/// the packet can't back aborts every time, not only where memory runs out.
const ALLOCATION_LIMIT: usize = 256 * 1024 * 1024;

struct Limited;

unsafe impl GlobalAlloc for Limited {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.size() > ALLOCATION_LIMIT {
            return std::ptr::null_mut();
        }
        System.alloc(layout)
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        if layout.size() > ALLOCATION_LIMIT {
            return std::ptr::null_mut();
        }
        System.alloc_zeroed(layout)
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        if new_size > ALLOCATION_LIMIT {
            return std::ptr::null_mut();
        }
        System.realloc(ptr, layout, new_size)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout)
    }
}

#[global_allocator]
static ALLOCATOR: Limited = Limited;

/// A client packet: its size (the opcode counted) and its opcode.
fn client(opcode: u32, body: &[u8]) -> Vec<u8> {
    let mut v = ((body.len() + 4) as u16).to_be_bytes().to_vec();
    v.extend_from_slice(&opcode.to_le_bytes());
    v.extend_from_slice(body);
    v
}

/// A server packet: its size (the opcode counted) and its opcode.
fn server(opcode: u16, body: &[u8]) -> Vec<u8> {
    let mut v = ((body.len() + 2) as u16).to_be_bytes().to_vec();
    v.extend_from_slice(&opcode.to_le_bytes());
    v.extend_from_slice(body);
    v
}

fn read_client(opcode: u32, body: &[u8]) -> Result<ClientOpcodeMessage, String> {
    ClientOpcodeMessage::read_unencrypted(client(opcode, body).as_slice()).map_err(|e| e.to_string())
}

fn read_server(opcode: u16, body: &[u8]) -> Result<ServerOpcodeMessage, String> {
    ServerOpcodeMessage::read_unencrypted(server(opcode, body).as_slice()).map_err(|e| e.to_string())
}

/// A bug report: not a suggestion, then its text with the size given.
fn bug(size: u32) -> Vec<u8> {
    let mut b = 0_u32.to_le_bytes().to_vec();
    b.extend_from_slice(&size.to_le_bytes());
    b.extend_from_slice(b"stuck\0");
    b.extend_from_slice(&5_u32.to_le_bytes());
    b.extend_from_slice(b"Bug!\0");
    b
}

#[test]
fn a_bug_report_whose_text_has_no_room_for_its_terminator_is_an_error() {
    assert!(read_client(0x1CA, &bug(0)).is_err());
}

#[test]
fn a_bug_report_with_its_sizes_reads() {
    let Ok(ClientOpcodeMessage::CMSG_BUG(m)) = read_client(0x1CA, &bug(6)) else {
        panic!("not a bug report");
    };
    assert_eq!(m.content, "stuck");
    assert_eq!(m.bug_type, "Bug!");
}

#[test]
fn warden_data_shorter_than_four_bytes_reads_whole() {
    let Ok(ClientOpcodeMessage::CMSG_WARDEN_DATA(m)) = read_client(0x2E7, &[1, 2, 3]) else {
        panic!("not warden data");
    };
    assert_eq!(m.encrypted_data, [1, 2, 3]);
}

#[test]
fn warden_data_reads_to_the_end_of_its_body() {
    let body: Vec<u8> = (1..=10).collect();
    let Ok(ClientOpcodeMessage::CMSG_WARDEN_DATA(m)) = read_client(0x2E7, &body) else {
        panic!("not warden data");
    };
    assert_eq!(m.encrypted_data, body);
}

#[test]
fn a_chat_message_whose_text_has_no_room_for_its_terminator_is_an_error() {
    // A system message, its text of size 0 though the terminator is there.
    let mut b = vec![0x0A];
    b.extend_from_slice(&0_u32.to_le_bytes());
    b.extend_from_slice(&0_u64.to_le_bytes());
    b.extend_from_slice(&0_u32.to_le_bytes());
    b.extend_from_slice(&[0, 0]);
    assert!(read_server(0x096, &b).is_err());
}
