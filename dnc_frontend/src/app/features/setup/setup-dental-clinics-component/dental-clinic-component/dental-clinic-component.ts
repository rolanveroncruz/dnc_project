import {Component, DestroyRef, computed, effect, inject, signal, OnInit} from '@angular/core'; // CHANGED: added `effect`
import {ActivatedRoute, Router} from '@angular/router';
import {FormControl, FormGroup, ReactiveFormsModule, Validators} from '@angular/forms';
import {takeUntilDestroyed} from '@angular/core/rxjs-interop';

import {MatCard, MatCardContent, MatCardHeader, MatCardSubtitle, MatCardTitle} from '@angular/material/card';
import {MatFormField, MatLabel} from '@angular/material/form-field';
import {MatInput} from '@angular/material/input';
import {MatSelect, MatOption} from '@angular/material/select';
import {MatSlideToggle} from '@angular/material/slide-toggle';
import {MatButton} from '@angular/material/button';
import {MatProgressBar} from '@angular/material/progress-bar';

import {
    DentalClinicService,
    DentalClinicRowDb,                 // CHANGED: use service types
    DentalClinicModel,
    CreateDentalClinicBody,       // CHANGED
    PatchDentalClinicBody         // CHANGED
} from '../../../../api_services/dental-clinic-service';

import {RegionService} from '../../../../api_services/region-service';
import {Province, ProvincesService} from '../../../../api_services/provinces-service';
import {CityService} from '../../../../api_services/city-service';
import {ClinicCapabilitiesService, ClinicCapability} from '../../../../api_services/clinic-capabilities-service';
import {MatCheckbox} from '@angular/material/checkbox';
import {MatDivider} from '@angular/material/list';
import {
    ClinicCapabilitiesListService,
    ClinicCapabilityLinkRow
} from '../../../../api_services/clinic-capabilities-list-service';
import {DentistClinicService, DentistClinicWithNames} from '../../../../api_services/dentist-clinic-service';
import {
    DataTableWithSelectComponent
} from '../../../../components/data-table-with-select-component/data-table-with-select-component';
import {TableColumn} from '../../../../components/generic-data-table-component/table-interfaces';
import {firstValueFrom} from 'rxjs';
import {
    AddClinicOrDentistDialogComponent,
    AddClinicOrDentistDialogData, AddClinicOrDentistDialogResult
} from '../../add-clinic-or-dentist-dialog-component/add-clinic-or-dentist-dialog-component';
import {DentistOrClinicWithIdAndName} from "../../setup-dentists/dentist-component/dentist-component";
import {MatDialog} from '@angular/material/dialog';
import {DentistService} from '../../../../api_services/dentist-service';
import {
    DentistClinicPositionService,
    DentistClinicPosition
} from '../../../../api_services/dentist-clinic-position-service';
import {MatTab, MatTabGroup} from '@angular/material/tabs';
import {AccountType, AccountTypeService} from '../../../../api_services/account-type-service';
import {DentistLookupsService, TaxClassification, TaxType} from '../../../../api_services/dentist-lookups-service';


type LoadState = 'loading' | 'loaded' | 'error';

type RegionRow = { id: number; name: string };
type CityRow = { id: number; name: string; province_id?: number | null };

@Component({
    selector: 'app-dental-clinic',
    standalone: true,
    imports: [
        ReactiveFormsModule,
        MatCard, MatCardHeader, MatCardContent, MatCardTitle, MatCardSubtitle,
        MatFormField, MatLabel, MatInput,
        MatSelect, MatOption,
        MatSlideToggle,
        MatButton,
        MatProgressBar, MatCheckbox, MatDivider, DataTableWithSelectComponent, MatTabGroup, MatTab,
    ],
    templateUrl: './dental-clinic-component.html',
    styleUrl: './dental-clinic-component.scss',
})
export class DentalClinicComponent implements OnInit {
    private readonly route = inject(ActivatedRoute);
    private readonly router = inject(Router);
    private readonly destroyRef = inject(DestroyRef);

    private readonly dentalClinicService = inject(DentalClinicService);
    private readonly dentistClinicService = inject(DentistClinicService);
    private readonly regionsService = inject(RegionService);
    private readonly provincesService = inject(ProvincesService);
    private readonly citiesService = inject(CityService);
    private readonly clinicCapabilitiesService = inject(ClinicCapabilitiesService);
    private readonly clinicCapabilitiesListService = inject(ClinicCapabilitiesListService);
    private readonly dentistService = inject(DentistService);
    private readonly dentistClinicPositionService = inject(DentistClinicPositionService);
    private readonly accountTypesService = inject(AccountTypeService);
    private readonly TaxLookupService = inject(DentistLookupsService);

    private readonly dialog = inject(MatDialog);
    readonly loadState = signal<LoadState>('loading');
    // ✅ UNSAVED CHANGES INDICATOR
    // True when the form has been modified since last load/save.
    readonly hasUnsavedChanges = signal<boolean>(false);


    readonly isCreate = signal<boolean>(false);
    readonly clinicId = signal<number | null>(null);

    readonly pageTitle = computed(() => this.isCreate() ? 'New Dental Clinic' : 'Dental Clinic Details');
    readonly pageSubtitle = computed(() => this.isCreate() ? 'Create a new clinic record' : `Clinic ID: ${this.clinicId()}`);

    readonly regions = signal<RegionRow[]>([]);
    readonly provinces = signal<Province[]>([]);
    readonly cities = signal<CityRow[]>([]);
    readonly dentistClinicPositions = signal<DentistClinicPosition[]>([]);

    readonly accountTypes = signal<AccountType[]>([]);
    readonly taxTypes = signal<TaxType[]>([]);
    readonly taxClassifications = signal<TaxClassification[]>([]);


    readonly selectedRegionId = signal<number | null>(null);
    readonly selectedProvinceId = signal<number | null>(null);

    readonly filteredProvinces = computed(() => {
        const rid = this.selectedRegionId();
        const all = this.provinces();
        if (!rid) return all;
        return all.filter(s => (s.region_id ?? null) === rid);
    });

    readonly filteredCities = computed(() => {
        const sid = this.selectedProvinceId();
        const all = this.cities();
        if (!sid) return all;
        return all.filter(c => (c.province_id ?? null) === sid);
    });

    readonly clinicCapabilities = signal<ClinicCapability[] | null>(null);
    readonly clinicCapabilitiesList = signal<ClinicCapabilityLinkRow[] | null>(null);
    readonly dentistClinics = signal<DentistClinicWithNames[]>([]);

    // CHANGED: keep the last API-loaded city_id so we can derive region/state once lookups arrive
    private readonly loadedCityIdFromApi = signal<number | null>(null);

    // CHANGED: form now matches DentalClinicService fields (adds zip_code/email/schedule)
    // NOTE: region_id/state_id remain UI helpers only (derived from city_id), NOT sent to backend.
    readonly form = new FormGroup({
        // Column 1:
        name: new FormControl<string>('', {nonNullable: true, validators: [Validators.required]}),
        owner_name: new FormControl<string>(''),
        address: new FormControl<string>('', {nonNullable: true, validators: [Validators.required]}),
        contact_numbers: new FormControl<string>(''),
        email: new FormControl<string>(''),         // CHANGED: added

        // Column 2:
        region_id: new FormControl<number | null>(null),
        province_id: new FormControl<number | null>(null),
        city_id: new FormControl<number | null>(null),
        zip_code: new FormControl<string>(''),      // CHANGED: added

        // Column 3:
        capability_ids: new FormControl<number[]>([], {nonNullable: true}),
        schedule: new FormControl<string>(''),      // CHANGED: added
        remarks: new FormControl<string>(''),
        active: new FormControl<boolean>(true, {nonNullable: true}),

        // Accounting
        tin: new FormControl<string>(''),
        tax_classification_id: new FormControl<number | null>(null),
        bank_name: new FormControl<string>(''),
        account_type_id: new FormControl<number | null>(null),
        account_name: new FormControl<string>(''),
        account_number: new FormControl<string>(''),
        tax_type_id: new FormControl<number | null>(null),
        trade_name: new FormControl<string>(''),
        taxpayer_name: new FormControl<string>(''),



    });

    dentist_clinic_columns: TableColumn[] = [
        {key: 'last_name', label: 'Last Name'},
        {key: 'given_name', label: 'Given Name'},
        {key: 'position_name', label: 'Position',},
        {key: 'schedule', label: 'Schedule'},
    ];
    private readonly initialFormValue = signal<ReturnType<typeof this.form.getRawValue> | null>(null)

    constructor() {

        this.form.controls.region_id.valueChanges
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe(v => {
                this.selectedRegionId.set(v ?? null);
                this.form.controls.province_id.setValue(null);
                this.form.controls.city_id.setValue(null);
                this.selectedProvinceId.set(null);
            });

        this.form.controls.province_id.valueChanges
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe(v => {
                this.selectedProvinceId.set(v ?? null);
                this.form.controls.city_id.setValue(null);
            });

        this.form.valueChanges
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe(() => this.hasUnsavedChanges.set(this.form.dirty));

        // CHANGED: whenever we have a loaded city_id AND lookups are present, derive region/state automatically
        effect(() => {
            const cityId = this.loadedCityIdFromApi();
            const cities = this.cities();
            const provinces = this.provinces();
            if (!cityId || cities.length === 0 || provinces.length === 0) return;

            this.applyRegionProvinceFromCityId(cityId);
        });
    }

    private captureInitialFormValue() {
        this.initialFormValue.set(this.form.getRawValue());
    }

    ngOnInit(): void {
        // ---- load capabilities (unchanged)
        this.clinicCapabilitiesService.getClinicCapabilities()
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe({
                next: (v) => this.clinicCapabilities.set(v.items),
                error: () => console.log("Error in clinic capabilities"),
            });

        // CHANGED: do route mode + initial loads here (not constructor)
        const idParam = this.route.snapshot.paramMap.get('id');

        if (!idParam || idParam === 'new') {
            this.isCreate.set(true);
            this.clinicId.set(null);
            this.loadLookups();

            // ✅ UNSAVED CHANGES INDICATOR
            // New form starts "clean" until the user edits something.
            this.form.reset({
                name: '',
                owner_name: '',
                address: '',
                region_id: null,
                province_id: null,
                city_id: null,
                zip_code: '',
                contact_numbers: '',
                email: '',
                schedule: '',
                remarks: '',
                active: true,
            });
            this.form.markAsPristine();
            this.hasUnsavedChanges.set(false);
            this.captureInitialFormValue();

            this.loadState.set('loaded'); // create mode: nothing else to load
            return;
        }

        const id = Number(idParam);
        if (!Number.isFinite(id)) {
            this.router.navigate(['/setup/dental-clinics']).then();
            return;
        }

        this.isCreate.set(false);
        this.clinicId.set(id);

        // Lookups + clinic
        this.loadLookups();
        this.route.paramMap
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe(pm => {
                const idParam = pm.get('id');

                if (!idParam || idParam === 'new') {
                    this.isCreate.set(true);
                    this.clinicId.set(null);
                    this.resetForCreate();
                    this.loadState.set('loaded');
                    return
                }
                const id = Number(idParam);
                if (!Number.isFinite(id)) {
                    this.router.navigate(['/main/setup/dental-clinics']).then();
                    return;
                }
                this.isCreate.set(false);
                this.clinicId.set(id);
                this.loadClinic(id);
            });
    }

    private resetForCreate() {
        this.form.reset({
            name: '',
            owner_name: '',
            address: '',
            region_id: null,
            province_id: null,
            city_id: null,
            zip_code: '',
            contact_numbers: '',
            email: '',
            schedule: '',
            remarks: '',
            active: true,
        }, {emitEvent: false});

        this.selectedRegionId.set(null);
        this.selectedProvinceId.set(null);
        this.loadedCityIdFromApi.set(null);

        this.form.markAsPristine();
        this.hasUnsavedChanges.set(false);
        this.captureInitialFormValue();
    }

    private loadLookups() {
        // Adjust to match your services’ return shapes
        this.regionsService.getRegions({})
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe({
                next: res => this.regions.set(res.items), error: () => {
                }
            });

        this.provincesService.getProvinces({})
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe({
                next: res => this.provinces.set(res.items), error: () => {
                }
            });

        this.citiesService.getCities({})
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe({
                next: res => {
                    this.cities.set(res.items);

                },
                error: () => {
                }
            });

        this.dentistClinicPositionService.getAllPositions()
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe({
                next: res => this.dentistClinicPositions.set(res)
            });
    }

    private loadClinic(id: number) {
        this.loadState.set('loading');

        // CHANGED: use DentalClinic type returned by DentalClinicService
        this.dentalClinicService.getDentalClinicById(id)
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe({
                next: (clinic: DentalClinicRowDb) => {
                    // CHANGED: API does NOT include region_id/state_id.
                    // We patch backend fields now, then derive region/state from city_id once lookups are available.
                    this.form.patchValue({
                        name: clinic.name ?? '',
                        owner_name: clinic.owner_name ?? '',
                        address: clinic.address ?? '',
                        city_id: clinic.city_id ?? null,

                        zip_code: clinic.zip_code ?? '',         // CHANGED
                        contact_numbers: clinic.contact_numbers ?? '',
                        email: clinic.email ?? '',               // CHANGED
                        schedule: clinic.schedule ?? '',         // CHANGED
                        remarks: clinic.remarks ?? '',
                        active: clinic.active ?? true,
                    });


                    // CHANGED: store city_id so effect() can derive region/state
                    this.loadedCityIdFromApi.set(clinic.city_id ?? null);

                    // UNSAVED CHANGES INDICATOR
                    this.form.markAsPristine();
                    this.hasUnsavedChanges.set(false);
                    this.captureInitialFormValue();
                    this.loadState.set('loaded');
                },
                error: () => this.loadState.set('error'),
            });

        this.clinicCapabilitiesListService.getForClinic(id)
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe({
                next: (cap_list: ClinicCapabilityLinkRow[]) => {
                    this.clinicCapabilitiesList.set(cap_list);
                    // Pull capability IDs from link rows
                    const capIds = cap_list.map(r => r.capability_id)
                    this.form.controls.capability_ids.setValue(capIds, {emitEvent: false});
                    //
                    this.form.markAsPristine();
                    this.hasUnsavedChanges.set(false);
                    this.captureInitialFormValue();
                },
                error: () => console.log("Error in clinic capabilities"),
            });

        this.fetchDentistsForClinicId(id);
    }

    private fetchDentistsForClinicId(id: number) {
        this.dentistClinicService.getDentistsForClinicId(id)
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe({
                next: (dentist_clinics: DentistClinicWithNames[]) => {
                    this.dentistClinics.set(dentist_clinics);
                },
                error: () => console.log("Error in dentist clinics"),
            })

    }

    // CHANGED: derive state_id + region_id from city_id using lookup tables
    private applyRegionProvinceFromCityId(cityId: number) {
        const city = this.cities().find(c => c.id === cityId);
        const provinceId = city?.province_id ?? null;

        const province = provinceId != null ? this.provinces().find(s => s.id === provinceId) : null;
        const regionId = province?.region_id ?? null;

        // Patch UI helpers
        this.form.controls.region_id.setValue(regionId, {emitEvent: false});
        this.form.controls.province_id.setValue(provinceId, {emitEvent: false});
        this.form.controls.city_id.setValue(cityId, {emitEvent: false});

        // Keep computed filters in sync
        this.selectedRegionId.set(regionId);
        this.selectedProvinceId.set(provinceId);
    }

    onBack() {
        this.router.navigate(['/main/setup/dental-clinics']).then();
    }

    onDiscardChanges() {
        const initial = this.initialFormValue();
        if (!initial) return;
        // 1.Reset form without triggering cascading valueChanges logic
        this.form.reset(initial, {emitEvent: false});
        // 2. Keep some signals in sync
        this.selectedRegionId.set(initial.region_id ?? null);
        this.selectedProvinceId.set(initial.province_id ?? null);

        // If a city exists, re-derive region/state from city_id
        if (initial.city_id != null) {
            this.applyRegionProvinceFromCityId(initial.city_id);
        } else {
            this.form.controls.region_id.setValue(initial.region_id ?? null, {emitEvent: false});
            this.form.controls.province_id.setValue(initial.province_id ?? null, {emitEvent: false});
            this.form.controls.city_id.setValue(initial.city_id ?? null, {emitEvent: false});
        }
        this.form.markAsPristine();
        this.hasUnsavedChanges.set(false);
    }

    // CHANGED: required by API; replace this with your real logged-in username later
    private getLastModifiedBy(): string {
        // TODO: replace with auth user (e.g., this.auth.userName()) or claim value
        return 'system';
    }

    // CHANGED: helper to convert empty string -> null (so PATCH can explicitly null nullable columns if the user clears them)
    private emptyToNull(v: string | null | undefined): string | null {
        const s = (v ?? '').trim();
        return s.length === 0 ? null : s;
    }

    onSave() {
        if (this.form.invalid) {
            this.form.markAllAsTouched();
            return;
        }

        const raw = this.form.getRawValue();
        this.loadState.set('loading');

        if (this.isCreate()) {
            // CHANGED: build CreateDentalClinicBody (DO NOT send region_id/state_id; API doesn’t accept them)
            const payload: CreateDentalClinicBody = {
                name: raw.name,
                address: raw.address,
                owner_name: this.emptyToNull(raw.owner_name),
                city_id: raw.city_id ?? null,

                zip_code: this.emptyToNull(raw.zip_code),
                remarks: this.emptyToNull(raw.remarks),
                contact_numbers: this.emptyToNull(raw.contact_numbers),
                email: this.emptyToNull(raw.email),
                schedule: this.emptyToNull(raw.schedule),
                active: raw.active,

                last_modified_by: this.getLastModifiedBy(), // REQUIRED by your API
            };

            this.dentalClinicService.createDentalClinic(payload)
                .pipe(takeUntilDestroyed(this.destroyRef))
                .subscribe({
                    next: (created: DentalClinicModel) => { // CHANGED: created is DentalClinic
                        this.loadState.set('loaded');
                        this.form.markAsPristine();
                        this.hasUnsavedChanges.set(false);
                        this.captureInitialFormValue();

                        const newId = created?.id;
                        if (newId) {
                            this.router.navigate(['/main/setup/dental-clinics', newId]).then();
                        } else {
                            this.onBack();
                        }
                    },
                    error: () => this.loadState.set('error'),
                });

        } else {
            // CHANGED: build PatchDentalClinicBody (DO NOT send region_id/state_id)
            // PATCH semantics:
            // - omit => don't change
            // - include null => set to null
            // Here we include fields based on the form; you can get fancier and only include changed fields later.
            const payload: PatchDentalClinicBody = {
                name: raw.name,
                address: raw.address,
                owner_name: this.emptyToNull(raw.owner_name),
                city_id: raw.city_id ?? null,

                zip_code: this.emptyToNull(raw.zip_code),
                remarks: this.emptyToNull(raw.remarks),
                contact_numbers: this.emptyToNull(raw.contact_numbers),
                email: this.emptyToNull(raw.email),
                schedule: this.emptyToNull(raw.schedule),
                active: raw.active,

                last_modified_by: this.getLastModifiedBy(), // REQUIRED by your API
            };

            this.dentalClinicService.patchDentalClinic(this.clinicId()!, payload)
                .pipe(takeUntilDestroyed(this.destroyRef))
                .subscribe({
                    next: () => {
                        this.form.markAsPristine();
                        this.hasUnsavedChanges.set(false);
                        this.captureInitialFormValue();

                        this.onBack()
                    },
                    error: () => this.loadState.set('error'),
                });
        }
    }

    onToggleCapability(capId: number, checked: boolean) {
        const ctrl = this.form.controls.capability_ids;
        const current = ctrl.value;

        const next = checked
            ? Array.from(new Set([...current, capId]))
            : current.filter(id => id !== capId);

        ctrl.markAsDirty();
        this.form.markAsDirty();
        ctrl.setValue(next);
        if (checked) {
            this.clinicCapabilitiesListService.addToClinic(this.clinicId()!, capId)
                .pipe(takeUntilDestroyed(this.destroyRef))
                .subscribe({
                    next: () => console.log("Added capability to clinic"),
                    error: () => console.log("Error in adding capability to clinic")
                })
        } else {
            this.clinicCapabilitiesListService.removeFromClinic(this.clinicId()!, capId)
                .pipe(takeUntilDestroyed(this.destroyRef))
                .subscribe({
                    next: () => console.log("Removed capability from clinic"),
                    error: () => console.log("Error in removing capability from clinic")
                })
        }
    }

    isCapabilityChecked(capId: number): boolean {
        return this.form.controls.capability_ids.value.includes(capId);
    }

    openDentistInNewTab(dentistId: number) {
        const tree = this.router.createUrlTree(['/main/setup/dentists', dentistId]);
        const url = this.router.serializeUrl(tree);

        // If you use HashLocationStrategy, url already includes '#/...'
        // If you use PathLocationStrategy and you want absolute, see Option B.
        window.open(url, '_blank', 'noopener');
    }

    onClickDentist(row: DentistClinicWithNames) {
        this.openDentistInNewTab(row.dentist_id);
    }

    rowId = (r: any) => r.id;
    selectedRow: any | null = null;


    async onAddDentist() {

        console.log("onAddDentist");
        let the_dentists: DentistOrClinicWithIdAndName[] = [];
        const res = await firstValueFrom(this.dentistService.getAllDentists());
        the_dentists = res.map(c => ({id: c.id, name: `${c.last_name}, ${c.given_name} ${c.middle_name} `}));


        const data: AddClinicOrDentistDialogData = {
            mode: 'dentist',
            options: the_dentists,
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
            console.log("result:", result);
            this.dentistClinicService.addDentistClinic(<number>this.clinicId(), result.selected.id, result.position_id, result.schedule)
                .pipe(takeUntilDestroyed(this.destroyRef))
                .subscribe({
                    next: () => {
                        this.fetchDentistsForClinicId(<number>this.clinicId());
                    },
                    error: () => console.log("Error in adding dentist clinic")
                })
        })
    }

    onDelete(row: any) {
        console.log("onDelete:", row);
        this.dentistClinicService.removeDentistClinic(<number>this.clinicId(), row.dentist_id)
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe({
                next: () => {
                    this.fetchDentistsForClinicId(<number>this.clinicId());
                },
                error: () => console.log("Error in removing dentist clinic")
            });

    }

    async addDentist() {
        console.log("onAddDentist");
        let the_dentists: DentistOrClinicWithIdAndName[] = [];
        const res = await firstValueFrom(this.dentistService.getAllDentists());
        the_dentists = res.map(c => ({id: c.id, name: `${c.last_name}, ${c.given_name} ${c.middle_name} `}));


        const data: AddClinicOrDentistDialogData = {
            mode: 'dentist',
            options: the_dentists,
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
            console.log("result:", result);
            this.dentistClinicService.addDentistClinic(<number>this.clinicId(), result.selected.id, result.position_id, result.schedule)
                .pipe(takeUntilDestroyed(this.destroyRef))
                .subscribe({
                    next: () => {
                        this.fetchDentistsForClinicId(<number>this.clinicId());
                    },
                    error: () => console.log("Error in adding dentist clinic")
                })
        })

    }
}
