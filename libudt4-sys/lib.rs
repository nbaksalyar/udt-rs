extern crate libc;

use libc::{c_int, c_char, c_void, c_uchar};

#[cfg(windows)]
extern crate winapi;

#[cfg(windows)]
mod plat_imports {
    pub use winapi::SOCKADDR as sockaddr;
}
#[cfg(not(windows))]
mod plat_imports {
    pub use libc::sockaddr;
}

use plat_imports::*;


pub type UDTSOCKET = c_int;
pub type SYSSOCKET = c_int;

/// success operation.
pub const SUCCESS : c_int = 0;
/// connection setup failure.
pub const ECONNSETUP  : c_int = 1000;
/// server does not exist.
pub const ENOSERVER   : c_int = 1001;
/// connection request was rejected by server.
pub const ECONNREJ    : c_int = 1002;
/// could not create/configure UDP socket.
pub const ESOCKFAIL   : c_int = 1003;
/// connection request was aborted due to security reasons.
pub const ESECFAIL    : c_int = 1004;
/// connection failure.
pub const ECONNFAIL   : c_int = 2000;
/// connection was broken.
pub const ECONNLOST   : c_int = 2001;
/// connection does not exist.
pub const ENOCONN : c_int = 2002;
/// system resource failure.
pub const ERESOURCE   : c_int = 3000;
/// could not create new thread.
pub const ETHREAD : c_int = 3001;
/// no memory space.
pub const ENOBUF  : c_int = 3002;
/// file access error.
pub const EFILE   : c_int = 4000;
/// invalid read offset.
pub const EINVRDOFF   : c_int = 4001;
/// no read permission.
pub const ERDPERM : c_int = 4002;
/// invalid write offset.
pub const EINVWROFF   : c_int = 4003;
/// no write permission.
pub const EWRPERM : c_int = 4004;
/// operation not supported.
pub const EINVOP  : c_int = 5000;
/// cannot execute the operation on a bound socket.
pub const EBOUNDSOCK  : c_int = 5001;
/// cannot execute the operation on a connected socket.
pub const ECONNSOCK   : c_int = 5002;
/// bad parameters.
pub const EINVPARAM   : c_int = 5003;
/// invalid UDT socket.
pub const EINVSOCK    : c_int = 5004;
/// cannot listen on unbound socket.
pub const EUNBOUNDSOCK    : c_int = 5005;
/// (accept) socket is not in listening state.
pub const ENOLISTEN   : c_int = 5006;
/// rendezvous connection process does not allow listen and accept call.
pub const ERDVNOSERV  : c_int = 5007;
/// rendezvous connection setup is enabled but bind has not been called before connect.
pub const ERDVUNBOUND : c_int = 5008;
/// operation not supported in SOCK_STREAM mode.
pub const ESTREAMILL  : c_int = 5009;
/// operation not supported in SOCK_DGRAM mode.
pub const EDGRAMILL   : c_int = 5010;
/// another socket is already listening on the same UDP port.
pub const EDUPLISTEN  : c_int = 5011;
/// message is too large to be hold in the sending buffer.
pub const ELARGEMSG   : c_int = 5012;
/// non-blocking call failure.
pub const EASYNCFAIL  : c_int = 6000;
/// no buffer available for sending.
pub const EASYNCSND   : c_int = 6001;
/// no data available for read.
pub const EASYNCRCV   : c_int = 6002;
/// timeout before operation completes.
pub const ETIMEOUT    : c_int = 6003;
/// Error has happened at the peer side.
pub const EPEERERR    : c_int = 7000;


pub const INVALID_SOCK: c_int = -1;
pub const UDT_ERROR: c_int = -1;

#[repr(C)]
pub enum EPOLLOpt {
    UDT_EPOLL_IN = 0x1,
    UDT_EPOLL_OUT = 0x4,
    UDT_EPOLL_ERR = 0x8
}

#[repr(C)]
pub enum UDTOpt {
#[allow(non_camel_case_types)]
   UDT_MSS,             // the Maximum Transfer Unit
   UDT_SNDSYN,          // if sending is blocking
   UDT_RCVSYN,          // if receiving is blocking
   UDT_CC,              // custom congestion control algorithm
   UDT_FC,		// Flight flag size (window size)
   UDT_SNDBUF,          // maximum buffer in sending queue
   UDT_RCVBUF,          // UDT receiving buffer size
   UDT_LINGER,          // waiting for unsent data when closing
   UDP_SNDBUF,          // UDP sending buffer size
   UDP_RCVBUF,          // UDP receiving buffer size
   UDT_MAXMSG,          // maximum datagram message size
   UDT_MSGTTL,          // time-to-live of a datagram message
   UDT_RENDEZVOUS,      // rendezvous connection mode
   UDT_SNDTIMEO,        // send() timeout
   UDT_RCVTIMEO,        // recv() timeout
   UDT_REUSEADDR,	// reuse an existing port or create a new one
   UDT_MAXBW,		// maximum bandwidth (bytes per second) that the connection can use
   UDT_STATE,		// current socket state, see UDTSTATUS, read only
   UDT_EVENT,		// current avalable events associated with the socket
   UDT_SNDDATA,		// size of data in the sending buffer
   UDT_RCVDATA		// size of data available for recv
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub enum UdtStatus {
    INIT = 1,
    OPENED,
    LISTENING,
    CONNECTING,
    CONNECTED,
    BROKEN,
    CLOSING,
    CLOSED,
    NONEXIST
}


pub type SOCKOPT = UDTOpt;

#[cfg(windows)]
pub type SYS_UDPSOCKET = std::os::windows::io::RawSocket;
#[cfg(not(windows))]
pub type SYS_UDPSOCKET = std::os::unix::io::RawFd;


extern {

    pub fn udt_startup();
    pub fn udt_cleanup();
    pub fn udt_socket(af: c_int, ty: c_int, protocol: c_int) -> UDTSOCKET;
    pub fn udt_bind(u: UDTSOCKET, name: *const sockaddr, namelen: c_int) -> c_int;
    pub fn udt_bind2(u: UDTSOCKET, other: SYS_UDPSOCKET) -> c_int;
    pub fn udt_listen(u: UDTSOCKET, backlog: c_int) -> c_int;
    pub fn udt_accept(u: UDTSOCKET, addr: *mut sockaddr, addrlen: *mut c_int) -> UDTSOCKET;
    pub fn udt_connect(u: UDTSOCKET, name: *const sockaddr, namelen: c_int) -> c_int;
    pub fn udt_close(u: UDTSOCKET) -> c_int;
    pub fn udt_getpeername(u: UDTSOCKET, name: *mut sockaddr, namelen: *mut c_int) -> c_int;
    pub fn udt_getsockname(u: UDTSOCKET, name: *mut sockaddr, namelen: *mut c_int) -> c_int;
    pub fn udt_getsockopt(u: UDTSOCKET, level: c_int, optname: SOCKOPT, optval: *mut c_void, optlen: *mut c_int) -> c_int;
    pub fn udt_setsockopt(u: UDTSOCKET, level: c_int, optname: SOCKOPT, optval: *const c_void, openlen: c_int) -> c_int;

    pub fn udt_send(u: UDTSOCKET, buf: *const c_uchar, len: c_int, flags: c_int) -> c_int;
    pub fn udt_sendmsg(U: UDTSOCKET, buf: *const c_uchar, len: c_int, ttl: c_int, inorder: c_int) -> c_int;

    pub fn udt_recv(u: UDTSOCKET, buf: *mut c_uchar, len: c_int, flags: c_int) -> c_int;
    pub fn udt_recvmsg(u: UDTSOCKET, but: *mut c_uchar, len: c_int) -> c_int;

    pub fn udt_epoll_create() -> c_int;
    pub fn udt_epoll_add_usock(eid: c_int, usock: UDTSOCKET, events: *const c_int) -> c_int;
    pub fn udt_epoll_add_ssock(eid: c_int, ssock: SYSSOCKET, events: *const c_int) -> c_int;

    pub fn udt_epoll_remove_usock(eid: c_int, usock: UDTSOCKET) -> c_int;
    pub fn udt_epoll_remove_ssock(eid: c_int, ssock: SYSSOCKET) -> c_int;

    pub fn udt_epoll_wait2(eid: c_int, readfs: *mut UDTSOCKET, rnum: *mut c_int, writefs: *mut UDTSOCKET, wnum: *mut c_int, msTimeOut: i64,
                        lrfds: *mut SYSSOCKET, lrnum: *mut c_int, lwfds: *mut SYSSOCKET, lwnum: *mut c_int) -> c_int;
    pub fn udt_epoll_release(eid: c_int) -> c_int;

    pub fn udt_getsockstate(u: UDTSOCKET) -> UdtStatus;


    pub fn udt_getlasterror_code() -> c_int;
    pub fn udt_getlasterror_desc() -> *const c_char;


}


#[test]
fn smoke() {
    unsafe {
        udt_startup();
        udt_cleanup();
    }
}
