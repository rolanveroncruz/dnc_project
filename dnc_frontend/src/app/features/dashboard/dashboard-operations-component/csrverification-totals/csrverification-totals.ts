import {Component, effect, inject, input,  signal} from '@angular/core';
import {BarChart, SimpleBarChartDataset} from "../../components/bar-chart/bar-chart";
import {DashboardCSRService, CsrVerificationActivityRow} from '../../../../api_services/dashboard-csrservice';
import {MatLabel} from '@angular/material/input';

export type CSRVerificationMetric = 'created' | 'approved' | 'reconciled';

@Component({
  selector: 'app-csrverification-totals',
    imports: [
        BarChart,

    ],
  templateUrl: './csrverification-totals.html',
  styleUrl: './csrverification-totals.scss',
})
export class CSRVerificationTotals {
    dashboardService = inject(DashboardCSRService)

    labels = signal<string[]>([]);
    series = signal<SimpleBarChartDataset[]>([]);

    title = input.required<string>();
    subtitle = input.required<string>();
    metric = input.required<CSRVerificationMetric>();
    date_start = input.required<string>();
    date_end = input.required<string>();
    refreshTick = input<number>(0);

    constructor() {
        effect(()=>{
            const start = this.date_start();
            const end = this.date_end();

            this.refreshTick();
            if (!start || !end) return;
            this.load_csr_verification_totals();
        });
    }

    load_csr_verification_totals() {
        this.dashboardService
            .getCSRVerificationTotals(this.date_start(), this.date_end())
            .subscribe({
                next: (rows: CsrVerificationActivityRow[]) => {
                    this.labels.set(
                        rows.map(row => row.email)
                    );
                    this.series.set([
                        {
                            label: this.get_series_label(),
                            values: rows.map(row =>this.get_metric_value( row)),
                        },
                    ]);
                    console.log("this.series:", this.series())
                },
                error: (err) => {
                    console.log("In load_csr_verification_totals(), failed to load csr verification totals", err);
                    this.labels.set([]);
                    this.series.set([]);
                }
            });
    }
    private get_series_label(): string{
        switch (this.metric()){
            case 'created':
                return 'Created';
            case 'approved':
                return 'Approved';
            case 'reconciled':
                return 'Reconciled';
        }
    }

    get_metric_value( row:CsrVerificationActivityRow):number{
            switch (this.metric()){
                case 'created':
                    return row.created_count;
               case 'approved':
                    return row.approved_count;
               case 'reconciled':
                    return row.reconciled_count;
            }
    }

}

