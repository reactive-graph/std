use std::sync::Arc;
use std::sync::RwLock;
use std::time::Instant;

use async_trait::async_trait;
use chrono::Datelike;
use log::info;
use log::trace;
use serde_json::json;

use crate::api::TimeGraph;
use crate::builder::EntityInstanceBuilder;
use crate::builder::RelationInstanceBuilder;
use crate::di::*;
use crate::model::PropertyInstanceGetter;
use crate::model::ReactiveEntityInstance;
use crate::model_date_time::DayOfMonthProperties;
use crate::model_date_time::DayProperties::DAY_OF_MONTH;
use crate::model_date_time::DayProperties::ISO8601;
use crate::model_date_time::MonthOfYearProperties;
use crate::model_date_time::MonthProperties::MONTH_AND_YEAR;
use crate::model_date_time::MonthProperties::MONTH_OF_YEAR;
use crate::model_date_time::YearProperties::LEAP;
use crate::model_date_time::YearProperties::YEAR;
use crate::model_date_time::ENTITY_TYPE_DAY;
use crate::model_date_time::ENTITY_TYPE_MONTH;
use crate::model_date_time::ENTITY_TYPE_YEAR;
use crate::model_date_time::RELATION_TYPE_DAY_OF_MONTH;
use crate::model_date_time::RELATION_TYPE_FIRST_DAY;
use crate::model_date_time::RELATION_TYPE_FIRST_MONTH;
use crate::model_date_time::RELATION_TYPE_LAST_DAY;
use crate::model_date_time::RELATION_TYPE_LAST_MONTH;
use crate::model_date_time::RELATION_TYPE_MONTH_OF_YEAR;
use crate::model_date_time::RELATION_TYPE_NEXT_DAY;
use crate::model_date_time::RELATION_TYPE_NEXT_MONTH;
use crate::model_date_time::RELATION_TYPE_NEXT_YEAR;
use crate::plugins::PluginContext;

#[wrapper]
pub struct PluginContextContainer(Arc<RwLock<Option<Arc<dyn PluginContext>>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    PluginContextContainer(Arc::new(RwLock::new(None)))
}

#[component]
pub struct TimeGraphImpl {
    context: PluginContextContainer,
}

impl TimeGraphImpl {
    async fn create_time_graph(&self) {
        let context = self.context.0.clone();
        async_std::task::spawn(async move {
            info!("Start generating time graph");
            let start = Instant::now();
            let guard = context.read().unwrap();
            if let Some(context) = guard.clone() {
                create_years(&context);
                let duration = start.elapsed();
                info!("Successfully generated time graph in {:?}", duration);
            }
        });
    }
}

#[async_trait]
#[provides]
impl TimeGraph for TimeGraphImpl {
    async fn init(&self) {
        self.create_time_graph().await;
    }

    async fn shutdown(&self) {}

    fn set_context(&self, context: Arc<dyn PluginContext>) {
        self.context.0.write().unwrap().replace(context.clone());
    }
}

fn create_years(context: &Arc<dyn PluginContext>) {
    let mut previous_year = None;
    let mut previous_year_last_month: Option<Arc<ReactiveEntityInstance>> = None;
    let mut previous_year_last_day: Option<Arc<ReactiveEntityInstance>> = None;
    let current_year = chrono::Utc::now().year() as i64;
    for year in current_year - 1..current_year + 1 {
        previous_year = match create_year(context, year) {
            Some(current_year) => {
                create_next_year(context, &previous_year, &current_year);
                let (current_year_last_month, current_year_last_day) = create_months(context, &current_year, previous_year_last_month, previous_year_last_day);
                previous_year_last_month = current_year_last_month;
                previous_year_last_day = current_year_last_day;
                Some(current_year)
            }
            None => None,
        }
    }
}

fn create_year(context: &Arc<dyn PluginContext>, year: i64) -> Option<Arc<ReactiveEntityInstance>> {
    trace!("Create year {}", year);
    let entity_instance_manager = context.get_entity_instance_manager();
    let entity_instance = EntityInstanceBuilder::new(ENTITY_TYPE_YEAR.clone())
        .property(YEAR, json!(year))
        .property(LEAP, json!(is_leap_year(year)))
        .build();
    entity_instance_manager.create(entity_instance).ok()
}

fn create_next_year(context: &Arc<dyn PluginContext>, previous_year: &Option<Arc<ReactiveEntityInstance>>, next_year: &Arc<ReactiveEntityInstance>) {
    let Some(previous_year)= previous_year else {
        return;
    };
    let relation_instance_manager = context.get_relation_instance_manager();
    let instance_id = format!("{}__{}", previous_year.as_i64(YEAR).unwrap_or(0), next_year.as_i64(YEAR).unwrap_or(0));
    let relation_instance =
        RelationInstanceBuilder::new_unique_for_instance_id(previous_year.id, RELATION_TYPE_NEXT_YEAR.clone(), instance_id, next_year.id).build();
    let _ = relation_instance_manager.create(relation_instance);
}

fn create_months(
    context: &Arc<dyn PluginContext>,
    current_year: &Arc<ReactiveEntityInstance>,
    previous_year_last_month: Option<Arc<ReactiveEntityInstance>>,
    previous_year_last_day: Option<Arc<ReactiveEntityInstance>>,
) -> (Option<Arc<ReactiveEntityInstance>>, Option<Arc<ReactiveEntityInstance>>) {
    let Some(year) = current_year.as_i64(YEAR) else {
        return (None, None);
    };
    let mut previous_month = previous_year_last_month;
    let mut previous_month_last_day = previous_year_last_day;
    // let mut last_month_last_day: Option<Arc<ReactiveEntityInstance>> = None;
    for month in 1..=12 {
        let current_month = match create_month(context, year, month) {
            Some(current_month) => {
                create_next_month(context, &previous_month, &current_month);
                create_month_of_year(context, current_year, &current_month);
                previous_month_last_day = create_days(context, current_year, &current_month, previous_month_last_day);
                Some(current_month)
            }
            None => None,
        };
        if month == 1 {
            previous_month = match current_month {
                Some(current_month) => {
                    create_first_month(context, current_year, &current_month);
                    // create_next_month(context, previous_year_last_month, &current_month);
                    Some(current_month)
                }
                None => None,
            };
        } else if month == 12 {
            previous_month = match current_month {
                Some(current_month) => {
                    create_last_month(context, current_year, &current_month);
                    Some(current_month)
                }
                None => None,
            };
            // last_month_last_day = previous_month_last_day;
        } else {
            previous_month = current_month;
        }
    }
    (previous_month, previous_month_last_day)
}

fn create_month(context: &Arc<dyn PluginContext>, year: i64, month: u64) -> Option<Arc<ReactiveEntityInstance>> {
    let entity_instance_manager = context.get_entity_instance_manager();
    let entity_instance = EntityInstanceBuilder::new(ENTITY_TYPE_MONTH.clone())
        .property(MONTH_OF_YEAR, json!(month))
        .property(MONTH_AND_YEAR, json!(format!("{:04}-{:02}", year, month)))
        .build();
    entity_instance_manager.create(entity_instance).ok()
}

fn create_month_of_year(context: &Arc<dyn PluginContext>, current_year: &Arc<ReactiveEntityInstance>, month: &Arc<ReactiveEntityInstance>) {
    let Some(year) = current_year.as_i64(YEAR) else {
        return;
    };
    let Some(month_of_year) = month.as_u64(MONTH_OF_YEAR) else {
        return;
    };
    let relation_instance_manager = context.get_relation_instance_manager();
    let instance_id = format!("{:04}__{:02}", year, month_of_year);
    let relation_instance = RelationInstanceBuilder::new_unique_for_instance_id(current_year.id, RELATION_TYPE_MONTH_OF_YEAR.clone(), instance_id, month.id)
        .property(MonthOfYearProperties::MONTH_OF_YEAR, json!(month_of_year))
        .build();
    let _ = relation_instance_manager.create(relation_instance);
}

fn create_first_month(context: &Arc<dyn PluginContext>, current_year: &Arc<ReactiveEntityInstance>, first_month: &Arc<ReactiveEntityInstance>) {
    let relation_instance_manager = context.get_relation_instance_manager();
    let instance_id = format!("{:04}__{:02}", current_year.as_i64(YEAR).unwrap_or(0), first_month.as_u64(MONTH_OF_YEAR).unwrap_or(0));
    let relation_instance =
        RelationInstanceBuilder::new_unique_for_instance_id(current_year.id, RELATION_TYPE_FIRST_MONTH.clone(), instance_id, first_month.id).build();
    let _ = relation_instance_manager.create(relation_instance);
}

fn create_last_month(context: &Arc<dyn PluginContext>, current_year: &Arc<ReactiveEntityInstance>, last_month: &Arc<ReactiveEntityInstance>) {
    let relation_instance_manager = context.get_relation_instance_manager();
    let instance_id = format!("{:04}__{:02}", current_year.as_i64(YEAR).unwrap_or(0), last_month.as_u64(MONTH_OF_YEAR).unwrap_or(0));
    let relation_instance =
        RelationInstanceBuilder::new_unique_for_instance_id(current_year.id, RELATION_TYPE_LAST_MONTH.clone(), instance_id, last_month.id).build();
    let _ = relation_instance_manager.create(relation_instance);
}

fn create_next_month(context: &Arc<dyn PluginContext>, previous_month: &Option<Arc<ReactiveEntityInstance>>, next_month: &Arc<ReactiveEntityInstance>) {
    let Some(previous_month)= previous_month else {
        return;
    };
    let relation_instance_manager = context.get_relation_instance_manager();
    let instance_id = format!(
        "{}__{}",
        previous_month.as_string(MONTH_AND_YEAR).unwrap_or(String::new()),
        next_month.as_string(MONTH_AND_YEAR).unwrap_or(String::new())
    );
    let relation_instance =
        RelationInstanceBuilder::new_unique_for_instance_id(previous_month.id, RELATION_TYPE_NEXT_MONTH.clone(), instance_id, next_month.id).build();
    let _ = relation_instance_manager.create(relation_instance);
}

fn create_days(
    context: &Arc<dyn PluginContext>,
    current_year: &Arc<ReactiveEntityInstance>,
    current_month: &Arc<ReactiveEntityInstance>,
    previous_month_last_day: Option<Arc<ReactiveEntityInstance>>,
) -> Option<Arc<ReactiveEntityInstance>> {
    let Some(year) = current_year.as_i64(YEAR) else {
        return None;
    };
    let Some(month_of_year) = current_month.as_u64(MONTH_OF_YEAR) else {
        return None;
    };
    let Some(last_day_of_month) = last_day_of_month(year, month_of_year) else {
        return None;
    };
    let mut previous_day = previous_month_last_day;
    for day_of_month in 1..=last_day_of_month {
        let current_day = match create_day(context, year, month_of_year, day_of_month) {
            Some(current_day) => {
                create_next_day(context, &previous_day, &current_day);
                create_day_of_month(context, current_month, &current_day);
                Some(current_day)
            }
            None => None,
        };
        if day_of_month == 1 {
            previous_day = match current_day {
                Some(current_day) => {
                    create_first_day(context, current_month, &current_day);
                    // create_next_day(context, previous_month_last_day, &current_day);
                    Some(current_day)
                }
                None => None,
            };
        } else if day_of_month == last_day_of_month {
            previous_day = match current_day {
                Some(current_day) => {
                    create_last_day(context, current_month, &current_day);
                    Some(current_day)
                }
                None => None,
            };
        } else {
            previous_day = current_day;
        }
    }
    previous_day
}

fn create_day(context: &Arc<dyn PluginContext>, year: i64, month: u64, day: u64) -> Option<Arc<ReactiveEntityInstance>> {
    let iso8601 = format!("{:04}-{:02}-{:02}", year, month, day);
    let entity_instance_manager = context.get_entity_instance_manager();
    let entity_instance = EntityInstanceBuilder::new(ENTITY_TYPE_DAY.clone())
        .property(DAY_OF_MONTH, json!(day))
        .property(ISO8601, json!(iso8601))
        .build();
    entity_instance_manager.create(entity_instance).ok()
}

fn create_day_of_month(context: &Arc<dyn PluginContext>, current_month: &Arc<ReactiveEntityInstance>, current_day: &Arc<ReactiveEntityInstance>) {
    let Some(month_and_year) = current_month.as_string(MONTH_AND_YEAR) else {
        return;
    };
    let Some(day_of_month) = current_day.as_u64(DAY_OF_MONTH) else {
        return;
    };
    let relation_instance_manager = context.get_relation_instance_manager();
    let instance_id = format!("{:04}__{:02}", month_and_year, day_of_month);
    let relation_instance =
        RelationInstanceBuilder::new_unique_for_instance_id(current_month.id, RELATION_TYPE_DAY_OF_MONTH.clone(), instance_id, current_day.id)
            .property(DayOfMonthProperties::DAY_OF_MONTH, json!(day_of_month))
            .build();
    let _ = relation_instance_manager.create(relation_instance);
}

fn create_first_day(context: &Arc<dyn PluginContext>, current_month: &Arc<ReactiveEntityInstance>, first_day: &Arc<ReactiveEntityInstance>) {
    let Some(month_and_year) = current_month.as_string(MONTH_AND_YEAR) else {
        return;
    };
    let Some(day_of_month) = first_day.as_u64(DAY_OF_MONTH) else {
        return;
    };
    let relation_instance_manager = context.get_relation_instance_manager();
    let instance_id = format!("{:04}__{:02}", month_and_year, day_of_month);
    let relation_instance =
        RelationInstanceBuilder::new_unique_for_instance_id(current_month.id, RELATION_TYPE_FIRST_DAY.clone(), instance_id, first_day.id).build();
    let _ = relation_instance_manager.create(relation_instance);
}

fn create_last_day(context: &Arc<dyn PluginContext>, current_month: &Arc<ReactiveEntityInstance>, last_day: &Arc<ReactiveEntityInstance>) {
    let Some(month_and_year) = current_month.as_string(MONTH_AND_YEAR) else {
        return;
    };
    let Some(day_of_month) = last_day.as_u64(DAY_OF_MONTH) else {
        return;
    };
    let relation_instance_manager = context.get_relation_instance_manager();
    let instance_id = format!("{:04}__{:02}", month_and_year, day_of_month);
    let relation_instance =
        RelationInstanceBuilder::new_unique_for_instance_id(current_month.id, RELATION_TYPE_LAST_DAY.clone(), instance_id, last_day.id).build();
    let _ = relation_instance_manager.create(relation_instance);
}

fn create_next_day(context: &Arc<dyn PluginContext>, previous_day: &Option<Arc<ReactiveEntityInstance>>, next_day: &Arc<ReactiveEntityInstance>) {
    let Some(previous_day)= previous_day else {
        return;
    };
    let relation_instance_manager = context.get_relation_instance_manager();
    let instance_id = format!("{}__{}", previous_day.as_string(ISO8601).unwrap_or_default(), next_day.as_string(ISO8601).unwrap_or_default());
    let relation_instance =
        RelationInstanceBuilder::new_unique_for_instance_id(previous_day.id, RELATION_TYPE_NEXT_DAY.clone(), instance_id, next_day.id).build();
    let _ = relation_instance_manager.create(relation_instance);
}

fn is_leap_year(year: i64) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn last_day_of_month(year: i64, month: u64) -> Option<u64> {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => Some(31),
        4 | 6 | 9 | 11 => Some(30),
        2 => {
            if is_leap_year(year) {
                Some(29)
            } else {
                Some(28)
            }
        }
        _ => None,
    }
}
