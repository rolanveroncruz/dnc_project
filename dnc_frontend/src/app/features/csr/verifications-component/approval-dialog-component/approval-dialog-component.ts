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
import { provideNativeDateAdapter } from '@angular/material/core';
import {
    GetApprovalCodeRequest,
    ToothServiceType,
    ToothSurface,
    VerificationService
} from '../../../../api_services/verification-service';
import { ChangeDetectorRef } from '@angular/core';

export interface ApprovalDialogData {
    verification_id: number;
    date: string | Date;
    dentist_id: number;
    dentist_name: string;
    master_list_member_id: number;
    master_list_member_name: string;
    dental_service_id: number;
    dental_service_name: string;
    dental_service_record_tooth: boolean,
    dental_service_record_surface: boolean,
    service_availed_date?: string | null;
    approval_code?: string | null;
    tooth_surfaces: ToothSurface[];
    tooth_service_types: ToothServiceType[];
    approved_amount: number |null;
    dentist_notes: string | null;
}

export interface ApprovalDialogResult {
    confirmed: boolean;
    service_availed_date: string | null;
    tooth_id: string | null;
    tooth_surface_id : number | null;
    tooth_service_type_id : number | null;
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
    ],
    providers: [provideNativeDateAdapter()],
    templateUrl: './approval-dialog-component.html',
    styleUrl: './approval-dialog-component.scss',
})
export class ApprovalDialogComponent {
    readonly verificationService = inject(VerificationService);

    approvalCode: string | null;
    rejectMessage: string | null = null;
    isRequestingApprovalCode = false;
    readonly toothIds: string[] = [
        '11', '12', '13', '14', '15', '16', '17', '18',
        '21', '22', '23', '24', '25', '26', '27', '28',
        '31', '32', '33', '34', '35', '36', '37', '38',
        '41', '42', '43', '44', '45', '46', '47', '48',
        '51', '52', '53', '54', '55',
        '61', '62', '63', '64', '65',
        '71', '72', '73', '74', '75',
        '81', '82', '83', '84', '85',

    ]

    readonly form: FormGroup<{
        service_availed_date: FormControl<Date | null>;
        tooth_id: FormControl<string | null>;
        tooth_surface_id: FormControl<number | null>;
        tooth_service_type_id: FormControl<number | null>;
    }>;

    constructor(
        private dialogRef: MatDialogRef<ApprovalDialogComponent, ApprovalDialogResult>,
        private cdr: ChangeDetectorRef,
        @Inject(MAT_DIALOG_DATA) public data: ApprovalDialogData,
    ) {
        this.form = new FormGroup({
            service_availed_date: new FormControl<Date | null>(
                this.data.service_availed_date ?
                    new Date(this.data.service_availed_date):null,
                {
                    validators: [Validators.required],
                }
            ),
            tooth_id: new FormControl<string|null>(
                null,
                {
                    validators: this.data.dental_service_record_tooth? [Validators.required]:[],
                }
            ),
            tooth_surface_id: new FormControl<number | null>(
                null,
                {
                    validators: this.data.dental_service_record_surface? [Validators.required]:[],
                }
            ),
            tooth_service_type_id: new FormControl<number | null>(
                null,
                {
                    validators: [],
                }
            ),
        });
        this.approvalCode = this.data.approval_code ?? null;
    }

    cancel(): void {
        this.dialogRef.close({
            confirmed: false,
            service_availed_date: null,
            tooth_id:null,
            tooth_surface_id: null,
            tooth_service_type_id: null,
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
        this.rejectMessage = null;
        this.approvalCode = null;
        this.isRequestingApprovalCode = true;
        const serviceDateValue = this.form.controls.service_availed_date.value;
        if (!serviceDateValue) {
            this.isRequestingApprovalCode = false;
            this.form.markAllAsTouched();
            return;
        }
        const serviceDate = this.toDateOnlyString(serviceDateValue);
        const toothId = this.form.controls.tooth_id.value;
        const toothSurfaceId = this.form.controls.tooth_surface_id.value;
        const toothServiceType = this.form.controls.tooth_service_type_id.value;
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
        if (this.data.dental_service_record_tooth && !toothSurfaceId ==null) {
            this.isRequestingApprovalCode = false;
            this.form.controls.tooth_surface_id.markAsTouched();
            return;
        }

        var request:GetApprovalCodeRequest = {
            date_service_performed: serviceDate,
            tooth_id: toothId,
            tooth_surface_id: toothSurfaceId,
            tooth_service_type_id: toothServiceType,
        }

        this.verificationService
            .requestApprovalCode(this.data.verification_id, request)
            .subscribe({
                next: (response) => {
                    if ( response.reject_code === 0){
                        this.approvalCode = response.approval_code ?? null;
                        this.rejectMessage = null;
                    } else{
                        this.approvalCode = null;
                        this.rejectMessage = response.reject_message ?? 'Approval Code Request Rejected.';
                    }
                    this.isRequestingApprovalCode = false;
                    this.cdr.detectChanges();
                },
                error: (error) => {
                    console.error('Error requesting approval code:', error);
                    this.approvalCode = null;
                    this.rejectMessage = 'Error requesting approval code. Please try again.';
                    this.isRequestingApprovalCode = false;
                    this.cdr.detectChanges();
                },
            })
    }

    private toDateOnlyString(date:Date): string{
        const year = date.getFullYear();
        const month = String(date.getMonth()+1).padStart(2, '0');
        const day = String(date.getDate()).padStart(2, '0');
        return `${year}-${month}-${day}`;
    }


}
