use use_cases::TagStatsResponse;

use crate::dto::{TagStatJson, TagStatsJson};

pub fn tag_stats_to_json(response: TagStatsResponse) -> TagStatsJson {
    TagStatsJson {
        tags: response
            .tags
            .into_iter()
            .map(|stat| TagStatJson {
                tag: stat.tag,
                duration_secs: stat.duration_secs,
                days_worked: stat.days_worked,
                worklog_count: stat.worklog_count,
            })
            .collect(),
    }
}
