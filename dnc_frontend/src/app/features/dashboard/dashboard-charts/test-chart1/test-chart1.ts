import { Component } from '@angular/core';
import {BarChart, SimpleBarChartDataset} from "../../components/bar-chart/bar-chart";


@Component({
    selector: 'app-test-chart1',
    imports: [
        BarChart,
    ],
    templateUrl: './test-chart1.html',
    styleUrl: './test-chart1.scss',
})
export class TestChart1 {
    labels: string[] = [
        'January',
        'February',
        'March',
        'April',
        'May',
        'June',
    ];

    series: SimpleBarChartDataset[] = [
        {
            label: 'Verifications',
            values: [12, 19, 8, 15, 22, 17],
        },
    ];
}
