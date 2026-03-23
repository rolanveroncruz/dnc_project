import {
    Component,
    DestroyRef,
    EventEmitter,
    Input,
    OnChanges,
    OnInit,
    Output,
    SimpleChanges,
    computed,
    inject,
    signal,
} from '@angular/core';
import { CommonModule } from '@angular/common';
import {
    FormBuilder,
    ReactiveFormsModule,
    Validators,
} from '@angular/forms';
import { takeUntilDestroyed } from '@angular/core/rxjs-interop';

import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatButtonModule } from '@angular/material/button';
import { MatDatepickerModule } from '@angular/material/datepicker';
import { MatNativeDateModule } from '@angular/material/core';
import { MatCardModule } from '@angular/material/card';
import { MasterListMemberService, MasterListMemberLookupResponse,
    PatchMasterListMemberRequest, MasterListMemberMutationResponse,
    CreateMasterListMemberRequest} from '../../../../api_services/master-list-members-service';

export interface AgreementCorpLookup {
    agreement_corp_number: string;
    hmo_name: string;
    company_name: string;
    master_list_id?: number | null;
    endorsement_id?: number | null;
}

@Component({
    selector: 'app-master-list-member-component',
    standalone: true,
    imports: [
        CommonModule,
        ReactiveFormsModule,
        MatFormFieldModule,
        MatInputModule,
        MatButtonModule,
        MatDatepickerModule,
        MatNativeDateModule,
        MatCardModule,
    ],
    templateUrl: './master-list-member-component.html',
    styleUrl: './master-list-member-component.scss',
})
export class MasterListMemberComponent implements OnInit, OnChanges {
    private readonly fb = inject(FormBuilder);
    private readonly destroyRef = inject(DestroyRef);
    private readonly masterListMemberService = inject(MasterListMemberService);

    @Input({ required: true })
    masterListMembers: MasterListMemberLookupResponse[] = [];

    @Input()
    masterListMemberId: number | null = null;

    /**
     * Needed because MasterListMemberLookupResponse
     * does not contain hmo_name or company_name.
     */
    @Input()
    agreementLookups: AgreementCorpLookup[] = [];

    @Output()
    saved = new EventEmitter<MasterListMemberMutationResponse>();

    @Output()
    cleared = new EventEmitter<void>();

    @Output()
    memberResolved = new EventEmitter<MasterListMemberLookupResponse | null>();

    readonly loading = signal(false);
    readonly saveError = signal<string | null>(null);
    readonly infoMessage = signal<string | null>(null);

    private currentResolvedMemberId: number | null = null;
    private allowCreateForUnknownMemberNo = false;

    readonly form = this.fb.nonNullable.group({
        agreement_corp_number: [''],
        member_account_no: ['', Validators.required],
        hmo_name: [{ value: '', disabled: true }],
        company_name: [{ value: '', disabled: true }],
        last_name: ['', Validators.required],
        first_name: ['', Validators.required],
        middle_name: [''],
        email_address: [''],
        mobile_number: [''],
        birth_date: [''],
    });

    readonly mode = computed(() =>
        this.currentResolvedMemberId != null ? 'edit' : 'create'
    );

    ngOnInit(): void {
        this.setupAgreementLookupWatcher();
        this.setupMemberAccountLookupWatcher();
        this.loadFromInputId();
    }

    ngOnChanges(changes: SimpleChanges): void {
        if (
            changes['masterListMemberId'] ||
            changes['masterListMembers']
        ) {
            this.loadFromInputId();
        }
    }

    private loadFromInputId(): void {
        if (this.masterListMemberId == null) {
            this.currentResolvedMemberId = null;
            return;
        }

        const found = this.masterListMembers.find(
            (x) => x.master_list_member_id === this.masterListMemberId
        );

        if (!found) {
            this.saveError.set(
                `Master list member with ID ${this.masterListMemberId} was not found in the provided list.`
            );
            return;
        }

        this.fillFromExistingMember(found);
    }

    private setupAgreementLookupWatcher(): void {
        this.form.controls.agreement_corp_number.valueChanges
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe((value) => {
                const key = (value ?? '').trim();

                if (!key) {
                    this.form.patchValue(
                        {
                            hmo_name: '',
                            company_name: '',
                        },
                        { emitEvent: false }
                    );
                    return;
                }

                const agreement = this.agreementLookups.find(
                    (x) => x.agreement_corp_number.trim().toLowerCase() === key.toLowerCase()
                );

                this.form.patchValue(
                    {
                        hmo_name: agreement?.hmo_name ?? '',
                        company_name: agreement?.company_name ?? '',
                    },
                    { emitEvent: false }
                );
            });
    }

    private setupMemberAccountLookupWatcher(): void {
        this.form.controls.member_account_no.valueChanges
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe((value) => {
                const accountNo = (value ?? '').trim();

                if (!accountNo) {
                    return;
                }

                const found = this.masterListMembers.find(
                    (x) =>
                        x.master_list_member_account_no.trim().toLowerCase() ===
                        accountNo.toLowerCase()
                );

                if (found) {
                    this.allowCreateForUnknownMemberNo = false;
                    this.fillFromExistingMember(found);
                    this.infoMessage.set('Existing member found. Fields were filled automatically.');
                    return;
                }

                if (this.currentResolvedMemberId != null) {
                    return;
                }

                const shouldCreate = window.confirm(
                    `Member account no "${accountNo}" was not found. Would you like to create a new member?`
                );

                this.allowCreateForUnknownMemberNo = shouldCreate;

                if (shouldCreate) {
                    this.currentResolvedMemberId = null;
                    this.memberResolved.emit(null);
                    this.infoMessage.set('Creating a new member.');
                    this.form.patchValue(
                        {
                            last_name: '',
                            first_name: '',
                            middle_name: '',
                            email_address: '',
                            mobile_number: '',
                            birth_date: '',
                        },
                        { emitEvent: false }
                    );
                } else {
                    this.infoMessage.set('Please enter an existing member account number.');
                }
            });
    }

    private fillFromExistingMember(member: MasterListMemberLookupResponse): void {
        this.currentResolvedMemberId = member.master_list_member_id;
        this.allowCreateForUnknownMemberNo = false;
        this.saveError.set(null);

        this.form.patchValue(
            {
                agreement_corp_number: member.endorsement_agreement_corp_number ?? '',
                member_account_no: member.master_list_member_account_no ?? '',
                last_name: member.master_list_member_last_name ?? '',
                first_name: member.master_list_member_first_name ?? '',
                middle_name: member.master_list_member_middle_name ?? '',
                email_address: member.master_list_member_email_address ?? '',
                mobile_number: member.master_list_member_mobile_number ?? '',
                birth_date: member.master_list_member_birth_date ?? '',
            },
            { emitEvent: false }
        );

        const agreement = this.findAgreement(member.endorsement_agreement_corp_number ?? '');
        this.form.patchValue(
            {
                hmo_name: agreement?.hmo_name ?? '',
                company_name: agreement?.company_name ?? '',
            },
            { emitEvent: false }
        );

        this.memberResolved.emit(member);
    }

    private findAgreement(agreementCorpNumber: string): AgreementCorpLookup | undefined {
        const key = agreementCorpNumber.trim().toLowerCase();
        return this.agreementLookups.find(
            (x) => x.agreement_corp_number.trim().toLowerCase() === key
        );
    }

    save(): void {
        this.saveError.set(null);
        this.infoMessage.set(null);

        if (this.form.invalid) {
            this.form.markAllAsTouched();
            return;
        }

        const raw = this.form.getRawValue();
        const agreement = this.findAgreement(raw.agreement_corp_number);

        const normalizedBirthDate = this.normalizeBirthDate(raw.birth_date);
        const normalizedEmail = this.normalizeNullableString(raw.email_address);
        const normalizedMobile = this.normalizeNullableString(raw.mobile_number);

        this.loading.set(true);

        if (this.currentResolvedMemberId != null) {
            const patchPayload: PatchMasterListMemberRequest = {
                master_list_id: agreement?.master_list_id ?? null,
                account_number: raw.member_account_no.trim(),
                last_name: raw.last_name.trim(),
                first_name: raw.first_name.trim(),
                middle_name: raw.middle_name.trim(),
                email_address: normalizedEmail,
                mobile_number: normalizedMobile,
                birth_date: normalizedBirthDate,
                is_active: true,
            };

            this.masterListMemberService
                .patchMasterListMember(this.currentResolvedMemberId, patchPayload)
                .pipe(takeUntilDestroyed(this.destroyRef))
                .subscribe({
                    next: (response) => {
                        this.loading.set(false);
                        this.infoMessage.set('Member updated successfully.');
                        this.saved.emit(response);
                    },
                    error: () => {
                        this.loading.set(false);
                        this.saveError.set('Failed to update member.');
                    },
                });

            return;
        }

        if (!this.allowCreateForUnknownMemberNo && !this.masterListMemberId) {
            const existing = this.masterListMembers.find(
                (x) =>
                    x.master_list_member_account_no.trim().toLowerCase() ===
                    raw.member_account_no.trim().toLowerCase()
            );

            if (!existing) {
                this.loading.set(false);
                this.saveError.set(
                    'This member account number does not exist yet. Enter it again and confirm creation when prompted.'
                );
                return;
            }
        }

        const createPayload: CreateMasterListMemberRequest = {
            master_list_id: agreement?.master_list_id ?? null,
            account_number: raw.member_account_no.trim(),
            last_name: raw.last_name.trim(),
            first_name: raw.first_name.trim(),
            middle_name: raw.middle_name.trim(),
            email_address: normalizedEmail,
            mobile_number: normalizedMobile,
            birth_date: normalizedBirthDate,
            is_active: true,
        };

        this.masterListMemberService
            .createMasterListMember(createPayload)
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe({
                next: (response) => {
                    this.loading.set(false);
                    this.currentResolvedMemberId = response.id;
                    this.allowCreateForUnknownMemberNo = false;
                    this.infoMessage.set('Member created successfully.');
                    this.saved.emit(response);
                },
                error: () => {
                    this.loading.set(false);
                    this.saveError.set('Failed to create member.');
                },
            });
    }

    clear(): void {
        this.currentResolvedMemberId = null;
        this.allowCreateForUnknownMemberNo = false;
        this.saveError.set(null);
        this.infoMessage.set(null);

        this.form.reset({
            agreement_corp_number: '',
            member_account_no: '',
            hmo_name: '',
            company_name: '',
            last_name: '',
            first_name: '',
            middle_name: '',
            email_address: '',
            mobile_number: '',
            birth_date: '',
        });

        this.cleared.emit();
        this.memberResolved.emit(null);
    }

    private normalizeNullableString(value: string | null | undefined): string | null {
        const trimmed = (value ?? '').trim();
        return trimmed === '' ? null : trimmed;
    }

    private normalizeBirthDate(value: string | null | undefined): string | null {
        const trimmed = (value ?? '').trim();
        return trimmed === '' ? null : trimmed;
    }
}
