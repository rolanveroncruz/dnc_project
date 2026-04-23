import {CommonModule} from '@angular/common';
import {Component, Inject} from '@angular/core';
import {FormBuilder, FormControl, FormGroup, ReactiveFormsModule, Validators,} from '@angular/forms';
import {MAT_DIALOG_DATA, MatDialogModule, MatDialogRef,} from '@angular/material/dialog';
import {MatButtonModule} from '@angular/material/button';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatInputModule} from '@angular/material/input';
import {MatAutocompleteModule} from '@angular/material/autocomplete';
import {MatSelectModule} from '@angular/material/select';
import {MatIconModule} from '@angular/material/icon';
import {Observable, of} from 'rxjs';
import {map, startWith} from 'rxjs/operators';

// replace this import with your real service
import {MasterListMemberService} from '../../../../api_services/master-list-members-service';

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
    ],
    templateUrl: './add-acc-reconciliation-dialog.html',
    styleUrl: './add-acc-reconciliation-dialog.scss',
})
export class AddAccReconciliationDialog {
    readonly form: FormGroup<{
        company_id: FormControl<number | null>;
        dentist_id: FormControl<number | null>;
        member_id: FormControl<number | null>;
        dental_service_id: FormControl<number | null>;
        date_service_performed: FormControl<string | null>;
        approval_code: FormControl<string>;
        tooth_id: FormControl<string>;
        tooth_service_type_id: FormControl<number | null>;
        tooth_surface_id: FormControl<number | null>;
    }>;

    readonly companySearch = new FormControl<string>('', {nonNullable: true});
    readonly dentistSearch = new FormControl<string>('', {nonNullable: true});
    readonly memberSearch = new FormControl<string>('', {
        nonNullable: true,
        validators: [Validators.required],
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
        @Inject(MAT_DIALOG_DATA) public readonly data: AddAccReconciliationDialogData,
    ) {
        this.form = this.fb.group({
            company_id: this.fb.control<number | null>(null, {validators: [Validators.required]}),
            dentist_id: this.fb.control<number | null>(null, {validators: [Validators.required]}),
            member_id: this.fb.control<number | null>(null),
            dental_service_id: this.fb.control<number | null>(null, {validators: [Validators.required]}),
            date_service_performed: this.fb.control<string | null>(null),
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

    save(): void {
        if (
            this.form.invalid ||
            !this.memberSearch.value.trim()
        ) {
            this.form.markAllAsTouched();
            this.memberSearch.markAsTouched();
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
            date_service_performed: raw.date_service_performed ?? null,
            approval_code: raw.approval_code.trim() || null,
            tooth_id: raw.tooth_id.trim() || null,
            tooth_service_type_id: raw.tooth_service_type_id ?? null,
            tooth_surface_id: raw.tooth_surface_id ?? null,
        };

        this.dialogRef.close(payload);
    }
}
