import {Component, DestroyRef, inject, OnInit, signal} from '@angular/core';
import {CSRVerificationTotals} from './csrverification-totals/csrverification-totals';
import {CSRVerificationDailyTotals} from './csrverification-daily-totals/csrverification-daily-totals';
import {interval} from 'rxjs';
import {takeUntilDestroyed} from '@angular/core/rxjs-interop';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatSelectModule} from '@angular/material/select';
import {MatOptionModule} from '@angular/material/core';

@Component({
  selector: 'app-dashboard-operations-component',
    imports: [
        CSRVerificationTotals,
        MatFormFieldModule,
        MatSelectModule,
        MatOptionModule,
        CSRVerificationDailyTotals,
    ],
  templateUrl: './dashboard-operations-component.html',
  styleUrl: './dashboard-operations-component.scss',
})
export class DashboardOperationsComponent implements OnInit{
    private destroyRef = inject(DestroyRef);

    selected_period = signal<'today'|'this_week'|'this_month'>('today');
    date_start = signal<string>('');
    date_end = signal<string>('');
    refreshTick = signal<number>(0);
    refreshInterval = 30;

    ngOnInit() {
        this.set_date_range('today');
        interval(this.refreshInterval * 1000)
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe(() => {
                this.refreshTick.update(tick => tick + 1);
            });
    }

    set_date_range(period: 'today'|'this_week'|'this_month') {
        this.selected_period.set(period);
        const today = new Date();
        let start = new Date(today);
        let end = new Date(today);

        if (period === 'this_week') {
            const day = today.getDay();
            const diffToMonday = day === 0 ? -6 : 1 - day;

            start = new Date(today);
            start.setDate(today.getDate() + diffToMonday);

            end = new Date(start);
            end.setDate(start.getDate() + 6);
        }

        if (period === 'this_month') {
            start = new Date(today.getFullYear(), today.getMonth(), 1);
            end = new Date(today.getFullYear(), today.getMonth() + 1, 0);
        }

        this.date_start.set(this.to_date_string(start));
        this.date_end.set(this.to_date_string(end));
        this.refreshTick.update(tick => tick + 1);
    }

    private to_date_string(date:Date): string{
        const year = date.getFullYear();
        const month = String(date.getMonth() + 1).padStart(2, '0');
        const day = String(date.getDate()).padStart(2, '0');
        return `${year}-${month}-${day}`;
    }
}

