use serde::Serialize;

use crate::bridge::ResolveBytes;
use crate::bridge::Serializer;

/// Implemented automatically with the Effect macro from `crux_macros`.
/// This is used by the [`Bridge`](crate::bridge::Bridge) to serialize effects going across the
/// FFI boundary.
pub trait Effect: Send + 'static {
    /// Ffi is an enum with variants corresponding to the Effect variants
    /// but instead of carrying a `Request<Op>` they carry the `Op` directly
    type Ffi: Serialize;

    /// Converts the `Effect` into its FFI counterpart and returns it alongside
    /// a deserializing version of the resolve callback for the request that the
    /// original `Effect` was carrying.
    ///
    /// You should not need to call this method directly. It is called by
    /// the [`Bridge`](crate::bridge::Bridge), which also supplies the `serializer`.
    fn serialize<S>(self, serializer: S) -> (Self::Ffi, ResolveBytes)
    where
        S: Serializer + Send + Sync + 'static;
}
