include!("../src/main.rs");

#[cfg(test)]
mod state_machine_adversaries {
    use super::*;

    fn event(epoch: u64, kind: u32, tid: u32, tgid: u32, value: u32) -> MetadataEvent {
        MetadataEvent {
            epoch,
            kind,
            tid,
            tgid,
            value,
            reserved: 0,
        }
    }

    #[test]
    fn concurrent_session_is_rejected_without_replacing_active_session() {
        let mut tracker = Tracker::default();
        tracker.begin(11, 101).expect("first session must start");

        let error = tracker
            .begin(12, 202)
            .expect_err("second active session must be rejected");

        assert!(error
            .to_string()
            .contains("concurrent persistent observation session"));
        let active = tracker
            .active
            .as_ref()
            .expect("first session must remain active");
        assert_eq!(active.epoch, 11);
        assert_eq!(active.root_tid, 101);
    }

    #[test]
    fn stale_epoch_is_rejected_before_session_state_mutation() {
        let mut session = ActiveSession::new(22, 220);

        session.accept(event(21, EVENT_EXEC, 220, 220, 0));

        assert_eq!(session.stale_epoch_events, 1);
        assert_eq!(session.sequence, 0);
        assert_eq!(session.event_count, 0);
        assert_eq!(session.exec_count, 0);
        assert_eq!(session.integrity_errors, 0);
        assert_eq!(session.active, BTreeSet::from([220]));
    }

    #[test]
    fn unknown_event_kind_marks_integrity_failure() {
        let mut session = ActiveSession::new(33, 330);

        session.accept(event(33, 0xffff_fffe, 330, 330, 0));

        assert_eq!(session.stale_epoch_events, 0);
        assert_eq!(session.event_count, 1);
        assert_eq!(session.sequence, 1);
        assert_eq!(session.integrity_errors, 1);
        assert_eq!(session.spawn_count, 0);
        assert_eq!(session.exec_count, 0);
        assert_eq!(session.exit_count, 0);
        assert!(session.active.contains(&330));
    }

    #[test]
    fn event_budget_exhaustion_fails_closed_before_accepting_more_events() {
        let mut session = ActiveSession::new(44, 440);
        session.event_count = MAX_SESSION_EVENTS;

        session.accept(event(44, EVENT_EXEC, 440, 440, 0));

        assert!(session.event_limit_hit);
        assert_eq!(session.event_count, MAX_SESSION_EVENTS);
        assert_eq!(session.sequence, 0);
        assert_eq!(session.exec_count, 0);
        assert_eq!(session.integrity_errors, 0);
    }

    #[test]
    fn malformed_wire_record_is_rejected() {
        assert!(decode_event(&[0_u8; 31]).is_err());
        assert!(decode_event(&[0_u8; 33]).is_err());
    }

    #[test]
    fn target_launch_failure_happens_before_session_activation() {
        let tracker = Tracker::default();
        let missing = PathBuf::from("/definitely/missing/execsurface-m8-7-fixture");

        let result = spawn_blocked(&missing);

        assert!(result.is_err());
        assert!(tracker.active.is_none());
        assert_eq!(tracker.unexpected_without_session, 0);
        assert_eq!(tracker.decode_errors, 0);
    }

    #[test]
    fn barrier_release_failure_is_explicit_error() {
        let result = release_target(-1);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "failed to release root-registration launch barrier");
    }
}
