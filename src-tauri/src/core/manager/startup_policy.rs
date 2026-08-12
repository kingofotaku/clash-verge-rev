pub(crate) const fn service_required_for_startup(tun_enabled: bool, tun_suppressed: bool, is_admin: bool) -> bool {
    tun_enabled && !tun_suppressed && !is_admin
}

#[cfg(test)]
mod tests {
    use super::service_required_for_startup;

    #[test]
    fn elevated_tun_does_not_require_service() {
        assert!(!service_required_for_startup(true, false, true));
    }

    #[test]
    fn non_elevated_tun_requires_service() {
        assert!(service_required_for_startup(true, false, false));
    }

    #[test]
    fn disabled_or_suppressed_tun_does_not_require_service() {
        assert!(!service_required_for_startup(false, false, false));
        assert!(!service_required_for_startup(true, true, false));
    }
}
