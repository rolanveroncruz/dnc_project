use sea_orm_migration::{prelude::* };

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::create_dentist_services_month_year_view(self, manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::drop_dentist_services_month_year_view(self, manager).await?;
        Ok(())
    }
}
impl Migration{
    async fn create_dentist_services_month_year_view(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr>{
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE VIEW dentist_approved_by_month AS
                    WITH months AS (
                        SELECT generate_series(
                        date_trunc('month', CURRENT_DATE) - interval '12 months',
                        date_trunc('month', CURRENT_DATE),
                        interval '1 month'
                        )::date AS month_start
                    ),
                grouped AS (
                SELECT
                    ua.dentist_id,
                    date_trunc('month', ua.date_service_performed)::date AS month_start,
                    array_agg(
                    ua.source || ':' || ua.id::text
                    ORDER BY ua.date_service_performed, ua.source, ua.id
                ) AS source_ids
                FROM unified_approved ua
                WHERE ua.date_service_performed IS NOT NULL
                GROUP BY
                ua.dentist_id,
                    date_trunc('month', ua.date_service_performed)::date
                )
                SELECT
                    d.id AS dentist_id,
                    to_char(m.month_start, 'YYYY-MM') AS month_year,
                    COALESCE(g.source_ids, ARRAY[]::text[]) AS source_ids
                FROM dentist d
                    CROSS JOIN months m
                    LEFT JOIN grouped g
                    ON g.dentist_id = d.id
                    AND g.month_start = m.month_start
                ORDER BY
                    d.id,
                    m.month_start;
                "#
            ).await?;
        Ok(())
    }
    async fn drop_dentist_services_month_year_view(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr>{
        manager
            .get_connection()
            .execute_unprepared(
                r#"DROP VIEW dentist_approved_services_by_month_year"#
            ).await?;
        Ok(())
    }
}
