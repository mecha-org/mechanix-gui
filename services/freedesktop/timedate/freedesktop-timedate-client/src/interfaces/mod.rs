use crate::proxies::ProxyError;
use async_trait::async_trait;
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait TimeDateInterface: Send + Sync {
    async fn list_time_zones(&self) -> anyhow::Result<Vec<String>, ProxyError>;
    async fn set_local_rtc(
        &self,
        local_rtc: bool,
        fix_system: bool,
        interactive: bool,
    ) -> anyhow::Result<(), ProxyError>;

    /// Use SetNTP() to control whether the system clock is synchronized
    //     with the network using systemd-timesyncd. This will enable and
    //     start or disable and stop the chosen time synchronization service.

    async fn set_ntp(
        &self,
        use_ntp: bool,
        interactive: bool,
    ) -> anyhow::Result<(), ProxyError>;

    /// Use SetTime() to change the system clock. Pass a value of
    //        microseconds since the UNIX epoch (1 Jan 1970 UTC). If relative is
    //        true, the passed usec value will be added to the current system
    //        time. If it is false, the current system time will be set to the
    //        passed usec value. If the system time is set with this method, the
    //        RTC will be updated as well.
    async fn set_time(
        &self,
        usec_utc: i64,
        relative: bool,
        interactive: bool,
    ) -> anyhow::Result<(), ProxyError>;

    /// Use SetTimezone() to set the system timezone. Pass a value like
    //        "Europe/Berlin" to set the timezone. Valid timezones are listed in
    //        /usr/share/zoneinfo/zone.tab. If the RTC is configured to be
    //        maintained in local time, it will be updated accordingly.
    async fn set_timezone(
        &self,
        timezone: &str,
        interactive: bool,
    ) -> anyhow::Result<(), ProxyError>;

    async fn get_timezone(&self) -> anyhow::Result<String, ProxyError>;

    async fn get_time_usec(&self) -> anyhow::Result<u64, ProxyError>;
}
