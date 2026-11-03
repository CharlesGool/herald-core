use std::cell::Cell;
use std::mem::size_of;

use rsbinder::{ExceptionCode, Status, StatusCode};

use super::*;
use crate::hook::rewrite::tests::*;

#[test]
fn generation_delay_precedes_key_publication_and_preserves_metadata() {
    use crate::android::hardware::security::keymint::KeyParameterValue::KeyParameterValue;

    let challenge = vec![KeyParameter {
        tag: Tag::ATTESTATION_CHALLENGE,
        value: KeyParameterValue::Blob(vec![7]),
    }];
    let plain = vec![KeyParameter {
        tag: Tag::APPLICATION_DATA,
        value: KeyParameterValue::Blob(vec![7]),
    }];
    for (params, delay, expected) in [
        (&challenge, 25, Some(Duration::from_millis(25))),
        (&challenge, 0, None),
        (&plain, 25, None),
    ] {
        let waited = Cell::new(None);
        let generation_calls = Cell::new(0);
        let generated = generate_key_with_delay(
            params,
            delay,
            |duration| {
                assert_eq!(generation_calls.get(), 0, "key must not exist during wait");
                waited.set(Some(duration));
            },
            || {
                assert_eq!(waited.get(), expected, "wait must precede generation");
                generation_calls.set(generation_calls.get() + 1);
                Ok(KeyMetadata {
                    key: KeyDescriptor {
                        domain: Domain::KEY_ID,
                        nspace: 123,
                        ..Default::default()
                    },
                    certificate: Some(vec![1, 2, 3]),
                    ..Default::default()
                })
            },
        )
        .unwrap();
        assert_eq!(waited.get(), expected);
        assert_eq!(generation_calls.get(), 1);
        let mut reply = parcel::build_plain_reply(&generated).unwrap();
        let decoded: KeyMetadata = parcel::parse_owned_success_reply(&mut reply).unwrap();
        assert_eq!(decoded.key.domain, Domain::KEY_ID);
        assert_eq!(decoded.key.nspace, 123);
        assert_eq!(decoded.certificate, Some(vec![1, 2, 3]));
    }
}

#[test]
fn generation_delay_precedes_failure_and_preserves_error() {
    use crate::android::hardware::security::keymint::KeyParameterValue::KeyParameterValue;

    let challenge = vec![KeyParameter {
        tag: Tag::ATTESTATION_CHALLENGE,
        value: KeyParameterValue::Blob(vec![7]),
    }];
    for error in [
        Status::new_service_specific_error(ResponseCode::PERMISSION_DENIED.0, None),
        Status::from(StatusCode::DeadObject),
    ] {
        let expected_error = format!("{error:?}");
        let waited = Cell::new(false);
        let generation_calls = Cell::new(0);
        let error = generate_key_with_delay(
            &challenge,
            25,
            |duration| {
                assert_eq!(generation_calls.get(), 0);
                assert_eq!(duration, Duration::from_millis(25));
                waited.set(true);
            },
            || {
                assert!(waited.get(), "errors must also follow the configured wait");
                generation_calls.set(generation_calls.get() + 1);
                Err(error)
            },
        )
        .unwrap_err();
        assert_eq!(generation_calls.get(), 1);
        assert_eq!(format!("{error:?}"), expected_error);
    }
}

pub(super) fn assert_unknown_transaction_reply(reply: SyntheticReply) {
    let SyntheticReply::Status(status) = reply else {
        panic!("unknown transaction should be returned as a binder status code");
    };
    assert_eq!(status, i32::from(StatusCode::UnknownTransaction));
}

pub(super) fn assert_synthetic_status(reply: SyntheticReply, expected: StatusCode) {
    let SyntheticReply::Status(status) = reply else {
        panic!("expected synthetic binder status");
    };
    assert_eq!(status, i32::from(expected));
}

pub(super) fn assert_synthetic_ok_reply(reply: SyntheticReply, label: &str) {
    let SyntheticReply::Parcel(mut reply) = reply else {
        panic!("{label} should be a status-bearing parcel reply");
    };
    assert_eq!(reply.offsets_size(), 0);
    let (data, data_size, offsets, offsets_size) = raw_parts(&mut reply);
    let status = unsafe { parcel::parse_reply_status(data, data_size, offsets, offsets_size) }
        .expect("status reply should parse");
    assert!(status.is_ok(), "{label} should be OK");
}

pub(super) fn assert_synthetic_empty_parcel_reply(reply: SyntheticReply, label: &str) {
    let SyntheticReply::Parcel(reply) = reply else {
        panic!("{label} should be an empty parcel reply");
    };
    assert_eq!(reply.data_size(), 0, "{label} data size");
    assert_eq!(reply.offsets_size(), 0, "{label} offsets size");
}

pub(super) fn assert_synthetic_exception_reply(reply: SyntheticReply, expected: ExceptionCode) {
    let SyntheticReply::Parcel(mut reply) = reply else {
        panic!("expected synthetic status parcel reply");
    };
    let (data, data_size, offsets, offsets_size) = raw_parts(&mut reply);
    let status = unsafe { parcel::parse_reply_status(data, data_size, offsets, offsets_size) }
        .expect("status reply should parse");
    assert_eq!(status.exception_code(), expected);
}

pub(super) fn assert_synthetic_raw_i32_reply(reply: SyntheticReply, expected: i32) {
    let SyntheticReply::Parcel(mut reply) = reply else {
        panic!("raw i32 reply should be a parcel");
    };
    assert_eq!(reply.data_size(), size_of::<i32>());
    assert_eq!(reply.offsets_size(), 0);
    let (data, _, _, _) = raw_parts(&mut reply);
    let value = unsafe { std::ptr::read_unaligned(data as *const i32) };
    assert_eq!(value, expected);
}

#[test]
fn plain_herald_error_becomes_system_error_reply() {
    let error = anyhow::anyhow!("plain HERALD failure");
    let mut reply =
        build_herald_error_reply(&error).expect("plain error should produce a status reply");
    let (data, data_size, offsets, offsets_size) = raw_parts(&mut reply);
    let parsed = unsafe { parcel::parse_reply_status(data, data_size, offsets, offsets_size) }
        .expect("status reply should parse");

    assert_eq!(
        parsed.exception_code(),
        rsbinder::ExceptionCode::ServiceSpecific
    );
    assert_eq!(
        parsed.service_specific_error(),
        ResponseCode::SYSTEM_ERROR.0
    );
}

#[test]
fn contextual_herald_status_error_keeps_service_specific_code() {
    let status = Status::new_service_specific_error(ResponseCode::PERMISSION_DENIED.0, None);
    let error = anyhow::Error::new(status).context("wrapped HERALD failure");
    let mut reply =
        build_herald_error_reply(&error).expect("wrapped status should produce a status reply");
    let (data, data_size, offsets, offsets_size) = raw_parts(&mut reply);
    let parsed = unsafe { parcel::parse_reply_status(data, data_size, offsets, offsets_size) }
        .expect("status reply should parse");

    assert_eq!(
        parsed.exception_code(),
        rsbinder::ExceptionCode::ServiceSpecific
    );
    assert_eq!(
        parsed.service_specific_error(),
        ResponseCode::PERMISSION_DENIED.0
    );
}

#[test]
fn unavailable_herald_errors_preserve_system_reply() {
    for status in [
        StatusCode::DeadObject,
        StatusCode::RpcError,
        StatusCode::NotEnoughData,
        StatusCode::NoInit,
        StatusCode::NameNotFound,
        StatusCode::Errno(libc::ECONNREFUSED),
        StatusCode::Errno(libc::EPIPE),
    ] {
        let error = anyhow::Error::new(Status::from(status));
        assert!(
            build_herald_error_reply_or_preserve_system(&error)
                .expect("unavailable HERALD errors should classify cleanly")
                .is_none(),
            "{status:?} after retry means HERALD is unavailable, not authoritative"
        );
    }

    for (message, unavailable) in [
        ("failed to connect to herald service", true),
        ("failed to connect to herald_maintenance service", true),
        ("failed to connect to herald_authorization service", true),
        ("failed to connect to herald service: permission denied", false),
        (
            "failed to connect to herald_maintenance service: permission denied",
            false,
        ),
        (
            "failed to connect to herald_authorization service: permission denied",
            false,
        ),
    ] {
        assert_eq!(
            herald_unavailable_error(&anyhow::anyhow!(message)),
            unavailable,
            "HERALD connection marker classification mismatch for {message}"
        );
    }

    for marker in [
        "failed to connect to herald service",
        "failed to connect to herald_maintenance service",
        "failed to connect to herald_authorization service",
    ] {
        let error = anyhow::Error::new(StatusCode::PermissionDenied).context(marker);
        assert!(
            !herald_unavailable_error(&error),
            "typed permission error must override connection marker {marker}"
        );
    }
    let stale = anyhow::Error::new(StatusCode::DeadObject)
        .context("failed to connect to herald_authorization service");
    assert!(herald_unavailable_error(&stale));

    let missing_service = anyhow::Error::new(StatusCode::NameNotFound);
    assert!(
        build_herald_error_reply_or_preserve_system(&missing_service)
            .expect("missing RPC service should classify cleanly")
            .is_none(),
        "missing HERALD RPC service means HERALD is unavailable"
    );

    for status in [
        StatusCode::DeadObject,
        StatusCode::RpcError,
        StatusCode::NotEnoughData,
    ] {
        for error in [
            anyhow::Error::new(Status::from(status)).context("wrapped status"),
            anyhow::Error::new(status).context("wrapped status code"),
        ] {
            assert!(build_herald_error_reply_or_preserve_system(&error)
                .expect("wrapped unavailable errors should classify cleanly")
                .is_none());
        }
    }

    let local = anyhow::anyhow!("plain HERALD failure");
    assert!(
        build_herald_error_reply_or_preserve_system(&local)
            .expect("plain HERALD errors should classify cleanly")
            .is_some(),
        "non-connection HERALD errors must be returned instead of preserving system"
    );
}

#[test]
fn reachable_non_stale_herald_status_code_errors_become_system_error_reply() {
    // AOSP keystore2 maps a bare transport StatusCode through
    // map_binder_status_code -> Error::BinderTransaction -> SYSTEM_ERROR,
    // surfaced as a service-specific parcel, never a raw transport status.
    for status in [
        StatusCode::TimedOut,
        StatusCode::PermissionDenied,
        StatusCode::UnknownTransaction,
    ] {
        let error = anyhow::Error::new(status);
        let mut reply = build_herald_error_reply_or_preserve_system(&error)
            .expect("reachable HERALD errors should classify cleanly")
            .expect("non-stale HERALD errors must replace system reply");
        let (data, data_size, offsets, offsets_size) = raw_parts(&mut reply);
        let parsed = unsafe { parcel::parse_reply_status(data, data_size, offsets, offsets_size) }
            .expect("status reply should parse");
        assert_eq!(
            parsed.exception_code(),
            rsbinder::ExceptionCode::ServiceSpecific
        );
        assert_eq!(
            parsed.service_specific_error(),
            ResponseCode::SYSTEM_ERROR.0
        );
    }
}

#[test]
fn reachable_herald_status_error_becomes_authoritative_reply() {
    let status = Status::new_service_specific_error(ResponseCode::PERMISSION_DENIED.0, None);
    let reply = build_herald_status_reply_or_preserve_system(&status)
        .expect("service-specific status should build")
        .expect("reachable HERALD status should replace system");
    let mut reply = reply;
    let (data, data_size, offsets, offsets_size) = raw_parts(&mut reply);
    let parsed = unsafe { parcel::parse_reply_status(data, data_size, offsets, offsets_size) }
        .expect("status reply should parse");

    assert_eq!(
        parsed.exception_code(),
        rsbinder::ExceptionCode::ServiceSpecific
    );
    assert_eq!(
        parsed.service_specific_error(),
        ResponseCode::PERMISSION_DENIED.0
    );
}
