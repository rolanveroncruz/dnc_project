import {
    Component,
    DestroyRef,
    computed,
    inject,
    signal,
    input,
    output,
    effect,
} from '@angular/core';
import {CommonModule} from '@angular/common';
import {FormControl, ReactiveFormsModule} from '@angular/forms';
import {takeUntilDestroyed} from '@angular/core/rxjs-interop';
import {MatChipsModule} from '@angular/material/chips';

import {MatFormFieldModule} from '@angular/material/form-field';
import {MatInputModule} from '@angular/material/input';
import {MatButtonModule} from '@angular/material/button';
import {MatDatepickerModule} from '@angular/material/datepicker';
import {MatNativeDateModule, MatOption} from '@angular/material/core';
import {MatCardModule} from '@angular/material/card';
import {
    MasterListMemberService,
    MasterListMemberLookupResponse,
} from '../../../../../api_services/master-list-members-service';
import {
    EndorsementService,
    DentistEndorsementLookupResponse,
} from '../../../../../api_services/endorsement-service';
import {
    MatAutocomplete,
    MatAutocompleteSelectedEvent,
    MatAutocompleteTrigger,
} from '@angular/material/autocomplete';

@Component({
    selector: 'app-master-list-member',
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
        MatAutocompleteTrigger,
        MatAutocomplete,
        MatOption,
        MatChipsModule,
    ],
    templateUrl: './master-list-member-component.html',
    styleUrl: './master-list-member-component.scss',
})
export class MasterListMemberComponent {
    private readonly destroyRef = inject(DestroyRef);
    private readonly masterListMemberService = inject(MasterListMemberService);
    private readonly endorsementService = inject(EndorsementService);

    readonly dentistId = input<number | null>(null);
    readonly selectedMasterListMemberId = input<number | null>(null);
    readonly selectedMasterListMemberIdChange = output<number | null>();


    /***********************
     * ✅ Master List Member info
     ***********************/
    readonly memberEditVersion = signal(0);

    readonly memberEditBaseline = signal<{
        lastName: string;
        firstName: string;
        middleName: string;
        mobileNumber: string;
        emailAddress: string;
        birthDate: string;
    } | null>(null);

    readonly hasUnsavedChanges = signal(false);

    private currentMemberEditSnapshot() {
        return {
            lastName: this.lastName.value ?? '',
            firstName: this.firstName.value ?? '',
            middleName: this.middleName.value ?? '',
            mobileNumber: this.mobileNumber.value ?? '',
            emailAddress: this.emailAddress.value ?? '',
            birthDate: this.birthDate.value ?? '',
        };
    }

    private setMemberEditBaseline(): void {
        this.memberEditBaseline.set(this.currentMemberEditSnapshot());
        this.hasUnsavedChanges.set(false);
        this.bumpMemberEditVersion();
    }

    private bumpMemberEditVersion(): void {
        this.memberEditVersion.update(v => v + 1);
    }

    resetMemberEdits(): void {
        const baseline = this.memberEditBaseline();
        if (!baseline) return;

        this.lastName.setValue(baseline.lastName, {emitEvent: false});
        this.firstName.setValue(baseline.firstName, {emitEvent: false});
        this.middleName.setValue(baseline.middleName, {emitEvent: false});
        this.mobileNumber.setValue(baseline.mobileNumber, {emitEvent: false});
        this.emailAddress.setValue(baseline.emailAddress, {emitEvent: false});
        this.birthDate.setValue(baseline.birthDate, {emitEvent: false});

        this.hasUnsavedChanges.set(false);
        this.bumpMemberEditVersion();
    }

    // region: Save Member Edits
    // Effectively, this is a "save" button.
    // It will save the current member edits to the database.
    // It will also clear the member edit baseline.
    // It will also clear the error message.
    // It will also clear the info message.
    // It will also clear the "unsaved changes" flag.
    saveMemberEdits(): void {
        const endorsementId = this.selectedEndorsementId();
        const accountNumber = this.normalizedAccountNumber();

        if (endorsementId === null) {
            this.saveError.set('Please select an endorsement.');
            return;
        }
        if (!accountNumber) {
            this.saveError.set('Please provide or select a member account number.');
            return;
        }
        if (!this.normalize(this.lastName.value)) {
            this.saveError.set('Last name is required.');
            this.infoMessage.set('');
            return;
        }

        if (!this.normalize(this.firstName.value)) {
            this.saveError.set('First name is required.');
            this.infoMessage.set('');
            return;
        }
        this.loading.set(true);
        this.saveError.set('');
        this.infoMessage.set('');

        const payload = {
            endorsement_id: endorsementId,
            account_number: accountNumber,
            last_name: this.normalize(this.lastName.value),
            first_name: this.normalize(this.firstName.value),
            middle_name: this.normalize(this.middleName.value),
            mobile_number: this.normalize(this.mobileNumber.value),
            email_address: this.normalize(this.emailAddress.value),
            birth_date: this.normalize(this.birthDate.value),
            is_active: true,
            last_edited_by: null,
        }
        const existingMemberId = this.resolvedMemberId();
        if (existingMemberId !== null) {
            this.masterListMemberService.patchMasterListMember(existingMemberId, payload).subscribe({
                next: (res) => {
                    this.resolvedMemberId.set(res.id);
                    this.selectedMasterListMemberIdChange.emit(res.id);

                    this.memberAccountSearch.setValue(res.account_number, {emitEvent: false});
                    this.lastName.setValue(res.last_name, {emitEvent: false});
                    this.firstName.setValue(res.first_name, {emitEvent: false});
                    this.middleName.setValue(res.middle_name, {emitEvent: false});
                    this.mobileNumber.setValue(res.mobile_number ?? '', {emitEvent: false});
                    this.emailAddress.setValue(res.email_address ?? '', {emitEvent: false});
                    this.birthDate.setValue(res.birth_date ?? '', {emitEvent: false});

                    this.setMemberEditBaseline();
                    this.infoMessage.set('Member details updated successfully.');
                    this.saveError.set('');
                    this.loading.set(false);
                },
                error: (err) => {
                    console.error('Failed to update member', err);
                    this.saveError.set('Failed to update member.');
                    this.infoMessage.set('');
                    this.loading.set(false);
                }
            });
            return;
        }
        if (this.isCreatingNewMember()) {
            this.masterListMemberService.createMasterListMember(payload).subscribe({
                next: (res) => {
                    this.resolvedMemberId.set(res.id);
                    this.selectedMasterListMemberIdChange.emit(res.id);

                    this.memberAccountSearch.setValue(res.account_number, {emitEvent: false});
                    this.lastName.setValue(res.last_name, {emitEvent: false});
                    this.firstName.setValue(res.first_name, {emitEvent: false});
                    this.middleName.setValue(res.middle_name, {emitEvent: false});
                    this.mobileNumber.setValue(res.mobile_number ?? '', {emitEvent: false});
                    this.emailAddress.setValue(res.email_address ?? '', {emitEvent: false});
                    this.birthDate.setValue(res.birth_date ?? '', {emitEvent: false});
                    this.members.update(list => [
                        ...list,
                        {
                            master_list_member_id: res.id,
                            endorsement_id: endorsementId,
                            endorsement_agreement_corp_number: null,
                            master_list_member_account_no: res.account_number,
                            master_list_member_last_name: res.last_name,
                            master_list_member_first_name: res.first_name,
                            master_list_member_middle_name: res.middle_name,
                            master_list_member_name: `${res.last_name}, ${res.first_name} ${res.middle_name}`.trim(),
                            master_list_member_email_address: res.email_address,
                            master_list_member_mobile_number: res.mobile_number,
                            master_list_member_birth_date: res.birth_date,
                            master_list_member_is_active: res.is_active,
                        }
                    ]);
                    this.setMemberEditBaseline();
                    this.infoMessage.set('Member created successfully.');
                    this.saveError.set('');
                    this.loading.set(false);
                },
                error: (err) => {
                    console.error('Failed to create member', err);
                    this.saveError.set('Failed to create member.');
                    this.infoMessage.set('');
                    this.loading.set(false);
                }
            });
            return;
        }
        this.saveError.set('Please select an existing member or enter a new account member.');
        this.infoMessage.set('');
        this.loading.set(false);

    }

    // endregion: Save Member Edits

    /***********************
     * ✅ Endorsement selection
     ***********************/
    readonly endorsementSearch = new FormControl<string | number>('', {nonNullable: true}); // ✅ widened type
    readonly endorsementSearchText = signal('');

    readonly loadingEndorsements = signal(false);
    readonly endorsements = signal<DentistEndorsementLookupResponse[]>([]);
    readonly selectedEndorsementId = signal<number | null>(null);

    readonly selectedHmoName = signal<string>('');
    readonly selectedCompanyName = signal<string>('');

    readonly filteredEndorsements = computed(() => {
        const search = this.endorsementSearchText().trim().toLowerCase();
        const items = this.endorsements();

        if (!search) {
            return items;
        }

        return items.filter(item =>
            (item.agreement_corp_number ?? '').toLowerCase().includes(search)
        );
    });

    /***********************
     * ✅ Member account selection
     ***********************/
    readonly memberAccountSearch = new FormControl<string | number>('', {nonNullable: true}); // ✅ new
    readonly memberAccountSearchText = signal(''); // ✅ new



    // if this is a new member account.

    readonly loadingMembers = signal(false); // ✅ new
    readonly members = signal<MasterListMemberLookupResponse[]>([]); // ✅ new
    readonly resolvedMemberId = signal<number | null>(null); // ✅ new

    readonly filteredMembers = computed(() => {
        const search = this.memberAccountSearchText().trim().toLowerCase();
        const items = [...this.members()].sort((a, b) =>
            (a.master_list_member_last_name ?? '').localeCompare(
                b.master_list_member_last_name ?? '',
                undefined,
                {numeric: true, sensitivity: 'base'}
            )
        );

        if (!search) {
            return items;
        }


        return items.filter(item =>
            (item.master_list_member_account_no ?? '').toLowerCase().includes(search)
        );
    });


    /***********************
     * ✅ Member detail fields
     ***********************/
    readonly lastName = new FormControl<string>('', {nonNullable: true}); // ✅ new
    readonly firstName = new FormControl<string>('', {nonNullable: true}); // ✅ new
    readonly middleName = new FormControl<string>('', {nonNullable: true}); // ✅ new
    readonly mobileNumber = new FormControl<string>('', {nonNullable: true}); // ✅ new
    readonly emailAddress = new FormControl<string>('', {nonNullable: true}); // ✅ new
    readonly birthDate = new FormControl<string>('', {nonNullable: true}); // ✅ new

    readonly infoMessage = signal<string>('');
    readonly saveError = signal<string>('');
    readonly loading = signal(false);

    constructor() {
        // ✅ reload endorsements whenever dentist changes
        effect(() => {
            const dentistId = this.dentistId();

            this.resetAll();

            if (dentistId === null) {
                return;
            }
            this.loadEndorsementsForDentist(dentistId);
        });
        this.lastName.valueChanges
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe(() => {
                if (this.memberEditBaseline() !== null) this.hasUnsavedChanges.set(true);
                this.bumpMemberEditVersion();
            });
        this.firstName.valueChanges
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe(() => {
                if (this.memberEditBaseline() !== null) this.hasUnsavedChanges.set(true);
                this.bumpMemberEditVersion();
            });
        this.middleName.valueChanges
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe(() => {
                if (this.memberEditBaseline() !== null) this.hasUnsavedChanges.set(true);
                this.bumpMemberEditVersion();
            });
        this.mobileNumber.valueChanges
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe(() => {
                if (this.memberEditBaseline() !== null) this.hasUnsavedChanges.set(true);
                this.bumpMemberEditVersion();
            });
        this.emailAddress.valueChanges
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe(() => {
                if (this.memberEditBaseline() !== null) this.hasUnsavedChanges.set(true);
                this.bumpMemberEditVersion();
            });
        this.birthDate.valueChanges
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe(() => {
                if (this.memberEditBaseline() !== null) this.hasUnsavedChanges.set(true);
                this.bumpMemberEditVersion();
            });

        // ✅ safe subscription for agreement number autocomplete
        this.endorsementSearch.valueChanges
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe(value => {
                const searchText = typeof value === 'string' ? value.trim().toLowerCase() : '';
                this.endorsementSearchText.set(searchText);

                // ✅ only clear when the user is typing text
                if (typeof value === 'string') {
                    this.selectedEndorsementId.set(null);
                    this.selectedHmoName.set('');
                    this.selectedCompanyName.set('');
                    this.clearMemberSection();
                }
            });

        // ✅ safe subscription for member account autocomplete
        // This is called whenever the user types something.
        this.memberAccountSearch.valueChanges
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe(value => {
                const searchText = typeof value === 'string' ? value.trim().toLowerCase() : '';
                this.memberAccountSearchText.set(searchText);

                // ✅ if the user is typing manually, this is no longer a resolved existing member
                if (typeof value === 'string') {
                    this.resolvedMemberId.set(null);
                    this.selectedMasterListMemberIdChange.emit(null);
                    this.clearMemberFieldsOnly();
                    this.setMemberEditBaseline();
                }
            });
    }

    // ✅ full reset when the dentist changes
    private resetAll(): void {
        this.endorsements.set([]);
        this.endorsementSearch.setValue('', {emitEvent: false});
        this.endorsementSearchText.set('');
        this.selectedEndorsementId.set(null);
        this.selectedHmoName.set('');
        this.selectedCompanyName.set('');
        this.clearMemberSection();
        this.infoMessage.set('');
        this.saveError.set('');
        this.memberEditBaseline.set(null);
        this.hasUnsavedChanges.set(false);
    }

    // ✅ clear all member-related state
    private clearMemberSection(): void {
        this.members.set([]);
        this.loadingMembers.set(false);
        this.memberAccountSearch.setValue('', {emitEvent: false});
        this.memberAccountSearchText.set('');
        this.resolvedMemberId.set(null);
        this.selectedMasterListMemberIdChange.emit(null);
        this.clearMemberFieldsOnly();
        this.memberEditBaseline.set(null);
        this.hasUnsavedChanges.set(false);
    }

    // ✅ clear just the detail fields
    private clearMemberFieldsOnly(): void {
        this.lastName.setValue('', {emitEvent: false});
        this.firstName.setValue('', {emitEvent: false});
        this.middleName.setValue('', {emitEvent: false});
        this.mobileNumber.setValue('', {emitEvent: false});
        this.emailAddress.setValue('', {emitEvent: false});
        this.birthDate.setValue('', {emitEvent: false});
        this.memberEditBaseline.set(null);
        this.hasUnsavedChanges.set(false);
    }

    private loadEndorsementsForDentist(dentistId: number): void {
        this.loadingEndorsements.set(true);
        this.saveError.set('');

        this.endorsementService.getEndorsementsForDentist(dentistId).subscribe({
            next: (res: DentistEndorsementLookupResponse[]) => {
                this.endorsements.set(res);
                this.loadingEndorsements.set(false);
            },
            error: (err) => {
                console.error('Failed to load endorsements for dentist', err);
                this.saveError.set('Failed to load endorsements.');
                this.loadingEndorsements.set(false);
            }
        });
    }

    // ✅ load members for selected endorsement
    private loadMembersForEndorsement(endorsementId: number): void {
        this.loadingMembers.set(true);
        this.saveError.set('');

        // ✅ replace this with your actual service method name if different
        this.masterListMemberService.getMasterListMembersForEndorsement(endorsementId).subscribe({
            next: (res: MasterListMemberLookupResponse[]) => {
                this.members.set(res);
                this.loadingMembers.set(false);
            },
            error: (err) => {
                console.error('Failed to load members for endorsement', err);
                this.saveError.set('Failed to load members for the selected endorsement.');
                this.loadingMembers.set(false);
            }
        });
    }

    onEndorsementSelected(event: MatAutocompleteSelectedEvent): void {
        const endorsementId = event.option.value as number;
        this.selectedEndorsementId.set(endorsementId);

        const selected = this.endorsements().find(e => e.endorsement_id === endorsementId);
        if (!selected) {
            return;
        }

        // ✅ show the agreement number in the text box
        this.endorsementSearch.setValue(selected.agreement_corp_number ?? '', {emitEvent: false});

        // ✅ populate readonly display fields
        this.selectedCompanyName.set(selected.endorsement_company_name ?? '');
        this.selectedHmoName.set(selected.hmo_short_name ?? '');

        // ✅ clear previous member state and load matching members
        this.clearMemberSection();
        this.loadMembersForEndorsement(endorsementId);

        this.infoMessage.set('Choose an existing Member Account No, or type a new one.');
    }

    // ✅ when an existing member account is selected
    onMemberAccountSelected(event: MatAutocompleteSelectedEvent): void {
        const memberId = event.option.value as number;
        const selected = this.members().find(m => m.master_list_member_id === memberId);

        if (!selected) {
            return;
        }

        this.resolvedMemberId.set(selected.master_list_member_id);
        this.selectedMasterListMemberIdChange.emit(selected.master_list_member_id);

        // ✅ show account number in the field
        this.memberAccountSearch.setValue(selected.master_list_member_account_no ?? '', {emitEvent: false});

        // ✅ populate member details
        this.lastName.setValue(selected.master_list_member_last_name ?? '', {emitEvent: false});
        this.firstName.setValue(selected.master_list_member_first_name ?? '', {emitEvent: false});
        this.middleName.setValue(selected.master_list_member_middle_name ?? '', {emitEvent: false});
        this.mobileNumber.setValue(selected.master_list_member_mobile_number ?? '', {emitEvent: false});
        this.emailAddress.setValue(selected.master_list_member_email_address ?? '', {emitEvent: false});
        this.birthDate.setValue(selected.master_list_member_birth_date ?? '', {emitEvent: false});

        this.setMemberEditBaseline();

    }


    clear(): void {
        this.endorsementSearch.setValue('', {emitEvent: false});
        this.endorsementSearchText.set('');
        this.selectedEndorsementId.set(null);
        this.selectedHmoName.set('');
        this.selectedCompanyName.set('');
        this.clearMemberSection();
        this.infoMessage.set('');
        this.saveError.set('');
        this.memberEditBaseline.set(null);
        this.hasUnsavedChanges.set(false);
    }

    // region Helper Functions

    private normalize(value: string | null | undefined): string {
        return (value ?? '').trim();
    }

    private normalizedAccountNumber(): string {
        return this.normalize(this.memberAccountSearch.value?.toString());
    }

    private findExistingMemberByAccountNumber(): MasterListMemberLookupResponse | null {
        const typed = this.normalizedAccountNumber().toLowerCase();
        if (!typed) return null;

        return this.members().find(
            m => (m.master_list_member_account_no ?? '').trim().toLowerCase() === typed
        ) ?? null;
    }

    private isCreatingNewMember(): boolean {
        const endorsementId = this.selectedEndorsementId();
        const typedAccountNo = this.normalizedAccountNumber();

        if (endorsementId === null || !typedAccountNo) {
            return false;
        }

        if (this.resolvedMemberId() !== null) {
            return false;
        }

        return this.findExistingMemberByAccountNumber() === null;
    }

    // endregion: Helper Functions

}
