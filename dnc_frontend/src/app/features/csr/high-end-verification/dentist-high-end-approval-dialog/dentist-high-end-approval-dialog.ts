import { CommonModule } from '@angular/common';
import { Component, Inject } from '@angular/core';
import {
    MAT_DIALOG_DATA,
    MatDialogModule,
    MatDialogRef,
} from '@angular/material/dialog';
import { MatButtonModule } from '@angular/material/button';
import { FormControl, FormGroup, ReactiveFormsModule, Validators } from '@angular/forms';
import {HighEndVerificationResponse, HighEndFileResponse, HighEndVerificationsService} from '../../../../api_services/high-end-verifications-service';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatInputModule} from '@angular/material/input';
export interface DentistHighEndApprovalDialogResult{
    confirmed: boolean;
    approved_cost: string | null;
    dentist_notes: string | null;
}
@Component({
    selector: 'app-dentist-high-end-approval-dialog',
    standalone: true,
    imports: [
        CommonModule,
        MatDialogModule,
        MatButtonModule,
        ReactiveFormsModule,
        MatFormFieldModule,
        MatInputModule,
    ],
    templateUrl: './dentist-high-end-approval-dialog.html',
    styleUrl: './dentist-high-end-approval-dialog.scss',
})
export class DentistHighEndApprovalDialogComponent {

    form = new FormGroup({
        approved_cost: new FormControl<string>('', {
            nonNullable: true,
            validators: [Validators.required, Validators.pattern(/^\d{1,3}(,\d{3})*(\.\d{0,2})?$|^\d+(\.\d{0,2})?$/)],
        }),
        dentist_notes: new FormControl<string >('', {
            nonNullable: true,
        }),
    })
    constructor(
        // ✅ receive the clicked row as dialog data
        @Inject(MAT_DIALOG_DATA) public data: HighEndVerificationResponse,
        private dialogRef: MatDialogRef<DentistHighEndApprovalDialogComponent>,
        private readonly highEndVerificationsService: HighEndVerificationsService,
    ) {}

    close(): void {
        this.dialogRef.close();
    }

    downloadFile(file: HighEndFileResponse): void {
        this.highEndVerificationsService.downloadHighEndFile(file.id).subscribe({
            next: (blob)=>{
                const url = window.URL.createObjectURL(blob);
                const a = document.createElement('a');
                a.href = url;
                a.download = file.original_filename || `high_end_file_${file.id}`;
                document.body.appendChild(a);
                a.click();
                a.remove();
                window.URL.revokeObjectURL(url);
            },
            error: (error)=>{
                console.error('Error downloading file:', error);
            }
        })
    }

    private normalizeApprovedCost(raw:string):string{
        return raw.replace(/,/g,'').trim();
    }
    formatApprovedCost(){
        const rawValue = this.form.controls.approved_cost.value?? '';
        const normalized = this.normalizeApprovedCost(rawValue);

        if (normalized===''){
            this.form.controls.approved_cost.setValue('', {emitEvent: false});
            return;
        }

        if (!/^\d+(\.\d{0,2})?$/.test(normalized)) {
            return;
        }

        const numericValue = Number(normalized);

        if (Number.isNaN(numericValue)) {
            return;
        }

        const formatted = new Intl.NumberFormat('en-PH', {
            minimumFractionDigits: 2,
            maximumFractionDigits: 2,
        }).format(numericValue);

        this.form.controls.approved_cost.setValue(formatted, { emitEvent: false });
}


    submit(): void{
        if (this.form.invalid) {
            this.form.markAllAsTouched();
            return;
        }
        const amountValue = this.normalizeApprovedCost(this.form.controls.approved_cost.value ?? '');
        const notesValue = this.form.controls.dentist_notes.value.trim();

        this.dialogRef.close({
            confirmed: true,
            approved_cost: amountValue === '' ? null : amountValue,
            dentist_notes: notesValue === '' ? null : notesValue,
        } satisfies DentistHighEndApprovalDialogResult);
    }
}
