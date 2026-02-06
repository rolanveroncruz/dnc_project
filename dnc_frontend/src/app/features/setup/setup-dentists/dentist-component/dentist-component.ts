import {Component, DestroyRef, computed, effect, inject, signal} from '@angular/core';
import {ActivatedRoute, Router} from '@angular/router';
import {CommonModule} from '@angular/common';
import {FormBuilder, FormGroup, ReactiveFormsModule, Validators} from '@angular/forms';
import {takeUntilDestroyed} from '@angular/core/rxjs-interop';

import {MatCardModule} from '@angular/material/card';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatInputModule} from '@angular/material/input';
import {MatSelectModule} from '@angular/material/select';
import {MatButtonModule} from '@angular/material/button';
import {MatTabsModule} from '@angular/material/tabs';
import {MatProgressBarModule} from '@angular/material/progress-bar';
import {MatIconModule} from '@angular/material/icon';

import {MatDatepickerModule} from '@angular/material/datepicker';
import {MatNativeDateModule} from '@angular/material/core';
import {DentistService} from '../../../../api_services/dentist-service';
import {
    DentistHistory,
    DentistLookupsService,
    DentistStatus, TaxClassification,
    TaxType
} from '../../../../api_services/dentist-lookups-service';
import {DentistContractRow, DentistContractsService} from '../../../../api_services/dentist-contracts-service';
import {DentistClinicService, DentistClinicWithNames} from '../../../../api_services/dentist-clinic-service';
import {
    DataTableWithSelectComponent
} from '../../../../components/data-table-with-select-component/data-table-with-select-component';
import {TableColumn} from '../../../../components/generic-data-table-component/table-interfaces';
import {DentistHMORelationsService} from '../../../../api_services/dentist-hmorelations-service';
import {MatListOption, MatSelectionList} from '@angular/material/list';
import {MatDialog} from '@angular/material/dialog';
import {
    AddClinicOrDentistDialogComponent,
    AddClinicOrDentistDialogData,
    AddClinicOrDentistDialogResult
} from '../../add-clinic-or-dentist-dialog-component/add-clinic-or-dentist-dialog-component';
import {firstValueFrom, map} from 'rxjs';
import {DentalClinicService} from '../../../../api_services/dental-clinic-service';

/** Matches your API response shape */
export interface DentistWithLookups {
    id: number;
    last_name: string;
    given_name: string;
    middle_name: string | null;
    email: string | null;
    retainer_fee: number;
    dentist_status_id: number | null;
    dentist_history_id: number | null;
    dentist_requested_by: string | null;
    accre_dentist_contract_id: number | null;
    accre_document_code: string | null;
    accreditation_date: string | null;
    accre_contract_sent_date: string | null;
    accre_contract_file_path: string | null;
    acc_tin: string | null;
    acc_bank_name: string | null;
    acc_account_name: string | null;
    acc_account_number: string | null;
    acc_tax_type_id: number | null;
    acc_tax_classification_id: number | null;

    dentist_contract_name: string | null;
    dentist_history_name: string | null;
    dentist_status_name: string | null;
    tax_type_name: string | null;
    tax_classification_name: string | null;
}

export interface DentistOrClinicWithIdAndName {
    id: number;
    name: string;
}

/** Simple lookup option */
interface LookupOption {
    id: number;
    name: string;
}

/**
 * Replace these with your real service.
 * - getById(id)
 * - create(body)
 * - patch(id, body)
 * - lookup endpoints for status/history/contract/tax type/classification
 */
@Component({
    selector: 'app-dentist',
    standalone: true,
    imports: [
        CommonModule,
        ReactiveFormsModule,

        MatCardModule,
        MatFormFieldModule,
        MatInputModule,
        MatSelectModule,
        MatButtonModule,
        MatTabsModule,
        MatProgressBarModule,
        MatIconModule,

        MatDatepickerModule,
        MatNativeDateModule,
        DataTableWithSelectComponent,
        MatSelectionList,
        MatListOption
    ],
    templateUrl: './dentist-component.html',
    styleUrls: ['./dentist-component.scss'],
})
export class DentistComponent {
    private readonly fb = inject(FormBuilder);
    private readonly route = inject(ActivatedRoute);
    private readonly router = inject(Router);
    private readonly destroyRef = inject(DestroyRef);
    private readonly dentistService = inject(DentistService);
    private readonly dentistContractsService = inject(DentistContractsService);
    private readonly dentistLookupService = inject(DentistLookupsService);
    private readonly dentistClinicService = inject(DentistClinicService);
    private readonly dentistHMORelations = inject(DentistHMORelationsService)
    private readonly dentalClinicService = inject(DentalClinicService);
    readonly dialog = inject(MatDialog);

    // ---- State
    readonly loading = signal(false);
    readonly saving = signal(false);
    readonly loaded = signal(false);

    readonly dentistId = signal<number | null>(null);
    readonly isEditMode = computed(() => this.dentistId() !== null);

    // ---- Lookups
    readonly statuses = signal<DentistStatus[]>([]);
    readonly histories = signal<DentistHistory[]>([]);
    readonly contracts = signal<DentistContractRow[]>([]);
    readonly taxTypes = signal<TaxType[]>([]);
    readonly taxClassifications = signal<TaxClassification[]>([]);
    readonly dentistClinics = signal<DentistClinicWithNames[]>([]);
    readonly exclusiveToHmos = signal<string[]>([]);
    readonly exceptForHmos = signal<string[]>([]);
    readonly exclusiveToCompanies = signal<string[]>([]);
    readonly exceptForCompanies = signal<string[]>([]);

    // ---- Form
    readonly form: FormGroup = this.fb.group({
        // Top part
        last_name: ['', [Validators.required, Validators.maxLength(120)]],
        email: ['', [Validators.email, Validators.maxLength(200)]],

        given_name: ['', [Validators.required, Validators.maxLength(120)]],
        middle_name: [null as string | null, [Validators.maxLength(120)]],

        dentist_history_id: [null as number | null],
        dentist_status_id: [null as number | null],
        retainer_fee: [0, [Validators.required, Validators.min(0)]],

        // Tabs: Accreditation
        accre_dentist_contract_id: [null as number | null],
        accre_document_code: [null as string | null, [Validators.maxLength(120)]],
        accreditation_date: [null as Date | null],
        accre_contract_sent_date: [null as Date | null],

        // Tabs: Accounting
        acc_tin: [null as string | null, [Validators.maxLength(60)]],
        acc_bank_name: [null as string | null, [Validators.maxLength(120)]],
        acc_account_name: [null as string | null, [Validators.maxLength(120)]],
        acc_account_number: [null as string | null, [Validators.maxLength(60)]],
        acc_tax_type_id: [null as number | null],
        acc_tax_classification_id: [null as number | null],
    });

    // Keep an initial snapshot for "dirty" comparison if you want to warn on leave later
    private initialSnapshot: any = null;
    readonly isDirty = computed(() => {
        if (!this.loaded()) return false;
        return JSON.stringify(this.initialSnapshot) !== JSON.stringify(this.form.getRawValue());
    });

    dentist_clinic_columns: TableColumn[] = [
        {key: 'clinic_name', label: 'Clinic Name'},
        {key: 'position', label: 'Position'},
        {key: 'schedule', label: 'Schedule'},
    ];

    constructor() {
        // Load lookups immediately
        this.loadLookups();

        // Load dentist if route has :id
        effect(() => {
            const idParam = this.route.snapshot.paramMap.get('id');
            const id = idParam ? Number(idParam) : null;

            if (!id) {
                // Create mode
                this.dentistId.set(null);
                this.loaded.set(true);
                this.initialSnapshot = this.form.getRawValue();
                return;
            }

            if (!Number.isFinite(id) || id <= 0) return;

            // Edit mode
            this.dentistId.set(id);
            this.fetchDentist(id);
        });
    }

    private loadLookups() {
        this.dentistLookupService.getAllDentistStatuses()
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe((rows: DentistStatus[]) => {
                console.log(`rows: ${rows}`);
                this.statuses.set(rows ?? []);
            });

        this.dentistLookupService.getAllDentistHistories()
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe((rows: LookupOption[]) => this.histories.set(rows ?? []));

        this.dentistContractsService.getAll()
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe((rows: DentistContractRow []) => this.contracts.set(rows ?? []));

        this.dentistLookupService.getAllTaxTypes()
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe((rows: TaxType[]) => this.taxTypes.set(rows ?? []));

        this.dentistLookupService.getAllTaxClassifications()
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe((rows: TaxClassification[]) => this.taxClassifications.set(rows ?? []));
    }

    private fetchDentist(id: number) {
        this.loading.set(true);
        this.loaded.set(false);

        this.dentistService.getDentistById(id)
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe({
                next: (d: DentistWithLookups) => {
                    // Patch form (map ISO string dates -> Date)
                    this.form.patchValue({
                        last_name: d.last_name ?? '',
                        email: d.email ?? null,

                        given_name: d.given_name ?? '',
                        middle_name: d.middle_name ?? null,

                        dentist_history_id: d.dentist_history_id ?? null,
                        dentist_status_id: d.dentist_status_id ?? null,
                        retainer_fee: d.retainer_fee ?? 0,

                        accre_dentist_contract_id: d.accre_dentist_contract_id ?? null,
                        accre_document_code: d.accre_document_code ?? null,
                        accreditation_date: d.accreditation_date ? new Date(d.accreditation_date) : null,
                        accre_contract_sent_date: d.accre_contract_sent_date ? new Date(d.accre_contract_sent_date) : null,

                        acc_tin: d.acc_tin ?? null,
                        acc_bank_name: d.acc_bank_name ?? null,
                        acc_account_name: d.acc_account_name ?? null,
                        acc_account_number: d.acc_account_number ?? null,
                        acc_tax_type_id: d.acc_tax_type_id ?? null,
                        acc_tax_classification_id: d.acc_tax_classification_id ?? null,
                    });

                    this.loaded.set(true);
                    this.initialSnapshot = this.form.getRawValue();
                    this.loading.set(false);
                },
                error: () => {
                    this.loading.set(false);
                    this.loaded.set(true);
                    // You can route away or show a toast/snackbar
                },
            });

        this.dentistClinicService.getClinicsForDentistId(id)
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe({
                next: (dentist_clinics: DentistClinicWithNames[]) => {
                    this.dentistClinics.set(dentist_clinics);
                },
                error: (error) => {
                    console.log(`error: ${error}`);
                }
            });

        this.dentistHMORelations.getExclusiveToHmos(id)
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe({
                next: (exclusiveToHmos: string[]) => {
                    this.exclusiveToHmos.set(exclusiveToHmos);
                },
                error: (error) => {
                    console.log(`error: ${error}`);
                }
            })

        this.dentistHMORelations.getExceptForHmos(id)
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe({
                next: (exceptForHmos: string[]) => {
                    this.exceptForHmos.set(exceptForHmos);
                },
            })
    }

    save() {
        if (this.form.invalid) {
            this.form.markAllAsTouched();
            return;
        }

        this.saving.set(true);

        const raw = this.form.getRawValue();

        // Convert Date -> ISO (or whatever your backend expects)
        const payload = {
            last_name: raw.last_name,
            given_name: raw.given_name,
            middle_name: raw.middle_name,
            email: raw.email,

            dentist_history_id: raw.dentist_history_id,
            dentist_status_id: raw.dentist_status_id,
            retainer_fee: Number(raw.retainer_fee ?? 0),

            accre_dentist_contract_id: raw.accre_dentist_contract_id,
            accre_document_code: raw.accre_document_code,
            accreditation_date: raw.accreditation_date ? new Date(raw.accreditation_date).toISOString() : null,
            accre_contract_sent_date: raw.accre_contract_sent_date ? new Date(raw.accre_contract_sent_date).toISOString() : null,

            acc_tin: raw.acc_tin,
            acc_bank_name: raw.acc_bank_name,
            acc_account_name: raw.acc_account_name,
            acc_account_number: raw.acc_account_number,
            acc_tax_type_id: raw.acc_tax_type_id,
            acc_tax_classification_id: raw.acc_tax_classification_id,
        };

        const id = this.dentistId();
        // //const req$ = id ? this.dentistService.patch(id, payload) : this.dentistService.create(payload);
        //
        // req$
        //     .pipe(takeUntilDestroyed(this.destroyRef))
        //     .subscribe({
        //         next: (saved: DentistWithLookups) => {
        //             this.saving.set(false);
        //             this.initialSnapshot = this.form.getRawValue();
        //
        //             // If create, navigate to edit route (optional)
        //             if (!id && saved?.id) {
        //                 this.router.navigate(['../', saved.id], { relativeTo: this.route });
        //             }
        //         },
        //         error: () => {
        //             this.saving.set(false);
        //         },
        //     });
    }

    cancel() {
        // Typical behavior: back to list
        this.router.navigate(['../'], {relativeTo: this.route});
    }

    // Helpers for showing the *_name conceptually
    lookupName(list: LookupOption[], id: number | null): string {
        if (!id) return '';
        return list.find(x => x.id === id)?.name ?? '';
    }


    openClinicInNewTab(clinicId: number | null) {
        if (!clinicId) return;
        const tree = this.router.createUrlTree(['/main/setup/dental-clinics/', clinicId]);
        const url = this.router.serializeUrl(tree);

        // If you use HashLocationStrategy, url already includes '#/...'
        // If you use PathLocationStrategy and you want absolute, see Option B.
        window.open(url, '_blank', 'noopener');
    }

    // onClickClinic responds to a click on  row in the clinic table.
    //  it basically opens a new tab with the clinic's detail page.'
    onClickClinic(row: DentistClinicWithNames) {
        // this.openClinicInNewTab(row.clinic_id);
    }

    async openNewClinicDialog() {
        let the_clinics: DentistOrClinicWithIdAndName[] = [];
        const res = await firstValueFrom(this.dentalClinicService.getDentalClinics());
        the_clinics = res.items.map(c=>({id:c.id, name:`${c.name}-(${c.address})`}));


        const data: AddClinicOrDentistDialogData = {
            mode: 'clinic',
            options: the_clinics
        };

        const ref = this.dialog.open<
            AddClinicOrDentistDialogComponent,
            AddClinicOrDentistDialogData,
            AddClinicOrDentistDialogResult | null>
        (AddClinicOrDentistDialogComponent,
            {
                width: '860px',
                maxWidth: '95vw',
                data
            });

        ref.afterClosed().subscribe(result => {
            if (!result) return;
            // result is AddClinicOrDentistDialogResult.
            // result.mode 'clinic' or 'dentist'; result.position, result.schedule, result.selected.id, result.selected.name
            console.log("result:", result);
        })

    }

    addExclusiveToHmo() {
    }

    removeExclusiveToHmo() {
    };

    addExceptForHmo() {
    }

    removeExceptForHmo() {
    }

    addExclusiveToCompany() {
    }

    removeExclusiveToCompany() {
    }

    addExceptForCompany() {
    }

    removeExceptForCompany() {
    }
}

