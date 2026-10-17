//! 1.12 packets that used to abort or panic the reader. Each is an error
//! now, or reads what is there.
#![cfg(all(feature = "vanilla", feature = "sync"))]

use std::alloc::{GlobalAlloc, Layout, System};
use std::io::Write;

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

fn zlib(data: &[u8]) -> Vec<u8> {
    let mut z = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
    z.write_all(data).unwrap();
    z.finish().unwrap()
}

/// A `/who`'s levels, names and masks, before its zones and strings.
fn who_start() -> Vec<u8> {
    let mut b = Vec::new();
    b.extend_from_slice(&1_u32.to_le_bytes());
    b.extend_from_slice(&60_u32.to_le_bytes());
    b.extend_from_slice(b"\0\0");
    b.extend_from_slice(&u32::MAX.to_le_bytes());
    b.extend_from_slice(&u32::MAX.to_le_bytes());
    b
}

#[test]
fn a_who_claiming_four_billion_strings_is_an_error() {
    let mut b = who_start();
    b.extend_from_slice(&0_u32.to_le_bytes());
    b.extend_from_slice(&u32::MAX.to_le_bytes());
    assert!(read_client(0x062, &b).is_err());
}

#[test]
fn a_who_claiming_four_billion_zones_is_an_error() {
    let mut b = who_start();
    b.extend_from_slice(&u32::MAX.to_le_bytes());
    b.extend_from_slice(&12_u32.to_le_bytes());
    assert!(read_client(0x062, &b).is_err());
}

#[test]
fn a_who_with_what_it_counts_reads() {
    let mut b = who_start();
    b.extend_from_slice(&1_u32.to_le_bytes());
    b.extend_from_slice(&12_u32.to_le_bytes());
    b.extend_from_slice(&2_u32.to_le_bytes());
    b.extend_from_slice(b"Bo\0Al\0");
    let Ok(ClientOpcodeMessage::CMSG_WHO(m)) = read_client(0x062, &b) else {
        panic!("not a who");
    };
    assert_eq!(m.zones, [12]);
    assert_eq!(m.search_strings, ["Bo", "Al"]);
}

#[test]
fn an_express_flight_claiming_four_billion_nodes_is_an_error() {
    let mut b = 0xF130_0000_0100_0001_u64.to_le_bytes().to_vec();
    b.extend_from_slice(&0_u32.to_le_bytes());
    b.extend_from_slice(&u32::MAX.to_le_bytes());
    b.extend_from_slice(&5_u32.to_le_bytes());
    assert!(read_client(0x312, &b).is_err());
}

#[test]
fn a_bidder_list_claiming_four_billion_items_is_an_error() {
    let mut b = 0xF130_0000_0100_0001_u64.to_le_bytes().to_vec();
    b.extend_from_slice(&0_u32.to_le_bytes());
    b.extend_from_slice(&u32::MAX.to_le_bytes());
    assert!(read_client(0x264, &b).is_err());
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
fn a_bug_report_claiming_four_gigabytes_of_text_is_an_error() {
    assert!(read_client(0x1CA, &bug(u32::MAX)).is_err());
}

#[test]
fn a_bug_report_with_its_sizes_reads() {
    let Ok(ClientOpcodeMessage::CMSG_BUG(m)) = read_client(0x1CA, &bug(6)) else {
        panic!("not a bug report");
    };
    assert_eq!(m.content, "stuck");
    assert_eq!(m.bug_type, "Bug!");
}

/// An auth session up to its addons' size uncompressed.
fn auth_session(decompressed_size: u32) -> Vec<u8> {
    let mut b = 5875_u32.to_le_bytes().to_vec();
    b.extend_from_slice(&0_u32.to_le_bytes());
    b.extend_from_slice(b"A\0");
    b.extend_from_slice(&7_u32.to_le_bytes());
    b.extend_from_slice(&[0; 20]);
    b.extend_from_slice(&decompressed_size.to_le_bytes());
    b
}

#[test]
fn an_auth_session_claiming_four_gigabytes_of_addons_reads_what_it_has() {
    let mut addon = b"Blizzard_AuctionUI\0".to_vec();
    addon.push(1);
    addon.extend_from_slice(&0x4C1C_776D_u32.to_le_bytes());
    addon.extend_from_slice(&0_u32.to_le_bytes());
    let mut b = auth_session(u32::MAX);
    b.extend_from_slice(&zlib(&addon));
    let Ok(ClientOpcodeMessage::CMSG_AUTH_SESSION(m)) = read_client(0x1ED, &b) else {
        panic!("not an auth session");
    };
    assert_eq!(m.addon_info.len(), 1);
    assert_eq!(m.addon_info[0].addon_name, "Blizzard_AuctionUI");
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
fn a_compressed_update_claiming_four_billion_objects_is_an_error() {
    let mut update = u32::MAX.to_le_bytes().to_vec();
    update.push(0);
    let mut b = u32::MAX.to_le_bytes().to_vec();
    b.extend_from_slice(&zlib(&update));
    assert!(read_server(0x1F6, &b).is_err());
}

#[test]
fn a_monster_move_claiming_four_billion_splines_is_an_error() {
    let mut b = vec![0x01, 0x05];
    b.extend_from_slice(&[0; 12]);
    b.extend_from_slice(&1_u32.to_le_bytes());
    b.push(0);
    b.extend_from_slice(&0x100_u32.to_le_bytes());
    b.extend_from_slice(&1000_u32.to_le_bytes());
    b.extend_from_slice(&u32::MAX.to_le_bytes());
    assert!(read_server(0x0DD, &b).is_err());
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
