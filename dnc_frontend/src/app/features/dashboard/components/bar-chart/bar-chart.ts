import { Component, Input } from '@angular/core';
import { BaseChartDirective } from 'ng2-charts';
import {
    ChartConfiguration,
    ChartDataset,
    ChartOptions,
} from 'chart.js';

export interface SimpleBarChartDataset {
    label: string;
    values: number[];
}

@Component({
    selector: 'app-bar-chart',
    imports: [
        BaseChartDirective,
    ],
    templateUrl: './bar-chart.html',
    styleUrl: './bar-chart.scss',
})
export class BarChart {
    @Input() labels: string[] = [];

    @Input() series: SimpleBarChartDataset[] = [];

    @Input() options: ChartOptions<'bar'> = {
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
                ticks: {
                    autoSkip: false,
                },
            },
            y: {
                beginAtZero: true,
            },
        },
    };

    readonly type: 'bar' = 'bar';

    get datasets(): ChartDataset<'bar'>[] {
        return this.series.map((item) => ({
            label: item.label,
            data: item.values,
        }));
    }

    get data(): ChartConfiguration<'bar'>['data'] {
        return {
            labels: this.labels,
            datasets: this.datasets,
        };
    }
}
