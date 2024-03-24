use std::sync::Arc;
use std::time::Instant;

use async_trait::async_trait;
use chrono::Datelike;
use log::info;
use log::trace;
use serde_json::json;

use crate::api::TimeGraph;
use inexor_rgf_graph::prelude::*;
use inexor_rgf_plugin_api::component_alias;
use inexor_rgf_plugin_api::Component;
use inexor_rgf_plugin_api::EntityInstanceManager;
use inexor_rgf_plugin_api::RelationInstanceManager;

use inexor_rgf_model_date_time::DayProperties::DAY_OF_MONTH;
use inexor_rgf_model_date_time::DayProperties::ISO8601;
use inexor_rgf_model_date_time::MonthProperties::MONTH_AND_YEAR;
use inexor_rgf_model_date_time::MonthProperties::MONTH_OF_YEAR;
use inexor_rgf_model_date_time::YearProperties::LEAP;
use inexor_rgf_model_date_time::YearProperties::YEAR;
use inexor_rgf_model_date_time::ENTITY_TYPE_DAY;
use inexor_rgf_model_date_time::ENTITY_TYPE_MONTH;
use inexor_rgf_model_date_time::ENTITY_TYPE_YEAR;
use inexor_rgf_model_date_time::RELATION_TYPE_DAY_OF_MONTH;
use inexor_rgf_model_date_time::RELATION_TYPE_FIRST_DAY;
use inexor_rgf_model_date_time::RELATION_TYPE_FIRST_MONTH;
use inexor_rgf_model_date_time::RELATION_TYPE_LAST_DAY;
use inexor_rgf_model_date_time::RELATION_TYPE_LAST_MONTH;
use inexor_rgf_model_date_time::RELATION_TYPE_MONTH_OF_YEAR;
use inexor_rgf_model_date_time::RELATION_TYPE_NEXT_DAY;
use inexor_rgf_model_date_time::RELATION_TYPE_NEXT_MONTH;
use inexor_rgf_model_date_time::RELATION_TYPE_NEXT_YEAR;
use inexor_rgf_reactive_model_impl::ReactiveEntity;
use inexor_rgf_reactive_model_impl::ReactiveProperties;
use inexor_rgf_reactive_model_impl::ReactiveRelation;
use uuid::Uuid;

#[derive(Component)]
pub struct TimeGraphImpl {
    #[component(default = "crate::plugin::entity_instance_manager")]
    entity_instance_manager: Arc<dyn EntityInstanceManager + Send + Sync>,

    #[component(default = "crate::plugin::relation_instance_manager")]
    relation_instance_manager: Arc<dyn RelationInstanceManager + Send + Sync>,
}

impl TimeGraphImpl {
    async fn create_time_graph(&self) {
        self.create_years().await;
    }

    async fn create_years<'a>(&self) {
        let mut previous_year = None;
        let mut previous_year_last_month: Option<ReactiveEntity> = None;
        let mut previous_year_last_day: Option<ReactiveEntity> = None;
        let current_year = chrono::Utc::now().year() as i64;
        for year in current_year - 1..current_year + 1 {
            previous_year = match self.create_year(year).await {
                Some(current_year) => {
                    self.create_next_year(&previous_year, &current_year).await;
                    let (current_year_last_month, current_year_last_day) =
                        self.create_months(&current_year, previous_year_last_month, previous_year_last_day).await;
                    previous_year_last_month = current_year_last_month;
                    previous_year_last_day = current_year_last_day;
                    Some(current_year)
                }
                None => None,
            }
        }
    }

    async fn create_year(&self, year: i64) -> Option<ReactiveEntity> {
        trace!("Create year {}", year);
        let id = Uuid::new_v4();
        let properties = PropertyInstances::new().property(YEAR, json!(year)).property(LEAP, json!(is_leap_year(year)));
        let reactive_entity = ReactiveEntity::builder()
            .ty(ENTITY_TYPE_YEAR.clone())
            .id(id)
            .properties(ReactiveProperties::new_with_id_from_properties(id, properties))
            .build();
        self.entity_instance_manager.register(reactive_entity).ok()
    }

    async fn create_next_year(&self, previous_year: &Option<ReactiveEntity>, next_year: &ReactiveEntity) {
        let Some(previous_year) = previous_year else {
            return;
        };
        let instance_id = format!("{}__{}", previous_year.as_i64(YEAR).unwrap_or(0), next_year.as_i64(YEAR).unwrap_or(0));
        let ty = RelationInstanceTypeId::new_unique_for_instance_id(RELATION_TYPE_NEXT_YEAR.clone(), instance_id);
        let reactive_relation = ReactiveRelation::builder_with_entities(previous_year.clone(), &ty, next_year.clone()).build();
        let _ = self.relation_instance_manager.register(reactive_relation);
    }

    async fn create_months(
        &self,
        current_year: &ReactiveEntity,
        previous_year_last_month: Option<ReactiveEntity>,
        previous_year_last_day: Option<ReactiveEntity>,
    ) -> (Option<ReactiveEntity>, Option<ReactiveEntity>) {
        let Some(year) = current_year.as_i64(YEAR) else {
            return (None, None);
        };
        let mut previous_month = previous_year_last_month;
        let mut previous_month_last_day = previous_year_last_day;
        // let mut last_month_last_day: Option<Arc<ReactiveEntityInstance>> = None;
        for month in 1..=12 {
            let current_month = match self.create_month(year, month).await {
                Some(current_month) => {
                    self.create_next_month(&previous_month, &current_month).await;
                    self.create_month_of_year(current_year, &current_month).await;
                    previous_month_last_day = self.create_days(current_year, &current_month, previous_month_last_day).await;
                    Some(current_month)
                }
                None => None,
            };
            if month == 1 {
                previous_month = match current_month {
                    Some(current_month) => {
                        self.create_first_month(current_year, &current_month).await;
                        // create_next_month(context, previous_year_last_month, &current_month);
                        Some(current_month)
                    }
                    None => None,
                };
            } else if month == 12 {
                previous_month = match current_month {
                    Some(current_month) => {
                        self.create_last_month(current_year, &current_month).await;
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

    async fn create_month(&self, year: i64, month: u64) -> Option<ReactiveEntity> {
        let id = Uuid::new_v4();
        let properties = PropertyInstances::new()
            .property(MONTH_OF_YEAR, json!(month))
            .property(MONTH_AND_YEAR, json!(format!("{:04}-{:02}", year, month)));
        let reactive_entity = ReactiveEntity::builder()
            .ty(ENTITY_TYPE_MONTH.clone())
            .id(id)
            .properties(ReactiveProperties::new_with_id_from_properties(id, properties))
            .build();
        self.entity_instance_manager.register(reactive_entity).ok()
    }

    async fn create_month_of_year(&self, current_year: &ReactiveEntity, month: &ReactiveEntity) {
        let Some(year) = current_year.as_i64(YEAR) else {
            return;
        };
        let Some(month_of_year) = month.as_u64(MONTH_OF_YEAR) else {
            return;
        };
        let instance_id = format!("{:04}__{:02}", year, month_of_year);
        let ty = RelationInstanceTypeId::new_unique_for_instance_id(RELATION_TYPE_MONTH_OF_YEAR.clone(), instance_id);
        let reactive_relation = ReactiveRelation::builder_with_entities(current_year.clone(), &ty, month.clone()).build();
        let _ = self.relation_instance_manager.register(reactive_relation);
    }

    async fn create_first_month(&self, current_year: &ReactiveEntity, first_month: &ReactiveEntity) {
        let instance_id = format!("{:04}__{:02}", current_year.as_i64(YEAR).unwrap_or(0), first_month.as_u64(MONTH_OF_YEAR).unwrap_or(0));
        let ty = RelationInstanceTypeId::new_unique_for_instance_id(RELATION_TYPE_FIRST_MONTH.clone(), instance_id);
        let reactive_relation = ReactiveRelation::builder_with_entities(current_year.clone(), &ty, first_month.clone()).build();
        let _ = self.relation_instance_manager.register(reactive_relation);
    }

    async fn create_last_month(&self, current_year: &ReactiveEntity, last_month: &ReactiveEntity) {
        let instance_id = format!("{:04}__{:02}", current_year.as_i64(YEAR).unwrap_or(0), last_month.as_u64(MONTH_OF_YEAR).unwrap_or(0));
        let ty = RelationInstanceTypeId::new_unique_for_instance_id(RELATION_TYPE_LAST_MONTH.clone(), instance_id);
        let reactive_relation = ReactiveRelation::builder_with_entities(current_year.clone(), &ty, last_month.clone()).build();
        let _ = self.relation_instance_manager.register(reactive_relation);
    }

    async fn create_next_month(&self, previous_month: &Option<ReactiveEntity>, next_month: &ReactiveEntity) {
        let Some(previous_month) = previous_month else {
            return;
        };
        let instance_id = format!(
            "{}__{}",
            previous_month.as_string(MONTH_AND_YEAR).unwrap_or_default(),
            next_month.as_string(MONTH_AND_YEAR).unwrap_or_default()
        );
        let ty = RelationInstanceTypeId::new_unique_for_instance_id(RELATION_TYPE_NEXT_MONTH.clone(), instance_id);
        let reactive_relation = ReactiveRelation::builder_with_entities(previous_month.clone(), &ty, next_month.clone()).build();
        let _ = self.relation_instance_manager.register(reactive_relation);
    }

    async fn create_days(
        &self,
        current_year: &ReactiveEntity,
        current_month: &ReactiveEntity,
        previous_month_last_day: Option<ReactiveEntity>,
    ) -> Option<ReactiveEntity> {
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
            let current_day = match self.create_day(year, month_of_year, day_of_month).await {
                Some(current_day) => {
                    self.create_next_day(&previous_day, &current_day).await;
                    self.create_day_of_month(current_month, &current_day).await;
                    Some(current_day)
                }
                None => None,
            };
            if day_of_month == 1 {
                previous_day = match current_day {
                    Some(current_day) => {
                        self.create_first_day(current_month, &current_day).await;
                        // create_next_day(context, previous_month_last_day, &current_day);
                        Some(current_day)
                    }
                    None => None,
                };
            } else if day_of_month == last_day_of_month {
                previous_day = match current_day {
                    Some(current_day) => {
                        self.create_last_day(current_month, &current_day).await;
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

    async fn create_day(&self, year: i64, month: u64, day: u64) -> Option<ReactiveEntity> {
        let iso8601 = format!("{:04}-{:02}-{:02}", year, month, day);
        let id = Uuid::new_v4();
        let properties = PropertyInstances::new().property(DAY_OF_MONTH, json!(day)).property(ISO8601, json!(iso8601));
        let reactive_entity = ReactiveEntity::builder()
            .ty(ENTITY_TYPE_DAY.clone())
            .id(id)
            .properties(ReactiveProperties::new_with_id_from_properties(id, properties))
            .build();
        self.entity_instance_manager.register(reactive_entity).ok()
    }

    async fn create_day_of_month(&self, current_month: &ReactiveEntity, current_day: &ReactiveEntity) {
        let Some(month_and_year) = current_month.as_string(MONTH_AND_YEAR) else {
            return;
        };
        let Some(day_of_month) = current_day.as_u64(DAY_OF_MONTH) else {
            return;
        };
        let instance_id = format!("{:04}__{:02}", month_and_year, day_of_month);
        let ty = RelationInstanceTypeId::new_unique_for_instance_id(RELATION_TYPE_DAY_OF_MONTH.clone(), instance_id);
        let reactive_relation = ReactiveRelation::builder_with_entities(current_month.clone(), &ty, current_day.clone()).build();
        let _ = self.relation_instance_manager.register(reactive_relation);
    }

    async fn create_first_day(&self, current_month: &ReactiveEntity, first_day: &ReactiveEntity) {
        let Some(month_and_year) = current_month.as_string(MONTH_AND_YEAR) else {
            return;
        };
        let Some(day_of_month) = first_day.as_u64(DAY_OF_MONTH) else {
            return;
        };
        let instance_id = format!("{:04}__{:02}", month_and_year, day_of_month);
        let ty = RelationInstanceTypeId::new_unique_for_instance_id(RELATION_TYPE_FIRST_DAY.clone(), instance_id);
        let reactive_relation = ReactiveRelation::builder_with_entities(current_month.clone(), &ty, first_day.clone()).build();
        let _ = self.relation_instance_manager.register(reactive_relation);
    }

    async fn create_last_day(&self, current_month: &ReactiveEntity, last_day: &ReactiveEntity) {
        let Some(month_and_year) = current_month.as_string(MONTH_AND_YEAR) else {
            return;
        };
        let Some(day_of_month) = last_day.as_u64(DAY_OF_MONTH) else {
            return;
        };
        let instance_id = format!("{:04}__{:02}", month_and_year, day_of_month);
        let ty = RelationInstanceTypeId::new_unique_for_instance_id(RELATION_TYPE_LAST_DAY.clone(), instance_id);
        let reactive_relation = ReactiveRelation::builder_with_entities(current_month.clone(), &ty, last_day.clone()).build();
        let _ = self.relation_instance_manager.register(reactive_relation);
    }

    async fn create_next_day(&self, previous_day: &Option<ReactiveEntity>, next_day: &ReactiveEntity) {
        let Some(previous_day) = previous_day else {
            return;
        };
        let instance_id = format!("{}__{}", previous_day.as_string(ISO8601).unwrap_or_default(), next_day.as_string(ISO8601).unwrap_or_default());
        let ty = RelationInstanceTypeId::new_unique_for_instance_id(RELATION_TYPE_NEXT_DAY.clone(), instance_id);
        let reactive_relation = ReactiveRelation::builder_with_entities(previous_day.clone(), &ty, next_day.clone()).build();
        let _ = self.relation_instance_manager.register(reactive_relation);
    }
}

#[async_trait]
#[component_alias]
impl TimeGraph for TimeGraphImpl {
    async fn init(&self) {
        info!("Start generating time graph");
        let start = Instant::now();
        self.create_time_graph().await;
        let duration = start.elapsed();
        info!("Successfully generated time graph in {:?}", duration);
    }

    async fn shutdown(&self) {}
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
