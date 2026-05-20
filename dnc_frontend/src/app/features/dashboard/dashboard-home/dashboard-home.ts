import { Component } from '@angular/core';
import { BaseChartDirective } from 'ng2-charts';
import {TestChart1} from '../dashboard-charts/test-chart1/test-chart1';
import {TestStackedBar} from '../dashboard-charts/test-stacked-bar/test-stacked-bar';

@Component({
    selector: 'app-dashboard-home',
    imports: [
        TestChart1,
        TestStackedBar
    ],
    templateUrl: './dashboard-home.html',
    styleUrl: './dashboard-home.scss',
})
export class DashboardHome {
}
