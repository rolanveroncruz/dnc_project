import { CommonModule } from '@angular/common';
import {Component, inject, Inject, signal} from '@angular/core';
import {
    AbstractControl,
    FormControl,
    FormGroup,
    ReactiveFormsModule, ValidationErrors,
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
import {MatCheckboxModule} from '@angular/material/checkbox';
import {MasterListMemberService} from '../../../../api_services/master-list-members-service';

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
    tooth_surface_ids: number[];
    tooth_service_type_id : number | null;
}
interface MemberDetailsSnapshot {
    mobileNumber: string;
    emailAddress: string;
    birthDate: string;
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
        MatCheckboxModule,
    ],
    providers: [provideNativeDateAdapter()],
    templateUrl: './approval-dialog-component.html',
    styleUrl: './approval-dialog-component.scss',
})
export class ApprovalDialogComponent {
    readonly verificationService = inject(VerificationService);
    private readonly masterListMemberService = inject(MasterListMemberService);

    readonly loadingMemberDetails = signal(false);
    readonly memberDetailsError = signal<string | null>(null);

    private readonly memberDetailsBaseline
        = signal<MemberDetailsSnapshot |null >(null);

    readonly hasUnsavedMemberDetails = signal(false);
    readonly savingMemberDetails = signal(false);
    readonly memberDetailsSaveError = signal<string | null>(null);
    readonly memberDetailsSaveMessage = signal<string | null>(null);


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
        tooth_surface_ids: FormControl<number[]>;
        tooth_service_type_id: FormControl<number | null>;
        member_mobile_number: FormControl<string>;
        member_email_address: FormControl<string>;
        member_birth_date: FormControl<string>;
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
            tooth_surface_ids: new FormControl<number[]>(
                [],
                {
                    nonNullable: true,
                    validators: this.data.dental_service_record_surface? [this.requireAtLeastOneSurface]:[],
                }
            ),
            tooth_service_type_id: new FormControl<number | null>(
                null,
                {
                    validators: [],
                }
            ),
            member_mobile_number: new FormControl<string> ('', {
                nonNullable: true,
            }),
            member_email_address: new FormControl<string>('',{
                nonNullable:true,
            }),
            member_birth_date: new FormControl<string> ('',{
                nonNullable:true
            })
        });
        this.approvalCode = this.data.approval_code ?? null;

        // if and when mobile_number, email_address, or birthdate is edited, call updateMemberDetailsChangeState().
        this.form.controls.member_mobile_number.valueChanges.subscribe(() => {
            this.onMemberDetailsChanged();
        });

        this.form.controls.member_email_address.valueChanges.subscribe(() => {
            this.onMemberDetailsChanged();
        });

        this.form.controls.member_birth_date.valueChanges.subscribe(() => {
            this.onMemberDetailsChanged();
        });
        this.loadMemberDetails();



    }
    // ✅ validator for "at least one checkbox must be selected"

    private requireAtLeastOneSurface(control: AbstractControl): ValidationErrors | null {
        const value =control.value as number[] |null;
        return value && value.length > 0 ? null:{required: true};
    }

    cancel(): void {
        this.dialogRef.close({
            confirmed: false,
            service_availed_date: null,
            tooth_id:null,
            tooth_surface_ids: [],
            tooth_service_type_id: null,
        });
    }
    hasApprovalCode(): boolean {
        return !!this.approvalCode;
    }
    isToothSurfaceChecked(surfaceId: number): boolean {
        return this.form.controls.tooth_surface_ids.value.includes(surfaceId);
    }

    onToothSurfaceToggle(surfaceId: number, checked: boolean): void {
        const control = this.form.controls.tooth_surface_ids;
        const current = control.value;

        const next = checked
            ? [...current, surfaceId]
            : current.filter(id => id !== surfaceId);

        control.setValue(next);
        control.markAsTouched();
        control.updateValueAndValidity();
    }

    getApprovalCode(): void {
        if (this.hasUnsavedMemberDetails()){
            return;
        }
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
        const toothSurfaceIds = this.form.controls.tooth_surface_ids.value;
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
        // ✅ if surfaces are required, at least one must be checked
        if (this.data.dental_service_record_surface && toothSurfaceIds.length === 0) {
            this.isRequestingApprovalCode = false;
            this.form.controls.tooth_surface_ids.markAsTouched();
            return;
        }

        var request:GetApprovalCodeRequest = {
            date_service_performed: serviceDate,
            tooth_id: toothId,
            tooth_surface_ids: toothSurfaceIds,
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

// ✅ ADD
    private loadMemberDetails(): void {
        this.loadingMemberDetails.set(true);
        this.memberDetailsError.set(null);

        this.masterListMemberService
            .getMasterListMember(this.data.master_list_member_id)
            .subscribe({
                next: member => {

                    const memberDetails: MemberDetailsSnapshot = {
                        mobileNumber: member.mobile_number ?? '',
                        emailAddress: member.email_address ?? "",
                        birthDate: member.birth_date ?? "",
                    };

                    this.form.controls.member_mobile_number.setValue(
                        member.mobile_number ?? '',
                        { emitEvent: false }
                    );

                    this.form.controls.member_email_address.setValue(
                        member.email_address ?? '',
                        { emitEvent: false }
                    );

                    this.form.controls.member_birth_date.setValue(
                        member.birth_date ?? '',
                        { emitEvent: false }
                    );

                    this.memberDetailsBaseline.set(memberDetails);

                    this.loadingMemberDetails.set(false);
                    this.hasUnsavedMemberDetails.set(false);
                },
                error: error => {
                    console.error(
                        'Failed to load master list member details:',
                        error
                    );

                    this.memberDetailsError.set(
                        'Failed to load member contact details.'
                    );

                    this.loadingMemberDetails.set(false);
                },
            });
    }
    private updateMemberDetailsChangeState(): void {
        const baseline = this.memberDetailsBaseline();

        if (baseline === null) {
            this.hasUnsavedMemberDetails.set(false);
            return;
        }

        const hasChanges =
            this.form.controls.member_mobile_number.value !== baseline.mobileNumber ||
            this.form.controls.member_email_address.value !== baseline.emailAddress ||
            this.form.controls.member_birth_date.value !== baseline.birthDate;

        this.hasUnsavedMemberDetails.set(hasChanges);
    }


    revertMemberDetails(): void {
        const baseline = this.memberDetailsBaseline();

        if (baseline === null) {
            return;
        }

        this.form.controls.member_mobile_number.setValue(
            baseline.mobileNumber,
            { emitEvent: false }
        );

        this.form.controls.member_email_address.setValue(
            baseline.emailAddress,
            { emitEvent: false }
        );

        this.form.controls.member_birth_date.setValue(
            baseline.birthDate,
            { emitEvent: false }
        );

        this.hasUnsavedMemberDetails.set(false);

        this.memberDetailsSaveMessage.set(null);
        this.memberDetailsSaveError.set(null);
    }

    private normalizeOptional( value: string | null | undefined): string | null {
        const normalized = (value ?? '').trim();
        return normalized === '' ? null : normalized;
    }

    saveMemberDetails(): void {
        if (
            !this.hasUnsavedMemberDetails() ||
            this.savingMemberDetails()
        ) {
            return;
        }

        this.savingMemberDetails.set(true);
        this.memberDetailsSaveError.set(null);
        this.memberDetailsSaveMessage.set(null);

        const payload = {
            // ✅ Blank optional fields are sent as null
            mobile_number: this.normalizeOptional(
                this.form.controls.member_mobile_number.value
            ),
            email_address: this.normalizeOptional(
                this.form.controls.member_email_address.value
            ),
            birth_date: this.normalizeOptional(
                this.form.controls.member_birth_date.value
            ),
        };

        this.masterListMemberService
            .patchMasterListMember(
                this.data.master_list_member_id,
                payload
            )
            .subscribe({
                next: member => {
                    const savedDetails: MemberDetailsSnapshot = {
                        mobileNumber: member.mobile_number ?? '',
                        emailAddress: member.email_address ?? '',
                        birthDate: member.birth_date ?? '',
                    };

                    // ✅ Reflect the values returned by the backend
                    this.form.controls.member_mobile_number.setValue(
                        savedDetails.mobileNumber,
                        { emitEvent: false }
                    );

                    this.form.controls.member_email_address.setValue(
                        savedDetails.emailAddress,
                        { emitEvent: false }
                    );

                    this.form.controls.member_birth_date.setValue(
                        savedDetails.birthDate,
                        { emitEvent: false }
                    );

                    // ✅ The successfully saved values become the new baseline
                    this.memberDetailsBaseline.set(savedDetails);
                    this.hasUnsavedMemberDetails.set(false);

                    this.memberDetailsSaveMessage.set(
                        'Member details saved successfully.'
                    );

                    this.savingMemberDetails.set(false);
                },
                error: error => {
                    console.error(
                        'Failed to save member details:',
                        error
                    );

                    this.memberDetailsSaveError.set(
                        'Failed to save member details.'
                    );

                    this.savingMemberDetails.set(false);
                },
            });
    }

    // ✅ ADD
    private onMemberDetailsChanged(): void {
        this.updateMemberDetailsChangeState();

        // Clear messages from the previous save attempt
        this.memberDetailsSaveMessage.set(null);
        this.memberDetailsSaveError.set(null);
    }
}
