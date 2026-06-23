import {Component, computed, inject, OnInit, signal} from '@angular/core';
import {CommonModule} from '@angular/common';

import {MatCardModule} from '@angular/material/card';
import {MatTableModule} from '@angular/material/table';
import {MatCheckboxModule} from '@angular/material/checkbox';
import {MatProgressSpinnerModule} from '@angular/material/progress-spinner';
import {MatButtonModule} from '@angular/material/button';

import {
    DentistPaymentMatrixCell,
    DentistPaymentMatrixMonth,
    DentistPaymentMatrixResponse,
    DentistPaymentMatrixRow,
    DentistPaymentsService,
} from '../../../../api_services/dentist-payments-service';

@Component({
    selector: 'app-retainer-fees-paid',
    standalone: true,
    imports: [
        CommonModule,
        MatCardModule,
        MatTableModule,
        MatCheckboxModule,
        MatProgressSpinnerModule,
        MatButtonModule,
    ],
    templateUrl: './retainer-fees-paid.html',
    styleUrl: './retainer-fees-paid.scss',
})
export class RetainerFeesPaid implements OnInit {
    private readonly dentistPaymentsService = inject(DentistPaymentsService);

    readonly matrix = signal<DentistPaymentMatrixResponse | null>(null);
    readonly loading = signal(false);
    readonly error = signal<string | null>(null);

    readonly displayedColumns = computed(() => {
        const matrix = this.matrix();

        if (!matrix) {
            return [];
        }

        return [
            'dentist_name',
            'clinic_name',
            ...matrix.months.map(month => this.monthColumnKey(month)),
        ];
    });

    ngOnInit(): void {
        this.loadMatrix();
    }

    loadMatrix(): void {
        this.loading.set(true);
        this.error.set(null);

        this.dentistPaymentsService.getDentistPaymentsMatrix().subscribe({
            next: response => {
                this.matrix.set(response);
                this.loading.set(false);
            },
            error: err => {
                console.error('Failed to load dentist payments matrix', err);
                this.error.set('Failed to load retainer fee payment matrix.');
                this.loading.set(false);
            },
        });
    }

    monthColumnKey(month: DentistPaymentMatrixMonth): string {
        return `month_${month.year}_${month.month}`;
    }

    getCell(row: DentistPaymentMatrixRow, month: DentistPaymentMatrixMonth): DentistPaymentMatrixCell | null {
        return row.cells.find(cell =>
            cell.year === month.year &&
            cell.month === month.month
        ) ?? null;
    }

    isPaid(row: DentistPaymentMatrixRow, month: DentistPaymentMatrixMonth): boolean {
        return this.getCell(row, month)?.paid ?? false;
    }

}
