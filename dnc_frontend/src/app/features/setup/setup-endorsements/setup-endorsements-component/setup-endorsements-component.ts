import { CommonModule } from '@angular/common';
import {Component, ChangeDetectionStrategy, DestroyRef, computed,  inject, signal, OnInit} from '@angular/core';
import { FormBuilder, ReactiveFormsModule, Validators } from '@angular/forms';
import {HMOService, HMOOptions} from '../../../../api_services/hmoservice';
import {EndorsementService, EndorsementTypeOptions, BillingFrequencyOptions} from '../../../../api_services/endorsement-service';
import {CurrencyInputComponent} from '../../../../components/currency-input-component/currency-input-component';
import { MatCardModule } from '@angular/material/card';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatSelectModule } from '@angular/material/select';
import { MatRadioModule } from '@angular/material/radio';
import { MatTabsModule } from '@angular/material/tabs';
import { MatDatepickerModule } from '@angular/material/datepicker';
import { MatNativeDateModule } from '@angular/material/core';
import {takeUntilDestroyed} from '@angular/core/rxjs-interop';
import {ActivatedRoute } from '@angular/router';
import {MatChip} from '@angular/material/chips';
import {MatProgressBar} from '@angular/material/progress-bar';
import {MatButton} from '@angular/material/button';
import {
    AddableAutocompleteComponent, AddableAutocompleteItem
} from '../../../../components/addable-autocomplete-component/addable-autocomplete-component';
import {catchError, finalize, map, of, switchMap, tap} from 'rxjs';

type BillingFrequency = 'annual' | 'monthly';
type UIState = 'idle' | 'loading' | 'saving' | 'error';

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
        AddableAutocompleteComponent,
    ],
    templateUrl: './setup-endorsements-component.html',
    styleUrls: ['./setup-endorsements-component.scss'],
})
export class SetupEndorsementsComponent implements OnInit{
    private readonly fb = inject(FormBuilder);
    private readonly route = inject(ActivatedRoute);
    private destroyRef = inject(DestroyRef);
    private readonly hmoService = inject(HMOService);
    private readonly endorsementService = inject(EndorsementService);

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

    readonly form = this.fb.group({
        // Column 1
        hmo_id: ['', [Validators.required]],
        endorsement_company_id: [null as string|null, [Validators.required]],
        agreement_corp_number: ['', Validators.required],

        // Column 2
        billingFrequency: ['annual' as BillingFrequency, [Validators.required]],
        retainer_fee: [null as number | null, [Validators.min(0)]],
        date_start: [null as Date | null],
        date_end: [null as Date | null],

        // Column 3
        endorsement_type_id: [null as string | null, [Validators.required]],
        remarks: [''],
        endorsement_method: [null as string | null],
    });

    /*
    * API Helpers
     */
    private refreshCompanies(){
        return this.endorsementService.getEndorsementCompanies()
            .pipe(
                takeUntilDestroyed(this.destroyRef),
                map((items)=>
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
        // Reset to defaults
        this.form.reset({
            hmo_id: null,
            endorsement_company_id: null,
            agreement_corp_number: '',

            billingFrequency: 'annual',
            retainer_fee: null,
            date_start: null,
            date_end: null,

            endorsement_type_id: null,
            remarks: '',
            endorsement_method: null,
        });

        // Mark pristine baseline
        const snap = this.snapshotForm();
        this.baseline.set(snap);

        // Make UI clean
        this.form.markAsPristine();
        this.form.markAsUntouched();
        this.loadState.set('idle');
    }

    // CHANGED
    private loadExisting(id: number|null): void {
        if (id == null) return;
        this.loadState.set('loading');

    }
    private snapshotForm() {
        const v = this.form.getRawValue();
        return {
            hmo_id: v.hmo_id,
            endorsement_company_id: v.endorsement_company_id,
            endorsement_type_id: v.endorsement_type_id,

            agreement_corp_number: v.agreement_corp_number ?? '',

            billingFrequency: v.billingFrequency,
            retainer_fee: v.retainer_fee ?? null,
            date_start: v.date_start ?? null,
            date_end: v.date_end ?? null,

            remarks: v.remarks ?? '',
            endorsement_method: v.endorsement_method ?? null,
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
    onSave(){

    }
    onDiscardChanges(): void {
        const base = this.baseline();
        if (!base) return; // baseline not set yet (should be set by resetForNew/loadExisting)

        this.form.reset({
            hmo_id: base.hmo_id,
            endorsement_company_id: base.endorsement_company_id,
            endorsement_type_id: base.endorsement_type_id,

            agreement_corp_number: base.agreement_corp_number ?? '',

            billingFrequency: base.billingFrequency,
            retainer_fee: base.retainer_fee ?? null,
            date_start: base.date_start ?? null,
            date_end: base.date_end ?? null,

            remarks: base.remarks ?? '',
            endorsement_method: base.endorsement_method ?? null,
        });

        // Make it “clean” again
        this.form.markAsPristine();
        this.form.markAsUntouched();
    }
}
