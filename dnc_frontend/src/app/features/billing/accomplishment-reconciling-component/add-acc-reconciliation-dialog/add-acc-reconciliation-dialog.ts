import {CommonModule} from '@angular/common';
import {Component, ElementRef, Inject, ViewChild} from '@angular/core';
import {AbstractControl,
    FormBuilder,
    FormControl,
    FormGroup,
    ReactiveFormsModule,
    ValidationErrors,
    Validators,} from '@angular/forms';
import {MAT_DIALOG_DATA, MatDialog, MatDialogModule, MatDialogRef,} from '@angular/material/dialog';
import {MatButtonModule} from '@angular/material/button';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatInputModule} from '@angular/material/input';
import {MatAutocompleteModule} from '@angular/material/autocomplete';
import {MatSelectModule} from '@angular/material/select';
import {MatIconModule} from '@angular/material/icon';
import {firstValueFrom, Observable, of} from 'rxjs';
import {map, startWith} from 'rxjs/operators';
import {ConfirmNewMemberDialog, ConfirmNewMemberDialogData, ConfirmNewMemberDialogResult} from './confirm-new-member-dialog/confirm-new-member-dialog';
// replace this import with your real service
import {MasterListMemberService, SaveMemberNameRequest} from '../../../../api_services/master-list-members-service';
import {MatDatepickerModule} from '@angular/material/datepicker';
import {MatNativeDateModule, provideNativeDateAdapter} from '@angular/material/core';



export interface CreateAccReconciliationRequest {
    company_id: number;
    dentist_id: number;
    member_id: number | null;
    member_name: string;
    dental_service_id: number;
    date_service_performed: string | null;
    approval_code: string | null;
    tooth_id: string | null;
    tooth_service_type_id: number | null;
    tooth_surface_id: number | null;
}

export interface IdLabelOption {
    id: number;
    label: string;
}

export interface AddAccReconciliationDialogData {
    companies: IdLabelOption[];
    dentists: IdLabelOption[];
    dental_services: IdLabelOption[];
    tooth_service_types: IdLabelOption[];
    tooth_surfaces: IdLabelOption[];
}

@Component({
    selector: 'app-add-acc-reconciliation-dialog',
    standalone: true,
    imports: [
        CommonModule,
        ReactiveFormsModule,
        MatDialogModule,
        MatButtonModule,
        MatFormFieldModule,
        MatInputModule,
        MatAutocompleteModule,
        MatSelectModule,
        MatIconModule,
        MatDatepickerModule,
        MatNativeDateModule,
    ],
    providers: [provideNativeDateAdapter()],
    templateUrl: './add-acc-reconciliation-dialog.html',
    styleUrl: './add-acc-reconciliation-dialog.scss',
})
export class AddAccReconciliationDialog {
    @ViewChild('memberInput') private memberInput?: ElementRef<HTMLInputElement>;
    readonly isSavingMember = new FormControl<boolean>(false, {nonNullable: true});
    private isConfirmingNewMember = false;

    readonly form: FormGroup<{
        company_id: FormControl<number | null>;
        dentist_id: FormControl<number | null>;
        member_id: FormControl<number | null>;
        dental_service_id: FormControl<number | null>;
        date_service_performed: FormControl<Date | null>;
        approval_code: FormControl<string>;
        tooth_id: FormControl<string>;
        tooth_service_type_id: FormControl<number | null>;
        tooth_surface_id: FormControl<number | null>;
    }>;

    readonly companySearch = new FormControl<string>('', {nonNullable: true});
    readonly dentistSearch = new FormControl<string>('', {nonNullable: true});
    readonly memberSearch = new FormControl<string>('', {
        nonNullable: true,
        validators: [Validators.required,
        memberNameFormatValidator],
    });
    readonly dentalServiceSearch = new FormControl<string>('', {nonNullable: true});

    readonly filteredCompanies$: Observable<IdLabelOption[]>;
    readonly filteredDentists$: Observable<IdLabelOption[]>;
    readonly filteredDentalServices$: Observable<IdLabelOption[]>;
    filteredMembers$: Observable<IdLabelOption[]> = of([]);

    private members: IdLabelOption[] = [];

    readonly isLoadingMembers = new FormControl<boolean>(false, {nonNullable: true});

    constructor(
        private readonly fb: FormBuilder,
        private readonly dialogRef: MatDialogRef<
            AddAccReconciliationDialog,
            CreateAccReconciliationRequest | null
        >,
        private readonly memberService: MasterListMemberService,
        private readonly dialog: MatDialog,
        @Inject(MAT_DIALOG_DATA) public readonly data: AddAccReconciliationDialogData,
    ) {
        this.form = this.fb.group({
            company_id: this.fb.control<number | null>(null, {validators: [Validators.required]}),
            dentist_id: this.fb.control<number | null>(null, {validators: [Validators.required]}),
            member_id: this.fb.control<number | null>(null),
            dental_service_id: this.fb.control<number | null>(null, {validators: [Validators.required]}),
            date_service_performed: this.fb.control<Date | null>(null),
            approval_code: this.fb.control<string>('', {nonNullable: true}),
            tooth_id: this.fb.control<string>('', {nonNullable: true}),
            tooth_service_type_id: this.fb.control<number | null>(null),
            tooth_surface_id: this.fb.control<number | null>(null),
        });

        this.filteredCompanies$ = this.companySearch.valueChanges.pipe(
            startWith(''),
            map(value => this.filterOptions(this.data.companies, value)),
        );

        this.filteredDentists$ = this.dentistSearch.valueChanges.pipe(
            startWith(''),
            map(value => this.filterOptions(this.data.dentists, value)),
        );

        this.filteredDentalServices$ = this.dentalServiceSearch.valueChanges.pipe(
            startWith(''),
            map(value => this.filterOptions(this.data.dental_services, value)),
        );

        this.resetMemberFiltering();
    }

    private filterOptions(options: IdLabelOption[], value: string): IdLabelOption[] {
        const needle = value.trim().toLowerCase();
        if (!needle) {
            return options;
        }

        return options.filter(
            option =>
                option.label.toLowerCase().includes(needle) ||
                String(option.id).includes(needle),
        );
    }

    private resetMemberFiltering(): void {
        this.filteredMembers$ = this.memberSearch.valueChanges.pipe(
            startWith(this.memberSearch.value),
            map(value => this.filterOptions(this.members, value)),
        );
    }

    onCompanySelected(option: IdLabelOption): void {
        this.form.controls.company_id.setValue(option.id);
        this.companySearch.setValue(option.label, {emitEvent: false});

        this.clearMember();

        this.isLoadingMembers.setValue(true);

        this.memberService.getAllMemberNamesFromCompany(option.id).subscribe({
            next: (members) => {
                this.members = members.map((member) => ({
                    id: member.id,
                    label: member.full_name,
                }));
                this.resetMemberFiltering();
                this.memberSearch.setValue('', {emitEvent: true});
                this.isLoadingMembers.setValue(false);
            },
            error: () => {
                this.members = [];
                this.resetMemberFiltering();
                this.isLoadingMembers.setValue(false);
            },
        });
    }

    onDentistSelected(option: IdLabelOption): void {
        this.form.controls.dentist_id.setValue(option.id);
        this.dentistSearch.setValue(option.label, {emitEvent: false});
    }

    onMemberSelected(option: IdLabelOption): void {
        this.form.controls.member_id.setValue(option.id);
        this.memberSearch.setValue(option.label, {emitEvent: false});
    }

    onMemberInput(): void {
        this.form.controls.member_id.setValue(null);
    }

    onDentalServiceSelected(option: IdLabelOption): void {
        this.form.controls.dental_service_id.setValue(option.id);
        this.dentalServiceSearch.setValue(option.label, {emitEvent: false});
    }

    clearCompany(): void {
        this.form.controls.company_id.setValue(null);
        this.companySearch.setValue('');
        this.clearMember();
        this.members = [];
        this.resetMemberFiltering();
    }

    clearDentist(): void {
        this.form.controls.dentist_id.setValue(null);
        this.dentistSearch.setValue('');
    }

    clearMember(): void {
        this.form.controls.member_id.setValue(null);
        this.memberSearch.setValue('');
    }

    clearDentalService(): void {
        this.form.controls.dental_service_id.setValue(null);
        this.dentalServiceSearch.setValue('');
    }

    cancel(): void {
        this.dialogRef.close(null);
    }

    async save(): Promise<void> {
        if (
            this.form.invalid ||
            !this.memberSearch.value.trim()
        ) {
            this.form.markAllAsTouched();
            this.memberSearch.markAsTouched();
            return;
        }
        const memberReady = await this.ensureTypedMemberExistsOrCreate();
        if (!memberReady) {
            return;
        }

        const raw = this.form.getRawValue();
        const memberName = this.memberSearch.value.trim();

        const payload: CreateAccReconciliationRequest = {
            company_id: raw.company_id as number,
            dentist_id: raw.dentist_id as number,
            member_id: raw.member_id ?? null,
            member_name: memberName,
            dental_service_id: raw.dental_service_id as number,
            date_service_performed: this.formatDateOnly(raw.date_service_performed),
            approval_code: raw.approval_code.trim() || null,
            tooth_id: raw.tooth_id.trim() || null,
            tooth_service_type_id: raw.tooth_service_type_id ?? null,
            tooth_surface_id: raw.tooth_surface_id ?? null,
        };

        this.dialogRef.close(payload);
    }


    // region Helper functions
    private formatDateOnly(value: Date|null): string | null {
        if (!value) {
            return null;
        }
        const year = value.getFullYear();
        const month = String(value.getMonth() + 1).padStart(2, '0');
        const day = String(value.getDate()).padStart(2, '0');
        return `${year}-${month}-${day}`;
    }

    // Normalizes names so "Juan  Dela Cruz" and "juan dela cruz" match.
    private normalizeName(value: string): string {
        return value.trim().replace(/\s+/g, ' ').toLowerCase();
    }

    // Checks if the typed member name already exists in the loaded member list.
    private findExistingMemberByName(name: string): IdLabelOption | null {
        const normalizedName = this.normalizeName(name);

        return this.members.find(
            member => this.normalizeName(member.label) === normalizedName
        ) ?? null;
    }

    // Converts whatever saveMemberForCompany returns into IdLabelOption.
    // Adjust this if your backend returns a different shape.
    private toMemberOption(savedMember: any, fallbackName: string): IdLabelOption {
        return {
            id: savedMember.id,
            label: savedMember.full_name ?? savedMember.name ?? fallbackName,
        };
    }

    // Runs when member input loses focus.
    async onMemberBlur(): Promise<void> {
        console.log('onMemberBlur()');
        // Let mat-autocomplete selection finish first before checking typed text.
        setTimeout(async () => {
            await this.ensureTypedMemberExistsOrCreate();
        });
    }

    // If typed the member is new, prompt user, save it, then select it.
    private async ensureTypedMemberExistsOrCreate(): Promise<boolean> {
        const companyId = this.form.controls.company_id.value;
        const typedName = this.memberSearch.value.trim();

        if (this.memberSearch.invalid){
            this.memberSearch.markAsTouched();
            return false;
        }

        if (!companyId || !typedName) {
            return false;
        }

        // Existing selected member: proceed as usual.
        if (this.form.controls.member_id.value) {
            return true;
        }

        // Typed name exactly matches an existing member: select it automatically.
        const existingMember = this.findExistingMemberByName(typedName);
        if (existingMember) {
            this.onMemberSelected(existingMember);
            return true;
        }

        if (this.isConfirmingNewMember) {
            return false;
        }

        this.isConfirmingNewMember = true;

        const result = await firstValueFrom(
            this.dialog.open<
                ConfirmNewMemberDialog,
                ConfirmNewMemberDialogData,
                ConfirmNewMemberDialogResult
            >(ConfirmNewMemberDialog, {
                width: '420px',
                disableClose: true,
                data: {
                    name: typedName,
                },
            }).afterClosed()
        );

        this.isConfirmingNewMember = false;

        if (!result?.confirmed) {
            this.clearMember();
            setTimeout(() => this.memberInput?.nativeElement.focus());
            return false;
        }

        const payload: SaveMemberNameRequest = {
            name: typedName,
            account_number: result.account_number,
        };

        this.isSavingMember.setValue(true);

        try {
            const savedMember = await firstValueFrom(
                this.memberService.saveMemberForCompany(companyId, payload)
            );

            const savedOption = this.toMemberOption(savedMember, typedName);

            this.members = [...this.members, savedOption];
            this.resetMemberFiltering();
            this.onMemberSelected(savedOption);

            return true;
        } catch {
            this.clearMember();
            setTimeout(() => this.memberInput?.nativeElement.focus());
            return false;
        } finally {
            this.isSavingMember.setValue(false);
        }
    }
    // endregion Helper functions

}

function memberNameFormatValidator(control: AbstractControl): ValidationErrors | null {
    const value = String(control.value ?? '').trim();

    if (!value) {
        return null;
    }

    // LASTNAME, FIRSTNAME or LASTNAME, FIRSTNAME MIDDLENAME
    const isValid = /^[A-Za-zÑñ.' -]+,\s*[A-Za-zÑñ.' -]+(?:\s+[A-Za-zÑñ.' -]+)*$/.test(value);

    return isValid ? null : { memberNameFormat: true };
}
