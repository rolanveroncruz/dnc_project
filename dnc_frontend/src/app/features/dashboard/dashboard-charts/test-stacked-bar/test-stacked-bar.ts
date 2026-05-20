import { Component } from '@angular/core';
import {
    StackedBarChart,
    SimpleStackedBarChartDataset
} from "../../components/stacked-bar-chart/stacked-bar-chart";

@Component({
    selector: 'app-test-stacked-bar',
    imports: [
        StackedBarChart,
    ],
    templateUrl: './test-stacked-bar.html',
    styleUrl: './test-stacked-bar.scss',
})
export class TestStackedBar {
    labels: string[] = [
        'January',
        'February',
        'March',
        'April',
        'May',
        'June',
    ];

    series: SimpleStackedBarChartDataset[] = [
        {
            label: 'Approved',
            values: [40, 55, 48, 62, 70, 66],
        },
        {
            label: 'Pending',
            values: [10, 14, 9, 12, 8, 11],
        },
        {
            label: 'Rejected',
            values: [3, 5, 2, 4, 6, 3],
        },
    ];
}
