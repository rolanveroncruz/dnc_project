import { CommonModule } from '@angular/common';
import {Component, inject, Inject} from '@angular/core';
import {
    FormControl,
    FormGroup,
    ReactiveFormsModule,
    Validators,
} from '@angular/forms';
import {
    MAT_DIALOG_DATA,
    MatDialogModule,
    MatDialogRef,
} from '@angular/material/dialog';
import { MatButtonModule } from '@angular/material/button';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatDatepickerModule } from '@angular/material/datepicker';
import { MatNativeDateModule } from '@angular/material/core';
import { VerificationService } from '../../../../api_services/verification-service';
import { ChangeDetectorRef } from '@angular/core';

export interface ApprovalDialogData {
    validation_id: number;
    date: string | Date;
    dentist_id: number;
    dentist_name: string;
    master_list_member_id: number;
    master_list_member_name: string;
    dental_service_id: number;
    dental_service_name: string;
    service_availed_date?: string | null;
    approval_code?: string | null;
}

export interface ApprovalDialogResult {
    confirmed: boolean;
    service_availed_date: Date | null;
}

@Component({
    selector: 'app-approval-dialog-component',
    standalone: true,
    imports: [
        CommonModule,
        ReactiveFormsModule,
        MatDialogModule,
        MatButtonModule,
        MatFormFieldModule,
        MatInputModule,
        MatDatepickerModule,
        MatNativeDateModule,
    ],
    templateUrl: './approval-dialog-component.html',
    styleUrl: './approval-dialog-component.scss',
})
export class ApprovalDialogComponent {
    readonly verificationService = inject(VerificationService);

    approvalCode: string |null;
    isRequestingApprovalCode = false;

    readonly form: FormGroup<{
        service_availed_date: FormControl<string| null>;
    }>;

    constructor(
        private dialogRef: MatDialogRef<ApprovalDialogComponent, ApprovalDialogResult>,
        private cdr: ChangeDetectorRef,
        @Inject(MAT_DIALOG_DATA) public data: ApprovalDialogData,
    ) {
        this.form = new FormGroup({
            service_availed_date: new FormControl<string | null>(
                this.data.service_availed_date ?? null,
                {
                    validators: [Validators.required],
                }
            ),
        });
        this.approvalCode = this.data.approval_code ?? null;
    }

    cancel(): void {
        this.dialogRef.close({
            confirmed: false,
            service_availed_date: null,
        });
    }
    hasApprovalCode(): boolean {
        return !!this.approvalCode;
    }

    getApprovalCode(): void {
        if (this.form.invalid) {
            this.form.markAllAsTouched();
            return;
        }
        this.isRequestingApprovalCode = true;
        const serviceDate = this.form.controls.service_availed_date.value;
        if (!serviceDate) {
            this.form.markAllAsTouched();
            return;
        }


        this.verificationService
            .requestApprovalCode(this.data.validation_id, serviceDate as string)
            .subscribe({
                next: (response) => {
                    this.approvalCode = response.approval_code ?? null;
                    this.isRequestingApprovalCode = false;
                    this.cdr.detectChanges();
                },
                error: (error) => {
                    console.error('Error requesting approval code:', error);
                    this.isRequestingApprovalCode = false;
                    this.cdr.detectChanges();
                },
            })
    }
}
