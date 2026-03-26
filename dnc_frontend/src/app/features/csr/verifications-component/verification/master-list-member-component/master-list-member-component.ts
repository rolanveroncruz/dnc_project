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
import { CommonModule } from '@angular/common';
import { FormControl, ReactiveFormsModule } from '@angular/forms';
import { takeUntilDestroyed } from '@angular/core/rxjs-interop';
import { MatChipsModule} from '@angular/material/chips';

import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatButtonModule } from '@angular/material/button';
import { MatDatepickerModule } from '@angular/material/datepicker';
import { MatNativeDateModule, MatOption } from '@angular/material/core';
import { MatCardModule } from '@angular/material/card';
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
    }|null> (null);

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
    private setMemberEditBaseline(): void{
        this.memberEditBaseline.set( this.currentMemberEditSnapshot());
        this.hasUnsavedChanges.set(false);
        this.bumpMemberEditVersion();
    }
    private bumpMemberEditVersion(): void{
        this.memberEditVersion.update(v => v + 1);
    }
    resetMemberEdits():void{
        const baseline = this.memberEditBaseline();
        if (!baseline)return;

        this.lastName.setValue(baseline.lastName, { emitEvent: false });
        this.firstName.setValue(baseline.firstName, { emitEvent: false });
        this.middleName.setValue(baseline.middleName, { emitEvent: false });
        this.mobileNumber.setValue(baseline.mobileNumber, { emitEvent: false });
        this.emailAddress.setValue(baseline.emailAddress, { emitEvent: false });
        this.birthDate.setValue(baseline.birthDate, { emitEvent: false });

        this.hasUnsavedChanges.set(false);
        this.bumpMemberEditVersion();
    }
    saveMemberEdits(): void {
        this.setMemberEditBaseline();
        this.infoMessage.set('Member changes saved.');
        this.saveError.set('');
    }

    /***********************
     * ✅ Endorsement selection
     ***********************/
    readonly endorsementSearch = new FormControl<string | number>('', { nonNullable: true }); // ✅ widened type
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
    readonly memberAccountSearch = new FormControl<string | number>('', { nonNullable: true }); // ✅ new
    readonly memberAccountSearchText = signal(''); // ✅ new

    readonly loadingMembers = signal(false); // ✅ new
    readonly members = signal<MasterListMemberLookupResponse[]>([]); // ✅ new
    readonly resolvedMemberId = signal<number | null>(null); // ✅ new

    readonly filteredMembers = computed(() => {
        const search = this.memberAccountSearchText().trim().toLowerCase();
        const items = [...this.members()].sort((a, b) =>
            (a.master_list_member_last_name ?? '').localeCompare(
                b.master_list_member_last_name ?? '',
                undefined,
                { numeric: true, sensitivity: 'base' }
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
    readonly lastName = new FormControl<string>('', { nonNullable: true }); // ✅ new
    readonly firstName = new FormControl<string>('', { nonNullable: true }); // ✅ new
    readonly middleName = new FormControl<string>('', { nonNullable: true }); // ✅ new
    readonly mobileNumber = new FormControl<string>('', { nonNullable: true }); // ✅ new
    readonly emailAddress = new FormControl<string>('', { nonNullable: true }); // ✅ new
    readonly birthDate = new FormControl<string>('', { nonNullable: true }); // ✅ new

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
            .subscribe(()=>{
                if (this.memberEditBaseline()!==null) this.hasUnsavedChanges.set(true);
                this.bumpMemberEditVersion();
            });
        this.firstName.valueChanges
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe(()=>{
                if (this.memberEditBaseline()!==null) this.hasUnsavedChanges.set(true);
                this.bumpMemberEditVersion();
            });
        this.middleName.valueChanges
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe(()=>{
                if (this.memberEditBaseline()!==null) this.hasUnsavedChanges.set(true);
                this.bumpMemberEditVersion();
            });
        this.mobileNumber.valueChanges
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe(()=>{
                if (this.memberEditBaseline()!==null) this.hasUnsavedChanges.set(true);
                this.bumpMemberEditVersion();
            });
        this.emailAddress.valueChanges
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe(()=>{
                if (this.memberEditBaseline()!==null) this.hasUnsavedChanges.set(true);
                this.bumpMemberEditVersion();
            });
        this.birthDate.valueChanges
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe(()=>{
                if (this.memberEditBaseline()!==null) this.hasUnsavedChanges.set(true);
                this.bumpMemberEditVersion();
            });

        // ✅ safe subscription for agreement number autocomplete
        this.endorsementSearch.valueChanges
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe(value => {
                const searchText = typeof value === 'string' ? value.trim().toLowerCase() : '';
                this.endorsementSearchText.set(searchText);

                // ✅ only clear when user is typing text
                if (typeof value === 'string') {
                    this.selectedEndorsementId.set(null);
                    this.selectedHmoName.set('');
                    this.selectedCompanyName.set('');
                    this.clearMemberSection();
                }
            });

        // ✅ safe subscription for member account autocomplete
        this.memberAccountSearch.valueChanges
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe(value => {
                const searchText = typeof value === 'string' ? value.trim().toLowerCase() : '';
                this.memberAccountSearchText.set(searchText);

                // ✅ if user is typing manually, this is no longer a resolved existing member
                if (typeof value === 'string') {
                    this.resolvedMemberId.set(null);
                    this.selectedMasterListMemberIdChange.emit(null);
                    this.clearMemberFieldsOnly();
                    this.setMemberEditBaseline();
                }
            });
    }

    // ✅ full reset when dentist changes
    private resetAll(): void {
        this.endorsements.set([]);
        this.endorsementSearch.setValue('', { emitEvent: false });
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
        this.memberAccountSearch.setValue('', { emitEvent: false });
        this.memberAccountSearchText.set('');
        this.resolvedMemberId.set(null);
        this.selectedMasterListMemberIdChange.emit(null);
        this.clearMemberFieldsOnly();
        this.memberEditBaseline.set(null);
        this.hasUnsavedChanges.set(false);
    }

    // ✅ clear just the detail fields
    private clearMemberFieldsOnly(): void {
        this.lastName.setValue('', { emitEvent: false });
        this.firstName.setValue('', { emitEvent: false });
        this.middleName.setValue('', { emitEvent: false });
        this.mobileNumber.setValue('', { emitEvent: false });
        this.emailAddress.setValue('', { emitEvent: false });
        this.birthDate.setValue('', { emitEvent: false });
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

        // ✅ show agreement number in text box
        this.endorsementSearch.setValue(selected.agreement_corp_number ?? '', { emitEvent: false });

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
        this.memberAccountSearch.setValue(selected.master_list_member_account_no ?? '', { emitEvent: false });

        // ✅ populate member details
        this.lastName.setValue(selected.master_list_member_last_name ?? '', { emitEvent: false });
        this.firstName.setValue(selected.master_list_member_first_name ?? '', { emitEvent: false });
        this.middleName.setValue(selected.master_list_member_middle_name ?? '', { emitEvent: false });
        this.mobileNumber.setValue(selected.master_list_member_mobile_number ?? '', { emitEvent: false });
        this.emailAddress.setValue(selected.master_list_member_email_address ?? '', { emitEvent: false });
        this.birthDate.setValue(selected.master_list_member_birth_date ?? '', { emitEvent: false });

        this.setMemberEditBaseline();

    }

    // ✅ optional helper if user wants to start fresh
    startNewMember(): void {
        this.memberAccountSearch.setValue('', { emitEvent: false });
        this.memberAccountSearchText.set('');
        this.resolvedMemberId.set(null);
        this.selectedMasterListMemberIdChange.emit(null);
        this.clearMemberFieldsOnly();
        this.setMemberEditBaseline();
        this.infoMessage.set('Enter a new Member Account No and fill in the member details.');
    }

    clear(): void {
        this.endorsementSearch.setValue('', { emitEvent: false });
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
}
