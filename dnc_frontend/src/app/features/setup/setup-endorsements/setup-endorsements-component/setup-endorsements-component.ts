import { CommonModule } from '@angular/common';
import {Component, ChangeDetectionStrategy, DestroyRef, computed, inject, signal, OnInit, effect} from '@angular/core';
import { FormArray, FormControl, FormGroup, FormBuilder, ReactiveFormsModule, Validators } from '@angular/forms';
import {HMOService, HMOOptions} from '../../../../api_services/hmoservice';
import {
    EndorsementService,
    EndorsementTypeOptions,
    BillingFrequencyOptions,
    PatchEndorsementRequest,
    EndorsementRateResponse,
    EndorsementCountResponse,
} from '../../../../api_services/endorsement-service';
import {CurrencyInputComponent} from '../../../../components/currency-input-component/currency-input-component';
import {DentalServicesService,DentalService} from '../../../../api_services/dental-services-service';
import { MatCardModule } from '@angular/material/card';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatSelectModule } from '@angular/material/select';
import { MatRadioModule } from '@angular/material/radio';
import { MatTabsModule } from '@angular/material/tabs';
import { MatDatepickerModule } from '@angular/material/datepicker';
import { MatNativeDateModule } from '@angular/material/core';
import {takeUntilDestroyed, toSignal} from '@angular/core/rxjs-interop';
import {ActivatedRoute, Router} from '@angular/router';
import {MatChip} from '@angular/material/chips';
import {MatProgressBar} from '@angular/material/progress-bar';
import {MatButton} from '@angular/material/button';
import {
    AddableAutocompleteComponent, AddableAutocompleteItem
} from '../../../../components/addable-autocomplete-component/addable-autocomplete-component';
import {Observable, catchError, finalize, forkJoin, map as rxMap, of, switchMap, tap} from 'rxjs';
import {ExistingMasterListMeta} from './endorsement-master-list-upload-component/data-types';
import {
    EndorsementMasterListUploadComponent
} from './endorsement-master-list-upload-component/endorsement-master-list-upload-component';
import {SpecialServicesFeesTabComponent} from './special-services-fees-tab-component/special-services-fees-tab-component';
import {SpecialServicesCountsTabComponent} from './special-services-counts-tab-component/special-services-counts-tab-component';
import {HighEndServicesCountsTabComponent} from './high-end-services-counts-tab-component/high-end-services-counts-tab-component';
import {HttpErrorResponse} from '@angular/common/http';
import {
    EndorsementMasterListMemberResponse,
    EndorsementMasterListService
} from '../../../../api_services/endorsement-master-list-service';
import{ MasterListDialogComponent} from '../../../../components/master-list-dialog-component/master-list-dialog-component';
import {MatDialog} from '@angular/material/dialog';
import {BasicServicesFeesTabComponent} from './basic-services-fees-tab-component/basic-services-fees-tab-component';
import {
    BasicServicesCountsTabComponent
} from './basic-services-counts-tab-component/basic-services-counts-tab-component';

type UIState = 'idle' | 'loading' | 'saving' | 'error';
type RuleSectionKey =
    | 'basicServicesFees'
    | 'basicServicesCounts'
    | 'specialServicesFees'
    | 'specialServicesCounts'
    | 'highEndServicesCounts'
    | 'additionalBillingRules';
const ENDORSEMENT_TYPE_ID = {
    RetainerOnly: 1,
    RetainerWithSpecialServices: 2,
    RetainerAndFeePerService: 3,
} as const;
const RULES_MATRIX: Record<number, readonly RuleSectionKey[]> = {
    [ENDORSEMENT_TYPE_ID.RetainerOnly]: [
        'basicServicesCounts',
    ],
    [ENDORSEMENT_TYPE_ID.RetainerWithSpecialServices]: [
        'basicServicesCounts',
        'specialServicesCounts'
    ],
    [ENDORSEMENT_TYPE_ID.RetainerAndFeePerService]: [
        'basicServicesFees',
        'basicServicesCounts',
        'specialServicesFees',
        'specialServicesCounts',
        'highEndServicesCounts',
    ],
};


@Component({
    selector: 'app-setup-endorsements',
    standalone: true,
    changeDetection: ChangeDetectionStrategy.OnPush,
    imports: [
        CommonModule,
        ReactiveFormsModule,
        MatCardModule,
        MatFormFieldModule,
        MatInputModule,
        MatSelectModule,
        MatRadioModule,
        MatTabsModule,
        MatDatepickerModule,
        MatNativeDateModule,
        MatChip,
        MatProgressBar,
        MatButton,
        AddableAutocompleteComponent,
        CurrencyInputComponent,
        EndorsementMasterListUploadComponent,
        SpecialServicesFeesTabComponent,
        SpecialServicesCountsTabComponent,
        HighEndServicesCountsTabComponent,
        BasicServicesFeesTabComponent,
        BasicServicesCountsTabComponent,
    ],
    templateUrl: './setup-endorsements-component.html',
    styleUrls: ['./setup-endorsements-component.scss'],
})
export class SetupEndorsementsComponent implements OnInit{
    private readonly fb = inject(FormBuilder);
    private readonly route = inject(ActivatedRoute);
    private readonly router=inject(Router);
    private destroyRef = inject(DestroyRef);
    private readonly hmoService = inject(HMOService);
    private readonly endorsementService = inject(EndorsementService);
    private readonly endorsementMasterListService = inject(EndorsementMasterListService);
    private readonly dentalServicesService = inject(DentalServicesService);

    readonly loadState = signal<UIState>('idle');
    readonly endorsementId = signal<number | null>(null);
    readonly isEditMode = computed(()=>this.endorsementId() != null);

    private readonly baseline = signal<ReturnType<SetupEndorsementsComponent['snapshotForm']> | null>(null);

    // Replace these with API-driven options later.
    readonly hmoOptions = signal<HMOOptions[]>([]);
    readonly companyOptions = signal<AddableAutocompleteItem[]>([]);
    readonly endorsementTypeOptions = signal<EndorsementTypeOptions[]>([]);
    readonly billingFrequencyOptions = signal<BillingFrequencyOptions[]>([]);

    readonly isSavingCompany = signal(false);

    readonly masterListEnabled = computed(()=>!! this.selectedHMO()?.expect_a_master_list);
    readonly masterListMeta = signal<ExistingMasterListMeta | null>(null);

    readonly all_dental_services = signal<DentalService[]>([]);
    readonly basicServices = computed(() =>
        this.all_dental_services().filter(s => s.type_id=== 1 && s.active));
    readonly specialServices = computed(() =>
        this.all_dental_services().filter(s => s.type_id === 2 && s.active));
    readonly highEndServices = computed(() =>
    this.all_dental_services().filter(s => s.type_id === 3 && s.active));

    readonly dialog = inject(MatDialog);
    readonly masterListMembers = signal<EndorsementMasterListMemberResponse[]>([]);


    readonly form = this.fb.group({
        // Column 1
        hmo_id: [null as number|null, [Validators.required]],
        endorsement_company_id: [null as string|null, [Validators.required]],
        agreement_corp_number: ['', Validators.required],

        // Column 2
        endorsement_billing_period_type_id: [null as number|null, [Validators.required]],
        retainer_fee: [null as number | null, [Validators.min(0)]],
        date_start: [null as Date | null],
        date_end: [null as Date | null],

        // Column 3
        endorsement_type_id: [null as number| null, [Validators.required]],
        remarks: [''],
        endorsement_method: [null as string | null],

        // Tab sections
        basicServicesFees: this.fb.array<FormGroup>([]),
        basicServicesCounts: this.fb.array<FormGroup>([]),
        specialServicesFees: this.fb.array<FormGroup>([]),
        specialServicesCounts: this.fb.array<FormGroup>([]),
        highEndServicesCounts: this.fb.array<FormGroup>([]),

    });

    readonly hmoIdSig = toSignal( this.form.controls.hmo_id.valueChanges, {initialValue: this.form.controls.hmo_id.value});

    readonly selectedHMO = computed(()=>{
        const raw = this.hmoIdSig();
        const id = raw==null? null: Number(raw);
        if(!id || !Number.isFinite(id)) return null;
        return this.hmoOptions().find(x=>x.id==id) ?? null;
    });
    readonly endorsementTypeIdSig = toSignal(this.form.controls.endorsement_type_id.valueChanges,
        {initialValue: this.form.controls.endorsement_type_id.value});
    readonly enabledSections = computed<Set<RuleSectionKey>>( ()=>{
        const raw = this.endorsementTypeIdSig();
        const typeId = raw == null? null : Number(raw);
        if (!typeId || !Number.isFinite(typeId)) return new Set<RuleSectionKey>();

        const enabled = RULES_MATRIX[typeId] ?? [];
        return new Set(enabled);

    });
    readonly basicServicesFeesEnabled = computed(()=>this.enabledSections().has('basicServicesFees'))
    readonly basicServicesCountsEnabled = computed(()=>this.enabledSections().has('basicServicesCounts'))
    readonly specialServicesFeesEnabled = computed(()=>this.enabledSections().has('specialServicesFees'));
    readonly specialServicesCountsEnabled = computed(()=>this.enabledSections().has('specialServicesCounts'));
    readonly highEndServicesCountsEnabled = computed(()=>this.enabledSections().has('highEndServicesCounts'));
    readonly additionalBillingRulesEnabled = computed(()=>this.enabledSections().has('additionalBillingRules'));

    // for this endorsement, loadedRateRows contain the information of service, rate
    //  then ladedCountRows contain the information of service, count
    readonly loadedRateRows = signal<EndorsementRateResponse[]>([]);
    readonly loadedCountRows = signal<EndorsementCountResponse[]>([]);

    /*
    * API Helpers and Functions
     */
    private feeRow(
        serviceId:number,
        recordId:number|null=null,
        rate:number|null=null
    ){
        return this.fb.group({
            id: new FormControl<number|null> (recordId),
            dental_service_id: new FormControl<number>(serviceId, {nonNullable: true}),
            rate: new FormControl<number|null>(rate, [Validators.min(0)]),
        })
    };
    private countRow(
        serviceId:number,
        recordId:number|null=null,
        limit:number|null=null
    ){
        return this.fb.group({
            id: new FormControl<number|null> (recordId),
            dental_service_id: new FormControl<number>(serviceId, {nonNullable: true}),
            limit: new FormControl<number|null>(limit, [Validators.min(0)]),
        })
    };
// ✅ NEW: convenience getters
    get basicServicesFeesArr() {
        return this.form.controls.basicServicesFees as FormArray<FormGroup>;
    }
    get basicServicesCountsArr() {
        return this.form.controls.basicServicesCounts as FormArray<FormGroup>;
    }
    get specialServiceFeesArr() {
        return this.form.controls.specialServicesFees as FormArray<FormGroup>;
    }
    get specialServiceCountsArr() {
        return this.form.controls.specialServicesCounts as FormArray<FormGroup>;
    }
    get highEndServiceCountsArr() {
        return this.form.controls.highEndServicesCounts as FormArray<FormGroup>;
    }

    private rebuildRuleMatrices():void{
        /*
        rate/count Map are basically maps of service_id to rate/count.
         */
        const rateMap = new Map(this.loadedRateRows().map(r=>[r.dental_service_id, r] as const));
        const countMap = new Map(this.loadedCountRows().map(c=>[c.dental_service_id, c] as const));

        const basicServicesFeesRows = this.basicServices().map(s => {
            const existing = rateMap.get(s.id);
            return this.feeRow(
                s.id,
                existing?.id ?? null,
                existing?.rate != null ? Number(existing.rate) : null,
            );
        });
        this.form.setControl('basicServicesFees', this.fb.array(basicServicesFeesRows));

        const basicServicesCountRows = this.basicServices()
            .filter( s=> !s.is_unlimited)
            .map(s => {
            const existing = countMap.get(s.id);
            return this.countRow(
                s.id,
                existing?.id ?? null,
                existing?.count ?? null,
            );
        });
        this.form.setControl('basicServicesCounts', this.fb.array(basicServicesCountRows));

        const specialServicesFeesRows = this.specialServices().map(s=>{
            const existing = rateMap.get(s.id);
            return this.feeRow(
                s.id,
                existing?.id ?? null,
                existing?.rate != null? Number(existing.rate) : null,
            );
        });
        this.form.setControl('specialServicesFees', this.fb.array(specialServicesFeesRows));

        const specialServicesCountRows = this.specialServices().map(s=>{
            const existing = countMap.get(s.id);
            return this.countRow(
                s.id,
                existing?.id ?? null,
                existing?.count ?? null,
            );
        });
        this.form.setControl('specialServicesCounts', this.fb.array(specialServicesCountRows));

        const highEndCountRows = this.highEndServices().map(s=>{
            const existing = countMap.get(s.id);
            return this.countRow(
                s.id,
                existing?.id ?? null,
                existing?.count ?? null,
            );
        });
        this.form.setControl('highEndServicesCounts', this.fb.array(highEndCountRows));
    }


    private refreshCompanies(){
        return this.endorsementService.getEndorsementCompanies()
            .pipe(
                takeUntilDestroyed(this.destroyRef),
                rxMap((items)=>
                    items.map((x)=>({
                        id: String(x.id),
                        label:x.name
                    })))
            );
    }

    ngOnInit(): void {

        this.hmoService.getHMOOptions()
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe(dto => this.hmoOptions.set(dto));

        this.refreshCompanies()
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe(items => {
                this.companyOptions.set(items)
            });

        this.endorsementService.getEndorsementTypes()
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe( items => {
                this.endorsementTypeOptions.set(items)
            });

        this.endorsementService.getEndorsementBillingPeriodTypes()
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe( items => {
                this.billingFrequencyOptions.set(items)
            });
        this.dentalServicesService.getDentalServices()
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe( page=> {this.all_dental_services.set(page.items)})

        this.route.paramMap
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe((pm)=>{
                const id=this.readRouteId(pm.get('id'));
                this.endorsementId.set(id);

                if (id == null){
                    this.resetForNew();
                }else {
                    this.loadExisting(id);
                }

            })
    }
    private readonly _rebuildRuleMatricesEffect = effect(
        ()=> {
            const all = this.all_dental_services();
            if (!all.length) return;
            this.rebuildRuleMatrices();
        },
        {allowSignalWrites:true}
    );
    // -----------------------------
    // Route handling
    // -----------------------------

    // CHANGED:
    // - If param is "new" or missing => new record
    // - If numeric => edit record
    private readRouteId(raw: string | null): number | null {
        if (!raw) return null;
        if (raw.toLowerCase() === 'new') return null;
        const n = Number(raw);
        return Number.isFinite(n) && n > 0 ? n : null;
    }
    // -----------------------------
    // Loading / Resetting
    // -----------------------------

    // CHANGED
    private resetForNew(): void {
        this.loadedRateRows.set([]);
        this.loadedCountRows.set([]);
        this.masterListMeta.set(null);

        // Reset to defaults
        this.form.reset({
            hmo_id: null,
            endorsement_company_id: null,
            agreement_corp_number: '',

            endorsement_billing_period_type_id: null,
            retainer_fee: null,
            date_start: null,
            date_end: null,

            endorsement_type_id: null,
            remarks: '',
            endorsement_method: null,
        });

        this.rebuildRuleMatrices();
        // Mark pristine baseline
        const snap = this.snapshotForm();
        this.baseline.set(snap);

        // Make UI clean
        this.form.markAsPristine();
        this.form.markAsUntouched();
        this.loadState.set('idle');
    }

    /*
    loadExisting accepts an id, the endorsement_id, and pulls
    1. endorsement_information
    2. endorsement_rates
    3. endorsement_counts
    4. endorsement_master_list_meta
     */

    private loadExisting(id: number | null): void {
        if (id == null) return;

        this.loadState.set('loading');

        const parseIsoDate = (s: string | null | undefined): Date | null => {
            if (!s) return null;
            // Expect "YYYY-MM-DD"
            const [yy, mm, dd] = s.split('-').map(Number);
            if (!Number.isFinite(yy) || !Number.isFinite(mm) || !Number.isFinite(dd)) return null;
            // Use local date (avoids timezone shifting)
            return new Date(yy, mm - 1, dd);
        };

        forkJoin({
            dto: this.endorsementService.get_endorsement_by_id(id),
            rates: this.endorsementService.getEndorsementRates(id).pipe(
                catchError((err) => {
                    console.error('Failed to load endorsement rates:', err);
                    return of([] as EndorsementRateResponse[]);
                })
            ),
            counts: this.endorsementService.getEndorsementCounts(id).pipe(
                catchError((err) => {
                    console.error('Failed to load endorsement counts:', err);
                    return of([] as EndorsementCountResponse[]);
                })
            ),
            masterListMeta: this.endorsementMasterListService.getEndorsementMasterListMeta(id).pipe(
                catchError((err) => {
                    if (err instanceof HttpErrorResponse && err.status === 404){
                        console.error('Caught : Endorsement master list meta not found:', err);
                        return of(null);
                    }
                    console.error('Caught : Failed to load endorsement master list meta:', err);
                    return of(null);
                })
            ),
        })
            .pipe(
                tap(({dto, rates, counts, masterListMeta}) => {
                    this.loadedRateRows.set(rates);
                    this.loadedCountRows.set(counts);
                    this.masterListMeta.set(masterListMeta);

                    this.form.reset({
                        hmo_id: dto.hmo_id,
                        endorsement_company_id: String(dto.endorsement_company_id),
                        agreement_corp_number: dto.agreement_corp_number ?? '',

                        endorsement_billing_period_type_id: dto.endorsement_billing_period_type_id,
                        retainer_fee: dto.retainer_fee == null ? null: Number(dto.retainer_fee),
                        date_start: dto.date_start ? parseIsoDate(dto.date_start) : null,
                        date_end: dto.date_end ? parseIsoDate(dto.date_end) : null,
                        endorsement_type_id: dto.endorsement_type_id !=null? dto.endorsement_type_id : null,
                        remarks: dto.remarks ?? '',
                        endorsement_method: dto.endorsement_method ?? null,
                    });
                    this.rebuildRuleMatrices();
                    this.baseline.set(this.snapshotForm());
                    this.form.markAsPristine();
                    this.form.markAsUntouched();
                    this.loadState.set('idle');
                }),
                catchError((err)=>{
                    console.error('Failed to load endorsement:', err);
                    this.loadState.set('error');
                    return of(null);
                }),
                finalize(()=>{
                    if (this.loadState() === 'loading') this.loadState.set('idle');
                }),
                takeUntilDestroyed(this.destroyRef),
            ).subscribe();
    }
    private buildRateRequests(endorsementId: number): Observable<EndorsementRateResponse>[] { // 🔵 CHANGED
        if (!this.specialServicesFeesEnabled()) return [];

        return this.specialServiceFeesArr.controls
            .map(ctrl => ctrl.getRawValue())
            .filter(row => row.rate != null)
            .map(row => {
                const rate = Number(row.rate).toFixed(2);

                if (row.id != null) {
                    return this.endorsementService.updateEndorsementRatePatch(
                        endorsementId,
                        row.id,
                        { rate }
                    );
                }

                return this.endorsementService.createEndorsementRate(
                    endorsementId,
                    {
                        dental_service_id: Number(row.dental_service_id),
                        rate,
                    }
                );
            });
    }

    private buildCountRequests(endorsementId: number): Observable<EndorsementCountResponse>[] { // ✅🔵 ADDED
        const requests: Observable<EndorsementCountResponse>[] = [];

        if (this.specialServicesCountsEnabled()) {
            for (const ctrl of this.specialServiceCountsArr.controls) {
                const row = ctrl.getRawValue();
                if (row.limit == null) continue;

                if (row.id != null) {
                    requests.push(
                        this.endorsementService.updateEndorsementCountPatch(
                            endorsementId,
                            row.id,
                            { count: Number(row.limit) }
                        )
                    );
                } else {
                    requests.push(
                        this.endorsementService.createEndorsementCount(
                            endorsementId,
                            {
                                dental_service_id: Number(row.dental_service_id),
                                count: Number(row.limit),
                            }
                        )
                    );
                }
            }
        }

        if (this.highEndServicesCountsEnabled()) {
            for (const ctrl of this.highEndServiceCountsArr.controls) {
                const row = ctrl.getRawValue();
                if (row.limit == null) continue;

                if (row.id != null) {
                    requests.push(
                        this.endorsementService.updateEndorsementCountPatch(
                            endorsementId,
                            row.id,
                            { count: Number(row.limit) }
                        )
                    );
                } else {
                    requests.push(
                        this.endorsementService.createEndorsementCount(
                            endorsementId,
                            {
                                dental_service_id: Number(row.dental_service_id),
                                count: Number(row.limit),
                            }
                        )
                    );
                }
            }
        }

        return requests;
    }

    private saveRuleTabs(endorsementId: number): Observable<{
        rates: EndorsementRateResponse[];
        counts: EndorsementCountResponse[];
    }> { // 🔵 CHANGED
        const requests: Observable<unknown>[] = [ // 🔵 CHANGED
            ...this.buildRateRequests(endorsementId),
            ...this.buildCountRequests(endorsementId),
        ];

        const save$: Observable<unknown> = requests.length > 0 // 🔵 CHANGED
            ? forkJoin(requests)
            : of(null);

        return save$.pipe(
            switchMap(() =>
                forkJoin({
                    rates: this.endorsementService.getEndorsementRates(endorsementId),
                    counts: this.endorsementService.getEndorsementCounts(endorsementId),
                })
            ),
            tap(({ rates, counts }) => {
                this.loadedRateRows.set(rates);
                this.loadedCountRows.set(counts);
                this.rebuildRuleMatrices();
            })
        );
    }

    // CHANGED
    private snapshotForm() {
        const v = this.form.getRawValue();
        return {
            hmo_id: v.hmo_id,
            endorsement_company_id: v.endorsement_company_id,
            endorsement_type_id: v.endorsement_type_id,

            agreement_corp_number: v.agreement_corp_number ?? '',

            endorsement_billing_period_type_id: v.endorsement_billing_period_type_id,
            retainer_fee: v.retainer_fee ?? null,
            date_start: v.date_start ?? null,
            date_end: v.date_end ?? null,

            remarks: v.remarks ?? '',
            endorsement_method: v.endorsement_method ?? null,
            basicServicesFees: v.basicServicesFees ?? [],
            basicServicesCounts: v.basicServicesCounts ?? [],
            specialServicesFees: v.specialServicesFees ?? [],
            specialServicesCounts: v.specialServicesCounts ?? [],
            highEndServicesCounts: v.highEndServicesCounts ?? [],
        };
    }

    onCompanySelected(ev: AddableAutocompleteItem){
        this.form.controls.endorsement_company_id.setValue(ev.id);
        this.form.controls.endorsement_company_id.markAsDirty();
    }

    onCompanyCreated(ev: AddableAutocompleteItem){
        const name=(ev.label ?? '').trim();
        if (!name) return;
        this.isSavingCompany.set(true)
        this.endorsementService.createEndorsementCompany(name).pipe(
            switchMap((createdRow)=>
            this.refreshCompanies().pipe(
                tap((options)=>{
                    this.companyOptions.set(options);
                    this.form.controls.endorsement_company_id.setValue(String(createdRow.id));
                    this.form.controls.endorsement_company_id.markAsDirty();
                })
            )
        ),
            catchError((err)=> {
                console.error("Failed to create company:", err);
                return of(null);
            }),
            finalize(()=>this.isSavingCompany.set(false)),
            takeUntilDestroyed(this.destroyRef),
            ).subscribe();
    }

    hasUnsavedChanges():boolean {
        return this.baseline() != null && !this.form.pristine;
    }
    onSave(): void {
        console.log("Saving...");
        if (this.loadState() === 'saving' || this.loadState() === 'loading') return;

        this.form.markAllAsTouched();
        if (this.form.invalid) return;

        const raw = this.form.getRawValue();

        const hmo_id = Number(raw.hmo_id);
        const endorsement_company_id = raw.endorsement_company_id != null ? Number(raw.endorsement_company_id) : NaN;
        const endorsement_type_id = raw.endorsement_type_id != null ? Number(raw.endorsement_type_id) : NaN;
        const endorsement_billing_period_type_id =
            raw.endorsement_billing_period_type_id != null ? Number(raw.endorsement_billing_period_type_id) : NaN;

        const ds = raw.date_start ?? null;
        const de = raw.date_end ?? null;

        if (!Number.isFinite(hmo_id) || hmo_id <= 0) return;
        if (!Number.isFinite(endorsement_company_id) || endorsement_company_id <= 0) return;
        if (!Number.isFinite(endorsement_type_id) || endorsement_type_id <= 0) return;
        if (!Number.isFinite(endorsement_billing_period_type_id) || endorsement_billing_period_type_id <= 0) return;

        if (!ds || !de) return;
        if (de < ds) return;

        const toIsoDate = (d: Date) => {
            const y = d.getFullYear();
            const m = String(d.getMonth() + 1).padStart(2, '0');
            const day = String(d.getDate()).padStart(2, '0');
            return `${y}-${m}-${day}`;
        };

        const retainer_fee = raw.retainer_fee == null ? null : Number(raw.retainer_fee).toFixed(2);

        const id = this.endorsementId();
        const base = this.baseline();

        const current = {
            hmo_id,
            endorsement_company_id,
            endorsement_type_id,
            agreement_corp_number: (raw.agreement_corp_number ?? '').trim() || null,
            date_start: toIsoDate(ds),
            date_end: toIsoDate(de),
            endorsement_billing_period_type_id,
            retainer_fee, // string | null
            remarks: (raw.remarks ?? '').trim() || null,
            endorsement_method: (raw.endorsement_method ?? '').trim() || null,
        };

        this.loadState.set('saving');

        const req$ =
            id == null
                ? this.endorsementService.create_endorsement(current)
                : this.endorsementService.patch_endorsement(id, (() => {
                    const patch: PatchEndorsementRequest = {};
                    if (!base) return current; // fallback

                    if (String(base.hmo_id ?? '') !== String(raw.hmo_id ?? '')) patch.hmo_id = current.hmo_id;
                    if (String(base.endorsement_company_id ?? '') !== String(raw.endorsement_company_id ?? '')) patch.endorsement_company_id = current.endorsement_company_id;
                    if (String(base.endorsement_type_id ?? '') !== String(raw.endorsement_type_id ?? '')) patch.endorsement_type_id = current.endorsement_type_id;

                    if ((base.agreement_corp_number ?? '') !== (raw.agreement_corp_number ?? '')) {
                        patch.agreement_corp_number = current.agreement_corp_number;
                    }

                    const baseDs = base.date_start ? toIsoDate(base.date_start as Date) : null;
                    const baseDe = base.date_end ? toIsoDate(base.date_end as Date) : null;
                    if (baseDs !== current.date_start) patch.date_start = current.date_start;
                    if (baseDe !== current.date_end) patch.date_end = current.date_end;

                    if ((base.endorsement_billing_period_type_id ?? null) !== (raw.endorsement_billing_period_type_id ?? null)) {
                        patch.endorsement_billing_period_type_id = current.endorsement_billing_period_type_id;
                    }

                    const baseRet = base.retainer_fee == null ? null : Number(base.retainer_fee).toFixed(2);
                    if (baseRet !== current.retainer_fee) patch.retainer_fee = current.retainer_fee;

                    if ((base.remarks ?? '') !== (raw.remarks ?? '')) patch.remarks = current.remarks;
                    if ((base.endorsement_method ?? null) !== (raw.endorsement_method ?? null)) patch.endorsement_method = current.endorsement_method;

                    return patch;
                })());

        console.log("Posting or Patching...");
        req$
            .pipe(
                switchMap( (saved)=> {
                    if (!saved) return of(null);

                    const wasNew = id == null;
                    if (wasNew) this.endorsementId.set(saved.id);

                    return this.saveRuleTabs(saved.id).pipe(
                        switchMap( ()=> of({saved, wasNew}))
                    );
                }),
                catchError((err) => {
                    console.error('Save failed:', err);
                    this.loadState.set('error');
                    return of(null);
                }),
                tap((result) => {
                    if (!result) return;
                    const {saved, wasNew} = result;

                    this.baseline.set(this.snapshotForm());
                    this.form.markAsPristine();
                    this.form.markAsUntouched();
                    if (wasNew){
                        this.router.navigate(['../', saved.id], {relativeTo: this.route }).then();
                    }
                }),
                finalize(() => {
                    if (this.loadState() === 'saving') this.loadState.set('idle');
                }),
                takeUntilDestroyed(this.destroyRef),
            )
            .subscribe();
    }

    onDiscardChanges(): void {
        const base = this.baseline();
        if (!base) return; // baseline not set yet (should be set by resetForNew/loadExisting)

        this.form.reset({
            hmo_id: base.hmo_id,
            endorsement_company_id: base.endorsement_company_id,
            endorsement_type_id: base.endorsement_type_id,

            agreement_corp_number: base.agreement_corp_number ?? '',

            endorsement_billing_period_type_id: base.endorsement_billing_period_type_id ?? null,
            retainer_fee: base.retainer_fee ?? null,
            date_start: base.date_start ?? null,
            date_end: base.date_end ?? null,

            remarks: base.remarks ?? '',
            endorsement_method: base.endorsement_method ?? null,
        });

        this.rebuildRuleMatrices();
        // Make it “clean” again
        this.form.markAsPristine();
        this.form.markAsUntouched();
    }

    onViewMasterList = ():void=>{
        console.log("While in setup-endorsement-component, Viewing master list...");
        const id = this.endorsementId();
        if (id == null) return;

        this.endorsementMasterListService.getMasterListForEndorsement(id)
            .pipe(
             tap((members:EndorsementMasterListMemberResponse[])=>{
                 this.masterListMembers.set(members);

                 this.dialog.open(MasterListDialogComponent, {
                     width: '1100px',
                     maxWidth: '95vw',
                     data: {
                         members: this.masterListMembers()
                     },
                 });
             }),
                catchError((err)=>{
                 console.error("Failed to load master list:", err);
                 this.masterListMembers.set([]);
                 return of(null);
                }),
                takeUntilDestroyed(this.destroyRef),
            )
            .subscribe();
    }
}
