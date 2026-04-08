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
    dental_service_record_tooth: boolean,
    service_availed_date?: string | null;
    approval_code?: string | null;
}

export interface ApprovalDialogResult {
    confirmed: boolean;
    service_availed_date: string | null;
    tooth_id: string | null;
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

    approvalCode: string | null;
    isRequestingApprovalCode = false;

    readonly form: FormGroup<{
        service_availed_date: FormControl<string | null>;
        tooth_id: FormControl<string | null>;
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
            tooth_id: new FormControl<string|null>(
                null,
                {
                    validators: this.data.dental_service_record_tooth? [Validators.required]:[],
                }
            )
        });
        this.approvalCode = this.data.approval_code ?? null;
    }

    cancel(): void {
        this.dialogRef.close({
            confirmed: false,
            service_availed_date: null,
            tooth_id:null,
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
        const toothId = this.form.controls.tooth_id.value;
        if (!serviceDate) {
            this.isRequestingApprovalCode = false;
            this.form.markAllAsTouched();
            return;
        }
        if (this.data.dental_service_record_tooth && !toothId?.trim()) {
            this.isRequestingApprovalCode = false;
            this.form.controls.tooth_id.markAsTouched();
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
