import {Component, DestroyRef, computed, effect, inject, signal, ViewChild, OnInit, AfterViewInit} from '@angular/core';
import {ActivatedRoute, Router} from '@angular/router';
import {CommonModule} from '@angular/common';
import {AbstractControl, FormBuilder, FormGroup, ReactiveFormsModule, Validators} from '@angular/forms';
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
import {DentistService, DentistWithLookups} from '../../../../api_services/dentist-service';
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
import {DentistHMORelationsService, HMOListItem} from '../../../../api_services/dentist-hmorelations-service';
import {MatDialog} from '@angular/material/dialog';
import {
    AddClinicOrDentistDialogComponent,
    AddClinicOrDentistDialogData,
    AddClinicOrDentistDialogResult
} from '../../add-clinic-or-dentist-dialog-component/add-clinic-or-dentist-dialog-component';
import {firstValueFrom, } from 'rxjs';
import {DentalClinicService} from '../../../../api_services/dental-clinic-service';
import {
    ListDialogComponent,
    ListDialogData,
    ListDialogResult
} from '../../list-dialog-component/list-dialog-component';
import {HMOService} from '../../../../api_services/hmoservice';
import {HMORelationsComponent} from '../../hmorelations-component/hmorelations-component';
import {
    SingleDocumentSlotComponent
} from '../../../../components/single-document-slot-component/single-document-slot-component';
import {StoredDocumentMeta} from '../../../../api_services/single-document-upload-service';
import {AccountType, AccountTypeService} from '../../../../api_services/account-type-service';
import { toSignal } from '@angular/core/rxjs-interop'
import {startWith} from 'rxjs/operators';
import {
    DentistClinicPosition,
    DentistClinicPositionService
} from '../../../../api_services/dentist-clinic-position-service';

interface CompanyListItem {
    id: number;
    short_name: string;
}
/** Matches your API response shape */

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
        HMORelationsComponent,
        SingleDocumentSlotComponent
    ],
    templateUrl: './dentist-component.html',
    styleUrls: ['./dentist-component.scss'],
})
export class DentistComponent implements OnInit, AfterViewInit {
    private readonly fb = inject(FormBuilder);
    private readonly route = inject(ActivatedRoute);
    private readonly router = inject(Router);
    private readonly destroyRef = inject(DestroyRef);
    private readonly dentistService = inject(DentistService);
    private readonly dentistContractsService = inject(DentistContractsService);
    private readonly dentistLookupService = inject(DentistLookupsService);
    private readonly dentistClinicService = inject(DentistClinicService);
    private readonly dentistHMORelationsService = inject(DentistHMORelationsService)
    private readonly dentalClinicService = inject(DentalClinicService);
    private readonly hmoService = inject(HMOService);
    readonly dialog = inject(MatDialog);
    private readonly accountTypeService = inject(AccountTypeService);
    private readonly dentistClinicPositionService = inject(DentistClinicPositionService);

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
    readonly accountTypes = signal<AccountType[]>([]);
    readonly dentistClinics = signal<DentistClinicWithNames[]>([]);
    readonly exclusiveToHmos = signal<HMOListItem[]>([]);
    readonly exceptForHmos = signal<HMOListItem[]>([]);
    readonly exclusiveToCompanies = signal<CompanyListItem[]>([]);
    readonly exceptForCompanies = signal<CompanyListItem[]>([]);
    readonly dentistClinicPositions = signal<DentistClinicPosition[]>([]);


    // SingleDocumentSlotComponent For Accreditation Contract
    @ViewChild('documentSlot') documentSlot!: SingleDocumentSlotComponent;
    docPending = signal(false);
    accreditationContractFilename = signal('');



    // ---- Form
    readonly form: FormGroup = this.fb.group({
        // Top part
        prc_no: [null as string | null, [Validators.maxLength(120)]],
        prc_expiry_date: [null as Date | null],
        last_name: ['', [Validators.required, Validators.maxLength(120)]],
        email: ['', [Validators.email, Validators.maxLength(200)]],
        notes: [null as string |null, [Validators.maxLength(10_000)]],

        given_name: ['', [Validators.required, Validators.maxLength(120)]],
        middle_name: [null as string | null, [Validators.maxLength(120)]],

        dentist_history_id: [null as number | null],
        dentist_requested_by: [null as string | null, [Validators.maxLength(200)]],
        dentist_status_id: [null as number | null],
        dentist_decline_remarks: [null as string | null],
        retainer_fee: [0, [Validators.required, Validators.min(0)]],

        // Tabs: Accreditation
        accre_dentist_contract_id: [null as number | null],
        accre_document_code: [null as string | null, [Validators.maxLength(120)]],
        accreditation_date: [null as Date | null],
        accre_contract_sent_date: [null as Date | null],
        accre_contract_file_path: [null as string | null],

        // Tabs: Accounting
        acc_tin: [null as string | null, [Validators.maxLength(60)]],
        acc_bank_name: [null as string | null, [Validators.maxLength(120)]],
        acc_account_type_id: [null as number | null],
        acc_account_name: [null as string | null, [Validators.maxLength(120)]],
        acc_account_number: [null as string | null, [Validators.maxLength(60)]],
        acc_tax_type_id: [null as number | null],
        acc_tax_classification_id: [null as number | null],
    });

    private dentistHistoryId = toSignal(
        this.form.get('dentist_history_id')!.valueChanges.pipe(
            startWith(this.form.get('dentist_history_id')!.value)
        ),
        {initialValue: this.form.get('dentist_history_id')!.value}
    );
    private dentistStatusId = toSignal(
        this.form.get('dentist_status_id')!.valueChanges.pipe(
            startWith(this.form.get('dentist_status_id')!.value)
        ),
        { initialValue: this.form.get('dentist_status_id')!.value }
    );

    readonly showRequestedBy = computed(() => Number(this.dentistHistoryId()) === 2);

    readonly declineRemarkOptions = [
        { value: 'Declined by DNC', name: 'Declined by DNC' },
        { value: 'Dentist Declined', name: 'Dentist Declined' },
    ] as const;

// ✅ ADD THIS: show field only when status is "Non-accredited"
    readonly showDeclineRemarks = computed(() => {
        const statusId = Number(this.dentistStatusId() ?? 0);
        if (!statusId) return false;

        const statusName = this.statuses().find(s => s.id === statusId)?.name ?? '';
        console.log('statusId:', statusId, 'statusName:', statusName);

        return statusName.trim().toLowerCase() === 'non-accredited';
    });


    // Keep an initial snapshot for "dirty" comparison if you want to warn on leave later
    private initialSnapshot: any = null;
    readonly isDirty = computed(() => {
        if (!this.loaded()) return false;
        return JSON.stringify(this.initialSnapshot) !== JSON.stringify(this.form.getRawValue());
    });

    dentist_clinic_columns: TableColumn[] = [
        {key: 'clinic_name', label: 'Clinic Name'},
        {key: 'position_name', label: 'Position'},
        {key: 'schedule', label: 'Schedule'},
    ];

    constructor() {
        // Load lookups immediately
        this.loadLookups();

        // Load dentist if route has :id
        effect(() => {
            const idParam = this.route.snapshot.paramMap.get('id');
            const id = idParam ? Number(idParam) : null;
            console.log(`idParam: ${idParam}, id: ${id}`);

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

        effect(() => {
            //When history_id === 2, require dentist_requested_by; otherwise clear it
            const mustShow = this.showRequestedBy();
            console.log(` in constructor(), showRequestedBy: ${this.showRequestedBy()}`)
            console.log(`mustShow: ${mustShow}`);
            const ctrl = this.form.get('dentist_requested_by') as AbstractControl | null;
            if (!ctrl) return;
            if (mustShow){
                ctrl.setValidators([Validators.maxLength(200)]);
            } else{
                ctrl.clearValidators();
                ctrl.setValue(null, {emitEvent: false});
            }
            ctrl.updateValueAndValidity({emitEvent: false});
        });

        effect(() => {
            const mustShow = this.showDeclineRemarks();
            const ctrl = this.form.get('dentist_decline_remarks') as AbstractControl | null;
            if (!ctrl) return;

            if (!mustShow) {
                ctrl.setValue(null, { emitEvent: false });
            }
        });


    }

    ngOnInit(): void {
    }

    ngAfterViewInit(): void {
        this.documentSlot.refresh();
    }

    private loadLookups() {

        this.dentistLookupService.getAllDentistStatuses()
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe((rows: DentistStatus[]) => {
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

        this.accountTypeService.getAllAccountTypes()
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe((rows: AccountType[]) => this.accountTypes.set(rows ?? []));

        this.dentistClinicPositionService.getAllPositions()
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe((rows: DentistClinicPosition[]) => this.dentistClinicPositions.set(rows ?? []));
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
                        prc_no: d.prc_no ?? null,
                        prc_expiry_date: d.prc_expiry_date ? new Date(d.prc_expiry_date) : null,
                        last_name: d.last_name ?? '',
                        given_name: d.given_name ?? '',
                        middle_name: d.middle_name ?? null,
                        email: d.email ?? null,
                        notes: d.notes ?? null,

                        dentist_history_id: d.dentist_history_id ?? null,
                        dentist_requested_by: d.dentist_requested_by ?? null,
                        dentist_status_id: d.dentist_status_id ?? null,
                        dentist_decline_remarks: d.dentist_decline_remarks ?? null,
                        retainer_fee: d.retainer_fee ?? 0,

                        accre_dentist_contract_id: d.accre_dentist_contract_id ?? null,
                        accre_document_code: d.accre_document_code ?? null,
                        accreditation_date: d.accreditation_date ? new Date(d.accreditation_date) : null,
                        accre_contract_sent_date: d.accre_contract_sent_date ? new Date(d.accre_contract_sent_date) : null,
                        accre_contract_file_path: d.accre_contract_file_path ?? null,

                        acc_tin: d.acc_tin ?? null,
                        acc_bank_name: d.acc_bank_name ?? null,
                        acc_account_type_id: d.acc_account_type_id ?? null,
                        acc_account_name: d.acc_account_name ?? null,
                        acc_account_number: d.acc_account_number ?? null,
                        acc_tax_type_id: d.acc_tax_type_id ?? null,
                        acc_tax_classification_id: d.acc_tax_classification_id ?? null,
                    });
                    this.accreditationContractFilename.set(d.accre_contract_file_path ?? '');

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

        this.fetchClinics(id);

        this.fetchExclusiveToHmos(id);
        this.fetchExceptForHmos(id);
    }


    private fetchExclusiveToHmos(id: number) {
        this.dentistHMORelationsService.getExclusiveToHmos(id)
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe({
                next: (exclusiveToHmos: HMOListItem[]) => {
                    this.exclusiveToHmos.set(exclusiveToHmos);
                },
                error: (error) => {
                    console.log(`error: ${error}`);
                }
            })
    }
    private fetchExceptForHmos(id: number) {
        this.dentistHMORelationsService.getExceptForHmos(id)
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe({
                next: (exceptForHmos: HMOListItem[]) => {
                    this.exceptForHmos.set(exceptForHmos);
                },
                error: (error) => {
                    console.log(`error: ${error}`);
                }
            })
    }

    private fetchClinics(id: number) {
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

    }
    private toDateOnly(value: unknown): string | null {
        if (!value) return null;
        const d = value instanceof Date ? value : new Date(value as any);
        if (!Number.isFinite(d.getTime())) return null;

        // local date -> YYYY-MM-DD
        const y = d.getFullYear();
        const m = String(d.getMonth() + 1).padStart(2, '0');
        const day = String(d.getDate()).padStart(2, '0');
        return `${y}-${m}-${day}`;
    }


    save() {
        if (this.form.invalid) {
            this.form.markAllAsTouched();
            return;
        }

        this.saving.set(true);

        // Let's save the file first.
        // this.documentSlot.commitPendingUpload();

        const raw = this.form.getRawValue();

        // Convert Date -> ISO (or whatever your backend expects)
        const payload = {
            prc_no: raw.prc_no,
            prc_expiry_date: this.toDateOnly(raw.prc_expiry_date),
            last_name: raw.last_name,
            given_name: raw.given_name,
            middle_name: raw.middle_name,
            email: raw.email,
            notes: raw.notes,

            dentist_history_id: raw.dentist_history_id,
            dentist_requested_by: raw.dentist_requested_by,
            dentist_status_id: raw.dentist_status_id,
            dentist_decline_remarks: raw.dentist_decline_remarks,
            retainer_fee: Number(raw.retainer_fee ?? 0),

            accre_dentist_contract_id: raw.accre_dentist_contract_id,
            accre_document_code: raw.accre_document_code,
            accreditation_date: raw.accreditation_date ? new Date(raw.accreditation_date).toISOString() : null,
            accre_contract_sent_date: raw.accre_contract_sent_date ? new Date(raw.accre_contract_sent_date).toISOString() : null,

            acc_tin: raw.acc_tin,
            acc_bank_name: raw.acc_bank_name,
            acc_account_type_id: raw.acc_account_type_id,
            acc_account_name: raw.acc_account_name,
            acc_account_number: raw.acc_account_number,
            acc_tax_type_id: raw.acc_tax_type_id,
            acc_tax_classification_id: raw.acc_tax_classification_id,
        };

        const id = this.dentistId();
        const req$ = id ? this.dentistService.patchDentist(id, payload) : this.dentistService.createDentist(payload);

        req$
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe({
                next: (saved: DentistWithLookups) => {
                    this.saving.set(false);
                    this.initialSnapshot = this.form.getRawValue();

                    this.documentSlot.commitPendingUpload();

                    // If create, navigate to edit route (optional)
                    if (!id && saved?.id) {
                        this.router.navigate(['../', saved.id], { relativeTo: this.route }).then();
                    }
                },
                error: () => {
                    this.saving.set(false);
                },
            });
    }

    onAccreditationContractUploaded(meta: StoredDocumentMeta) {
        console.log(`onAccreditationContractUploaded: ${meta}`);
        this.form.patchValue({
            accre_contract_file_path: meta.file_name,
        })
         this.dentistService.patchDentist(<number>this.dentistId(), {accre_contract_file_path: meta.file_name})
             .pipe(takeUntilDestroyed(this.destroyRef))
             .subscribe({
                 next: ()=>{
                     console.log(`patching${meta.file_name}} successful`);
                 },
                 error: ()=>{
                     console.log(`patching failure.`);
                 }
             })
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

    async openNewClinicDialog() {
        let the_clinics: DentistOrClinicWithIdAndName[] = [];
        const res = await firstValueFrom(this.dentalClinicService.getDentalClinics());
        the_clinics = res.items.map(c=>({id:c.id, name:`${c.name}-(${c.address})`}));


        const data: AddClinicOrDentistDialogData = {
            mode: 'clinic',
            options: the_clinics,
            positions: this.dentistClinicPositions()
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
            this.dentistClinicService.addDentistClinic(result.selected.id,<number>this.dentistId(), result.position_id, result.schedule)
                .pipe(takeUntilDestroyed(this.destroyRef))
                .subscribe({
                    next: ()=>{
                        console.log(`clinic added successfully`);
                        this.fetchClinics(<number>this.dentistId());
                    },
                    error: ()=>{
                        console.log(`clinic add failed`);
                    }
                })
        })

    }

    async onDeleteClinic(event:any){
        console.log("onDeleteClinic:", event);
         this.dentistClinicService.removeDentistClinic(event.clinic_id, <number>this.dentistId())
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe({
                next: ()=>{
                    console.log(`clinic removed successfully`);
                    this.fetchClinics(<number>this.dentistId());
                },
                error: ()=>{
                    console.log(`clinic remove failed`);
                }
            });
    }

    async addExclusiveToHmo() {
        const res = await firstValueFrom(this.hmoService.getHMOs());
        const hmo_list = res.items.map((c)=>({id:c.id, label: c.short_name}));

        const data:ListDialogData = {
            title: 'Select Exclusive to HMO',
            subtitle: 'Select an HMO',
            enableGroupFilter:true,
            items: hmo_list
        }
        const ref = this.dialog.open<ListDialogComponent, ListDialogData, ListDialogResult |null> (
            ListDialogComponent,
            {data, width: '720px', maxWidth: '92vw' }
        );
        ref.afterClosed().subscribe((res)=>{
            if (!res) return;
            console.log('Selected:', res);
            this.dentistHMORelationsService.addExclusiveToHmos(
                <number>this.dentistId(),
                res.selectedId
            ).subscribe({
                next: ()=>{
                    console.log(`HMO added successfully`);
                    this.fetchExclusiveToHmos(<number>this.dentistId());
                }
            })
        })
    }

    removeExclusiveToHmo(event:any) {
        console.log("removeExclusiveToHmo:", event.selected.id);
        this.dentistHMORelationsService.removeExclusiveToHmos(<number>this.dentistId(), event.selected.id)
            .subscribe({
                next: ()=>{
                    console.log(`HMO removed successfully`);
                    this.fetchExclusiveToHmos(<number>this.dentistId());
                },
                error: ()=>{
                    console.log(`HMO remove failed`);
                }
            })

    };

    async addExceptForHmo() {
        const res = await firstValueFrom(this.hmoService.getHMOs());
        const hmo_list = res.items.map((c)=>({id:c.id, label: c.short_name}));

        const data:ListDialogData = {
            title: 'Select Except for HMO',
            subtitle: 'Select an HMO',
            enableGroupFilter:true,
            items: hmo_list
        }
        const ref = this.dialog.open<ListDialogComponent, ListDialogData, ListDialogResult |null> (
            ListDialogComponent,
            {data, width: '720px', maxWidth: '92vw' }
        );
        ref.afterClosed().subscribe((res)=>{
            if (!res) return;
            console.log('Selected:', res);
            this.dentistHMORelationsService.addExceptForHmos(
                <number>this.dentistId(),
                res.selectedId
            ).subscribe({
                next: ()=>{
                    console.log(`HMO added successfully`);
                    this.fetchExceptForHmos(<number>this.dentistId());
                }
            })
        })
    }

    removeExceptForHmo(event:any) {
        console.log("removeExceptForHmo:", event.selected.id);
        this.dentistHMORelationsService.removeExceptForHmos(<number>this.dentistId(), event.selected.id)
            .subscribe({
                next: ()=>{
                    console.log(`HMO removed successfully`);
                    this.fetchExceptForHmos(<number>this.dentistId());
                },
                error: ()=>{
                    console.log(`HMO remove failed`);
                }
            })
    }

    addExclusiveToCompany() {
    }

    removeExclusiveToCompany(selected:any) {
    }

    addExceptForCompany() {
    }

    removeExceptForCompany(selected:any) {
    }

    async addClinic(){
        let the_clinics: DentistOrClinicWithIdAndName[] = [];
        const res = await firstValueFrom(this.dentalClinicService.getDentalClinics());
        the_clinics = res.items.map(c=>({id:c.id, name:`${c.name}-(${c.address})`}));


        const data: AddClinicOrDentistDialogData = {
            mode: 'clinic',
            options: the_clinics,
            positions: this.dentistClinicPositions(),
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
            this.dentistClinicService.addDentistClinic(result.selected.id,<number>this.dentistId(), result.position_id, result.schedule)
                .pipe(takeUntilDestroyed(this.destroyRef))
                .subscribe({
                    next: ()=>{
                        console.log(`clinic added successfully`);
                        this.fetchClinics(<number>this.dentistId());
                    },
                    error: ()=>{
                        console.log(`clinic add failed`);
                    }
                })
        })
    }
}

