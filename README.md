# `firestore-serde`

`firestore-serde` is a serializer/deserializer implementation for Firestore `Value`s and `Document`s.

This allows you to store arbitrary Rust values in a Firestore database, but this crate does _not_ handle
any of the communication with the Firestore service. Instead, it's meant
to fit into a stack of other crates:

- [`tonic`](https://github.com/hyperium/tonic) for connecting to Firebase
over [`gRPC`](https://grpc.io/).
- [`googapis`](https://github.com/mechiru/googapis) for Firebase gRPC message
definitions, derived from the [official API definitions](https://github.com/googleapis/googleapis) using [`prost`](https://github.com/tokio-rs/prost).
- [`gouth`](https://github.com/mechiru/gouth) for authentication.

## Preliminaries

Firestore is a cloud-based, proprietary document database from Google (not to be confused
with Firebase or Cloud Datastore, which are also proprietary document databases from Google).

Google does not provide official Rust bindings for Google Cloud Platform, but they _do_ provide
API specifications for REST and gRPC APIs. As a result, a number of Rust projects have appeared
to produce Rust bindings from these specifications:

- [`Byron/google-apis-rs`](https://github.com/Byron/google-apis-rs) produces bindings for the REST APIs, implemented on top of the [`hyper`](https://github.com/hyperium/hyper) HTTP library.
- [`mechiru/googapis`](https://github.com/mechiru/googapis) produces bindings for the gRPC APIs,
implemented on top of the [`tonic`](https://github.com/hyperium/tonic) gRPC library.

There is a 1:1 mapping between REST API calls and gRPC API calls. Unfortunately, there is a
[known issue](https://github.com/Byron/google-apis-rs/issues/220) in the REST API spec for
Firestore which breaks querying, so I recommend not using the REST API. As a consequence, this
crate only supports the gRPC API. If you do want to use the REST API, see the 
[`firestore-db-and-auth-rs`](https://github.com/davidgraeff/firestore-db-and-auth-rs)
crate instead of this one.

The unit of data retrieval in Firestore is a [`Document`](https://firebase.google.com/docs/firestore/reference/rpc/google.firestore.v1#google.firestore.v1.Document),
which is a map of string fields to [`Value`s](https://firebase.google.com/docs/firestore/reference/rpc/google.firestore.v1#google.firestore.v1.Value). `Value`s themselves are a rich
type which can represent arrays and maps composed of other values. As such, we can represent
many Rust types in an intuitive way as both `Value`s and `Document`s. This crate provides a
[`serde`](https://serde.rs/) serializer and deserializer to do just that.

## Usage

This crate provides four primary functions:

```rust
// Conversions between Rust types and Value gRPC type.

pub fn to_grpc_value<T>(value: &T)
    -> Result<Value, SerializationError>
    where T: Serialize;

pub fn from_grpc_value<T>(value: &Value)
    -> Result<T, DeserializationError>
    where T: DeserializeOwned;

// Conversions between Rust types and Document gRPC type.

pub fn to_document<T>(value: &T)
    -> Result<Document, SerializationError>
    where T: Serialize;

pub fn from_document<T>(document: Document)
    -> Result<T, DeserializationError>
    where T: DeserializeOwned;
```

Note that the `from_document` takes ownership of its argument, so if you need the original
`Document` after conversion you will have to clone it.

### Timestamps

The [chrono](https://github.com/chronotope/chrono) crate supports serializable timestamps, by
converting the timestamp to a string or number. Without intervention, `firestore-serde` doesn't
differentiate between these and other numbers or strings, so they are turned into
`ValueType::IntegerValue` and `ValueType::StringValue` respectively.

This is fine if you always deserialize the values to Rust, but if you want to access the data in other ways (for example, the web-based data console), it's often useful to store time data 
in Firestore's `ValueType::TimestampValue`. To do this, add `firestore-serde-timestamp` as a
dependency and tell Serde to use it as the encoding:

```rust
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize)]
struct MyStruct {
    #[serde(with="firestore_serde_timestamp::timestamp")]
    some_timestamp: DateTime<Utc>,
}
```

### API versions

There are currently two versions of the gRPC API, `google.firestore.v1.*` and
`google.firestore.v1beta1.*`. Each version is represented by a different namespace which is
isolated from the other, so even types which are used in common by both (e.g.
`Document` and `Value`) are represented by different protocol buffers.

`firestore-serde` uses the `v1` namespace by default, but it is possible to change that with
a feature flag. In your `Cargo.toml`, specify the dependency as follows:

```
[dependencies]
firestore-serde = {version = "0.1.0", default-features=false, features=["google-firestore-v1beta1"]}
```

It is important that you disable `default-features`, because `firestore-serde` will refuse
to compile if both `google-firestore-v1` and `google-firestore-v1beta1` are enabled.
