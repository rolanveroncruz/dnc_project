import {Component, DestroyRef, inject, Input, OnChanges, SimpleChanges} from '@angular/core';
import {CsrVerificationActivityUnitRow, DashboardCSRService} from '../../../../api_services/dashboard-csrservice';
import {ChartConfiguration } from 'chart.js';
import {takeUntilDestroyed} from '@angular/core/rxjs-interop';
import {BaseChartDirective} from 'ng2-charts';



type CsrDailyMetric = 'created' | 'approved' | 'reconciled';

@Component({
  selector: 'app-csrverification-daily-totals',
    imports: [
        BaseChartDirective
    ],
  templateUrl: './csrverification-daily-totals.html',
  styleUrl: './csrverification-daily-totals.scss',
})
export class CSRVerificationDailyTotals implements OnChanges{
    private dashboardService = inject(DashboardCSRService);
    private destroyRef = inject(DestroyRef);

    @Input({required: true})title = '';
    @Input() subtitle = '';
    @Input({required: true}) metric: CsrDailyMetric = 'created';

    @Input({required: true}) date_start = '';
    @Input({required: true}) date_end = '';
    @Input() refreshTick = 0;

    loading = false;
    errorMsg :string | null = null;
    barChartType: 'bar' = 'bar';

    barChartData: ChartConfiguration<'bar'>['data'] = {
        labels: [],
        datasets: [],
    };

    barChartOptions: ChartConfiguration<'bar'>['options'] = {
        responsive: true,
        maintainAspectRatio: false,
        plugins: {
            legend: {
                display: true,
                position: 'top',
            },
        },
        scales: {
            x: {
                stacked: false,
            },
            y: {
                stacked: false,
                beginAtZero: true,
                ticks: {
                    precision: 0,
                },
            },
        },
    };

    ngOnChanges( changes: SimpleChanges):void  {
        if (!this.date_start || !this.date_end) return;

        if ( changes['date_start'] || changes['date_end'] || changes['refreshTick'] || changes['metric'] ){
            setTimeout(() => {
                this.load();

            })
        }
    }
    private load(){
        this.loading = true;
        this.errorMsg = null;

        this.dashboardService
            .getCSRVerificationUnitTotals(this.date_start, this.date_end)
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe({
                next: (rows) => {
                    this.setChartData(rows);
                    this.loading = false;
                },
                error: (err) => {
                    console.error('Failed to load CSR daily totals', err);
                    this.errorMsg = 'Failed to load CSR daily totals.';
                    this.loading = false;
                },
            });
    }
    private setChartData(rows: CsrVerificationActivityUnitRow[]) {
        const days = this.getDateRange(this.date_start, this.date_end);

        const users = Array.from(
            new Map(
                rows.map(row => [
                    row.user_id,
                    {
                        user_id: row.user_id,
                        name: row.name,
                    },
                ])
            ).values()
        );
        const countByUserAndDay = new Map<string, number>();

        for (const row of rows) {
            const day = row.period_start.slice(0, 10);
            const key = `${row.user_id}|${day}`;

            countByUserAndDay.set(key, this.getMetricValue(row));
        }

        this.barChartData = {
            labels: days,
            datasets: users.map(user => ({
                label: user.name,
                data: days.map(day => {
                    const key = `${user.user_id}|${day}`;
                    return countByUserAndDay.get(key) ?? 0;
                }),
            })),
        };
    }

    private getMetricValue(row: CsrVerificationActivityUnitRow): number {
        switch (this.metric) {
            case 'created':
                return row.created_count;
            case 'approved':
                return row.approved_count;
            case 'reconciled':
                return row.reconciled_count;
        }
    }

    private getDateRange(start: string, end: string): string[] {
        const dates: string[] = [];

        const current = new Date(`${start}T00:00:00`);
        const last = new Date(`${end}T00:00:00`);

        while (current <= last) {
            dates.push(this.toDateString(current));
            current.setDate(current.getDate() + 1);
        }

        return dates;
    }
    private toDateString(date: Date): string {
        const year = date.getFullYear();
        const month = String(date.getMonth() + 1).padStart(2, '0');
        const day = String(date.getDate()).padStart(2, '0');

        return `${year}-${month}-${day}`;
    }
}
