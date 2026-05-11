import { Component } from '@angular/core';
import { BaseChartDirective } from 'ng2-charts';
import { ChartConfiguration, ChartOptions } from 'chart.js';

@Component({
    selector: 'app-dashboard-home',
    imports: [
        BaseChartDirective, // ✅ needed for <canvas baseChart>
    ],
    templateUrl: './dashboard-home.html',
    styleUrl: './dashboard-home.scss',
})
export class DashboardHome {
    // ✅ Bar chart data
    barChartData: ChartConfiguration<'bar'>['data'] = {
        labels: ['Jan', 'Feb', 'Mar', 'Apr', 'May'],
        datasets: [
            {
                label: 'Verifications',
                data: [12, 19, 8, 15, 22],
            },
        ],
    };

    // ✅ Bar chart options
    barChartOptions: ChartOptions<'bar'> = {
        responsive: true,
        maintainAspectRatio: false,
    };

    // ✅ Chart type
    barChartType: 'bar' = 'bar';
}
