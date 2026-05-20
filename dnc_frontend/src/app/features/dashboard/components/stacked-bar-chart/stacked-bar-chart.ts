import { Component, Input } from '@angular/core';
import { BaseChartDirective } from 'ng2-charts';
import {
    ChartConfiguration,
    ChartDataset,
    ChartOptions,
} from 'chart.js';

export interface SimpleStackedBarChartDataset {
    label: string;
    values: number[];
}

@Component({
    selector: 'app-stacked-bar-chart',
    imports: [
        BaseChartDirective,
    ],
    templateUrl: './stacked-bar-chart.html',
    styleUrl: './stacked-bar-chart.scss',
})
export class StackedBarChart {
    @Input() labels: string[] = [];

    @Input() series: SimpleStackedBarChartDataset[] = [];

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
                stacked: true,
                ticks: {
                    autoSkip: false,
                },
            },
            y: {
                stacked: true,
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
