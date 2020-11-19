//= https://tools.ietf.org/id/draft-ietf-quic-recovery-32.txt#7
//# Similar to TCP, packets containing only ACK frames do not count
//# towards bytes in flight and are not congestion controlled.

/// Trait to retrieve CongestionControlled for a given value
pub trait CongestionControlled {
    #[inline]
    fn is_congestion_controlled(&self) -> bool {
        true
    }
}

impl<AckRanges> CongestionControlled for crate::frame::Ack<AckRanges> {
    #[inline]
    fn is_congestion_controlled(&self) -> bool {
        false
    }
}
impl CongestionControlled for crate::frame::ConnectionClose<'_> {}
impl<Data> CongestionControlled for crate::frame::Crypto<Data> {}
impl CongestionControlled for crate::frame::DataBlocked {}
impl CongestionControlled for crate::frame::HandshakeDone {}
impl CongestionControlled for crate::frame::MaxData {}
impl CongestionControlled for crate::frame::MaxStreamData {}
impl CongestionControlled for crate::frame::MaxStreams {}
impl CongestionControlled for crate::frame::NewConnectionID<'_> {}
impl CongestionControlled for crate::frame::NewToken<'_> {}
impl CongestionControlled for crate::frame::Padding {}
impl CongestionControlled for crate::frame::PathChallenge<'_> {}
impl CongestionControlled for crate::frame::PathResponse<'_> {}
impl CongestionControlled for crate::frame::Ping {}
impl CongestionControlled for crate::frame::ResetStream {}
impl CongestionControlled for crate::frame::RetireConnectionID {}
impl CongestionControlled for crate::frame::StopSending {}
impl<Data> CongestionControlled for crate::frame::Stream<Data> {}