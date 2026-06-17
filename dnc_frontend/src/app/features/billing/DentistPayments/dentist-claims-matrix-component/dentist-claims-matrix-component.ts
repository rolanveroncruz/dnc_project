import { Component, computed, inject, signal } from '@angular/core';
import { CommonModule, formatDate } from '@angular/common';
import { FormBuilder, ReactiveFormsModule, Validators } from '@angular/forms';

import { MatButtonModule } from '@angular/material/button';
import { MatCardModule } from '@angular/material/card';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatTableModule } from '@angular/material/table';
import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';

import {
    DentistHmoAuditCell,
    DentistHmoAuditDentistRow,
    DentistHmoServicesMatrixService,
    DentistHmoServiceAuditMatrixResponse,
} from '../../../../api_services/dentist-hmo-services-matrix-service';

@Component({
    selector: 'app-dentist-claims-matrix',
    standalone: true,
    imports: [
        CommonModule,
        ReactiveFormsModule,

        MatButtonModule,
        MatCardModule,
        MatFormFieldModule,
        MatInputModule,
        MatTableModule,
        MatIconModule,
        MatProgressSpinnerModule,
    ],
    templateUrl: './dentist-claims-matrix-component.html',
    styleUrl: './dentist-claims-matrix-component.scss',
})
export class DentistClaimsMatrixComponent {
    private readonly fb = inject(FormBuilder);
    private readonly dentistHmoServicesMatrixService = inject(DentistHmoServicesMatrixService);

    readonly matrix = signal<DentistHmoServiceAuditMatrixResponse | null>(null);
    readonly loading = signal(false);
    readonly errorMessage = signal<string | null>(null);

    readonly today = formatDate(new Date(), 'yyyy-MM-dd', 'en-US');

    readonly form = this.fb.nonNullable.group({
        start_date: ['', Validators.required],
        end_date: [this.today, Validators.required],
    });

    readonly displayedColumns = computed(() => {
        const matrix = this.matrix();

        if (!matrix) {
            return [];
        }

        return [
            'dentist_name',
            ...matrix.hmos.map(hmo => this.hmoColumnKey(hmo.hmo_id)),
            'row_total',
        ];
    });

    generateMatrix(): void {
        if (this.form.invalid) {
            this.form.markAllAsTouched();
            return;
        }

        const { start_date, end_date } = this.form.getRawValue();

        if (start_date > end_date) {
            this.errorMessage.set('Start date cannot be later than end date.');
            this.matrix.set(null);
            return;
        }

        this.loading.set(true);
        this.errorMessage.set(null);

        this.dentistHmoServicesMatrixService
            .getDentistHmoServicesClaimsMatrix(start_date, end_date)
            .subscribe({
                next: response => {
                    this.matrix.set(response);
                    this.loading.set(false);
                },
                error: err => {
                    console.error('Failed to load dentist claims matrix', err);
                    this.matrix.set(null);
                    this.errorMessage.set('Failed to generate dentist claims matrix.');
                    this.loading.set(false);
                },
            });
    }

    clearMatrix(): void {
        this.matrix.set(null);
        this.errorMessage.set(null);
    }

    hmoColumnKey(hmoId: number): string {
        return `hmo_${hmoId}`;
    }

    getCell(
        row: DentistHmoAuditDentistRow,
        hmoId: number
    ): DentistHmoAuditCell | null {
        return row.cells.find(cell => cell.hmo_id === hmoId) ?? null;
    }

    getHmoTotalQty(hmoId: number): number {
        const matrix = this.matrix();

        if (!matrix) {
            return 0;
        }

        return matrix.rows.reduce((sum, row) => {
            const cell = this.getCell(row, hmoId);
            return sum + (cell?.cell_total_qty ?? 0);
        }, 0);
    }

    getHmoTotalFee(hmoId: number): number {
        const matrix = this.matrix();

        if (!matrix) {
            return 0;
        }

        return matrix.rows.reduce((sum, row) => {
            const cell = this.getCell(row, hmoId);
            return sum + (cell?.cell_total_fee ?? 0);
        }, 0);
    }

    hasStartDateError(): boolean {
        const control = this.form.controls.start_date;
        return control.invalid && (control.dirty || control.touched);
    }

    hasEndDateError(): boolean {
        const control = this.form.controls.end_date;
        return control.invalid && (control.dirty || control.touched);
    }
}
