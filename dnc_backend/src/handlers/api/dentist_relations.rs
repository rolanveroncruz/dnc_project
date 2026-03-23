// This file contains functions needed to implement filtering the list of endorsements a dentist can work with.
// First, collect all exclusive-to company/HMOs into "allow" lists and "not-exclusives" into "deny-lists"
// then filter endorsements by both sets
// Steps
// 1. Gather all exclusive companies into one union set
// 2. Gather all exclusive HMOs into one union set
// 3. If either set is non-empty, only endorsements matching these are allowed;otherwise all are allowed.
// 4. Gather all forbidden companies.
// 5. Gather all forbidden HMOs
// 6. Remove endorsements that match either of the above
// CASES
// 1. -, -
// everything is accepted.
// 2. -, except HMO
// everything is accepted, remove endorsements with hmo_ids in the except list.
// 3. -, except companies
// everything is accepted, remove endorsements with company_ids in the except list.

// 2. Exclusive to HMOs, -
// put only the endorsements with exclusive to hmo_ids in the set. remove nothing.
// 3. Exclusive to HMOs, except HMO
// put only the endorsements with exclusive to hmo_ids in the set. remove hmo_ids in the except set.
// 4. Exclusive to HMOs, except company
// put only the endorsements with exclusive to hmo_ids in the set. remove company_ids in the except set.

// 6. Exclusive to company, -
// put only the endorsements with company_ids in the list, remove nothing.
// 7. Exclusive to company, except HMO
// put only the endorsements with company_ids in the list, remove endorsements with hmo_ids.
// 8. Exclusive to company, except company
// put only the endorsements with company_ids in the list, remove endorsements with company_ids.
use std::collections::HashSet;

use sea_orm::{
    ColumnTrait, Condition, ConnectionTrait, EntityTrait, QueryFilter, QuerySelect
};

use crate::entities::{
    dentist_company_relations,
    dentist_hmo_relations,
    endorsement,
};

pub async fn get_endorsements_for_dentist_id<C>(
    db: &C,
    dentist_id: i32,
) -> Result<Vec<i32>, sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    // Load all company relations for this dentist
    let company_relations = dentist_company_relations::Entity::find()
        .filter(dentist_company_relations::Column::DentistId.eq(dentist_id))
        .all(db)
        .await?;

    // Load all HMO relations for this dentist
    let hmo_relations = dentist_hmo_relations::Entity::find()
        .filter(dentist_hmo_relations::Column::DentistId.eq(dentist_id))
        .all(db)
        .await?;

    // Union of exclusive company IDs
    let exclusive_company_ids: HashSet<i32> = company_relations
        .iter()
        .filter(|r| r.is_exclusive_to_company == Some(true))
        .map(|r| r.company_id)
        .collect();

    // Union of "except-for" company IDs
    let forbidden_company_ids: HashSet<i32> = company_relations
        .iter()
        .filter(|r| r.is_exclusive_to_company != Some(true))
        .map(|r| r.company_id)
        .collect();

    // Union of exclusive HMO IDs
    let exclusive_hmo_ids: HashSet<i32> = hmo_relations
        .iter()
        .filter(|r| r.is_exclusive_to_hmo == Some(true))
        .map(|r| r.hmo_id)
        .collect();

    // Union of "except-for" HMO IDs
    let forbidden_hmo_ids: HashSet<i32> = hmo_relations
        .iter()
        .filter(|r| r.is_exclusive_to_hmo != Some(true))
        .map(|r| r.hmo_id)
        .collect();

    let exclusive_company_ids: Vec<i32> = exclusive_company_ids.into_iter().collect();
    let forbidden_company_ids: Vec<i32> = forbidden_company_ids.into_iter().collect();
    let exclusive_hmo_ids: Vec<i32> = exclusive_hmo_ids.into_iter().collect();
    let forbidden_hmo_ids: Vec<i32> = forbidden_hmo_ids.into_iter().collect();

    let has_any_exclusive =
        !exclusive_company_ids.is_empty() || !exclusive_hmo_ids.is_empty();

    let mut query = endorsement::Entity::find().select_only().column(endorsement::Column::Id);

    // Apply EXCLUSIVE union rule first:
    // If there are any exclusives at all, an endorsement is allowed if:
    // - its company is in exclusive companies
    //   OR
    // - its HMO is in exclusive HMOs
    if has_any_exclusive {
        let mut exclusive_cond = Condition::any();

        if !exclusive_company_ids.is_empty() {
            exclusive_cond = exclusive_cond.add(
                endorsement::Column::EndorsementCompanyId.is_in(exclusive_company_ids.clone()),
            );
        }

        if !exclusive_hmo_ids.is_empty() {
            exclusive_cond = exclusive_cond.add(
                endorsement::Column::HmoId.is_in(exclusive_hmo_ids.clone()),
            );
        }

        query = query.filter(exclusive_cond);
    }

    // Apply EXCEPT-FOR union rule next:
    // Exclude endorsements whose company is forbidden
    // OR whose HMO is forbidden.
    if !forbidden_company_ids.is_empty() || !forbidden_hmo_ids.is_empty() {
        let mut forbidden_cond = Condition::any();

        if !forbidden_company_ids.is_empty() {
            forbidden_cond = forbidden_cond.add(
                endorsement::Column::EndorsementCompanyId.is_in(forbidden_company_ids),
            );
        }

        if !forbidden_hmo_ids.is_empty() {
            forbidden_cond = forbidden_cond.add(
                endorsement::Column::HmoId.is_in(forbidden_hmo_ids),
            );
        }

        query = query.filter(forbidden_cond.not());
    }

    let endorsement_ids: Vec<i32> = query.into_tuple().all(db).await?;

    Ok(endorsement_ids)
}