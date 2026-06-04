use sea_orm_migration::{prelude::* };

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
            r#"
                CREATE OR REPLACE VIEW dentist_clinics_reconciled_jobs_count_last_12_months AS
                ---- First create a CTE for the last 12 months
                with months AS (
                    select date_trunc('month', CURRENT_DATE)::date
                    - (interval '1 month' * gs.n) as month_start
                    from generate_series(1, 12) as gs(n)
                ),
                ---- Then create a CTE for the dentists who are principals (dc.position_id=position.id=1) of their clinics
                ---- and whose contracts are "FLAT FEE" (dentist_contract.id=1)
                dentist_clinics_flat_fee AS (
                    SELECT
                        dc.id as id,
                        d.id as dentist_id,
                        concat_ws(' ',
                            d.last_name || ',',
                            d.given_name,
                            nullif(d.middle_name, '')
                        ) as dentist_name,
                        cl.id as clinic_id,
                        cl.name as clinic_name,
                        p.name as position_name,
                        contract.name as contract_name,
                        d.accre_dentist_contract_id

                        -- dental_clinic inner join dentist inner join dental_clinic
                        FROM dentist_clinic dc
                        INNER JOIN dentist d on dc.dentist_id = d.id
                        INNER JOIN dental_clinic cl on dc.clinic_id=cl.id

                        -- now we need contract and position
                        INNER JOIN dentist_contract contract on d.accre_dentist_contract_id = contract.id
                        INNER JOIN position p on dc.position_id = p.id
                    WHERE d.accre_dentist_contract_id = 1
                    AND p.id = 1
                )
                --- Finally the main Select
                SELECT
                    dc.id,
                    dc.dentist_name,
                    dc.clinic_name,
                    dc.position_name,
                    dc.contract_name,
                    m.month_start,
                    to_char(m.month_start, 'YYYY-MM') AS month_label,
                    COUNT(v.id) AS rec_services_count
                FROM dentist_clinics_flat_fee dc
                CROSS JOIN months m
                LEFT JOIN verification v
                    ON v.dentist_id = dc.dentist_id
                    AND v.dental_clinic_id = dc.clinic_id
                    AND v.is_reconciled = true
                    AND v.date_service_performed >= m.month_start
                    AND v.date_service_performed <  m.month_start + interval '1 month'
                GROUP BY
                    dc.id,
                    dc.dentist_name,
                    dc.clinic_name,
                    dc.position_name,
                    dc.contract_name,
                    m.month_start
                ORDER BY
                    dc.dentist_name,
                    dc.clinic_name,
                    m.month_start;
            "#).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                r#"DROP VIEW dentist_clinics_reconciled_jobs_count_last_12_months"#
            ).await?;
        Ok(())
    }
}
