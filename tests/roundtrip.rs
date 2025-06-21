// Since this is an external test, we import from our library by its crate name.
// Rust converts the crate name "rust-wa-protocol-proto" into "rust_wa_protocol_proto" for use statements.
use rust_wa_protocol_proto::waproto::{HandshakeMessage, handshake_message::ClientHello};

// We still need the Message trait to get the .encode() and .decode() methods.
use prost::Message;

// The #[test] attribute marks this function as a test.
#[test]
fn test_handshake_message_roundtrip() {
    // 1. Create an instance of your struct with some example data.
    // Adjust the fields here to match your actual struct definition.
    let original_message = HandshakeMessage {
        client_hello: Some(ClientHello {
            ephemeral: Some(b"my_external_public_key".to_vec()),
            ..Default::default()
        }),
        ..Default::default()
    };

    // 2. Encode the struct into a byte vector.
    let mut buf = Vec::new();
    original_message.encode(&mut buf).expect("Encoding failed");

    // 3. Decode the bytes back into a new instance of the struct.
    let decoded_message = HandshakeMessage::decode(&buf[..]).expect("Decoding failed");

    // 4. Assert that the original and decoded structs are identical.
    assert_eq!(original_message, decoded_message);

    println!("External roundtrip test passed!");
}
