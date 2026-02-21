import {Component, computed, DestroyRef, inject, OnInit, signal} from '@angular/core';
import {HMO, HMOService, HMOEditable} from '../../../../api_services/hmoservice';
import {ActivatedRoute, Router} from '@angular/router';
import {takeUntilDestroyed, toSignal} from '@angular/core/rxjs-interop';
import {NonNullableFormBuilder, ReactiveFormsModule, Validators} from '@angular/forms';
import {EMPTY, startWith} from 'rxjs';
import {MatError, MatHint, MatInput, MatLabel} from '@angular/material/input';
import {MatSlideToggle} from '@angular/material/slide-toggle';
import {DatePipe, isPlatformBrowser} from '@angular/common';
import {MatTableModule} from '@angular/material/table';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatButton} from '@angular/material/button';
import {
    GenericDataTableComponent
} from '../../../../components/generic-data-table-component/generic-data-table-component';
import {TableColumn} from '../../../../components/generic-data-table-component/table-interfaces';

interface Endorsement {
    id: number;
    company: string;
    additional_benefits: string;
    date_start: string;
    date_end: string;
    created_by: string;
    endorsed_by: string;
}

@Component({
    selector: 'app-hmopage-component',
    standalone: true,
    imports: [
        ReactiveFormsModule,
        MatLabel,
        MatError,
        MatHint,
        MatSlideToggle,
        DatePipe,
        MatFormFieldModule,
        MatTableModule,
        MatButton,
        MatInput,
        GenericDataTableComponent,
    ],
    templateUrl: './hmopage-component.html',
    styleUrl: './hmopage-component.scss',
})
export class HMOPageComponent implements OnInit {
    isPostingNew = false;
    private fb = inject(NonNullableFormBuilder);
    route = inject(ActivatedRoute);
    readonly hmo = signal<HMO | null>(null);
    id: number | null = null;
    private destroyRef = inject(DestroyRef);

    endorsements: Endorsement[] = [
        {
            id: 1,
            company: 'Petron Corp.',
            additional_benefits: '-',
            date_start: '2021-01-01',
            date_end: '2021-01-31',
            created_by: 'Mhenie',
            endorsed_by: 'Juan Dela Cruz'
        },
        {
            id: 1,
            company: 'Golden Arches Corp.',
            additional_benefits: '-',
            date_start: '2021-01-01',
            date_end: '2021-01-31',
            created_by: 'Mhenie',
            endorsed_by: 'Juan Dela Cruz'
        },
        {
            id: 1,
            company: 'Mercury Drug Inc.',
            additional_benefits: '-',
            date_start: '2021-01-01',
            date_end: '2021-01-31',
            created_by: 'Mhenie',
            endorsed_by: 'Juan Dela Cruz'
        },
    ]
    endorsementColumns: TableColumn[] = [
        {key: 'id', label: 'ID'},
        {key: 'company', label: 'Company'},
        {key: 'additional_benefits', label: 'Additional Benefits'},
        {key: 'date_start', label: 'Date Start', cellTemplateKey: 'date'},
        {key: 'date_end', label: 'Date End', cellTemplateKey: 'date'},
        {key: 'created_by', label: 'Created By'},
        {key: 'endorsed_by', label: 'Endorsed By'},
    ];

    readonly form = this.fb.group({
        short_name: ['', [Validators.required, Validators.maxLength(50)]],
        long_name: ['', Validators.maxLength(255)],
        address: ['', Validators.maxLength(500)],
        tax_account_number: ['', [Validators.maxLength(20)]],
        contact_nos: ['', Validators.maxLength(255)],
        expect_a_master_list: [false],
        active: [true],
    })
    private initialEditable: HMOEditable = {
        short_name: '',
        long_name: '',
        address: '',
        tax_account_number: '',
        contact_nos: '',
        expect_a_master_list: false,
        active: true,
    }
    // Track current form values as a signal so we can compute hasChanges() precisely
    private readonly formValueSig = toSignal(
        this.form.valueChanges.pipe(startWith(this.form.getRawValue())),
        {initialValue: this.form.getRawValue()}
    );

    readonly hasChanges = computed(() => {
        const v = this.formValueSig();
        return !this.equalEditable(
            {
                short_name: v.short_name ?? '',
                long_name: v.long_name ?? '',
                address: v.address ?? '',
                tax_account_number: v.tax_account_number ?? '',
                contact_nos: v.contact_nos ?? '',
                expect_a_master_list: v.expect_a_master_list ?? false,
                active: v.active ?? false,
            },
            this.initialEditable
        );
    });

    // 7 columns, 4 rows lorem table


    constructor(
        private hmoService: HMOService,
        private router: Router,
    ) {
        const raw_id = this.route.snapshot.paramMap.get('id');

        if (raw_id === 'new') {
            this.isPostingNew = true;
            this.id = null;
        } else {
            const n = raw_id !== null ? Number(raw_id) : NaN;

            if (Number.isInteger(n) && n > 0) {
                this.isPostingNew = false;
                this.id = n;
            } else {
                console.log("Invalid HMO ID:", raw_id);
                this.isPostingNew = false;
                this.id = null;
            }
        }


    }

    title() {
        if (this.isPostingNew) {
            return "New HMO";
        } else {
            if (this.id === null) return "Error with HMO ID";
            return "Edit HMO: " + (this.hmo()?.short_name ?? "Unknown");
        }
    }

    subtitle() {
        if (this.isPostingNew) {
            return "Create a new HMO";
        } else {
            if (this.id === null) return "Error with HMO ID";
            return "Edit the information of this HMO";
        }
    }


    ngOnInit(): void {
        if (this.isPostingNew) {
            return;
        }
        this.getHMOData(this.id)
    }

    setHmo(h: HMO): void {
        this.hmo.set(h);

        const editable = this.pickEditable(h);
        this.initialEditable = editable;

        // reset() sets pristine/untouched while applying values
        this.form.reset(editable);
    }


    save(): void {
        if (this.form.invalid || !this.hasChanges()) return;

        const editable = this.editableFromForm();

        const req$ = this.isPostingNew
            ? this.hmoService.postHMO(editable)
            : (this.id != null ? this.hmoService.patchHMO(this.id, editable) : EMPTY)

        req$.subscribe({
            next: (saved) => {
                this.setHmo(saved);
                this.router.navigateByUrl('/main/setup/hmos').then();
            },
            error: (err) => {
                console.log("In save():", err);
            }

        })

    }

    resetChanges(): void {
        this.form.reset(this.initialEditable);
    }

    private pickEditable(h: HMO): HMOEditable {
        return {
            short_name: h.short_name ?? '',
            long_name: h.long_name ?? '',
            address: h.address ?? '',
            tax_account_number: h.tax_account_number ?? '',
            contact_nos: h.contact_nos ?? '',
            expect_a_master_list: h.expect_a_master_list ?? false,
            active: h.active,
        };
    }

    getHMOData(id: number | null) {
        if (id === null) return;
        this.hmoService.getHMOById(id)
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe({
                next: (res) => {
                    this.setHmo(res);
                    console.log("In getHMOData(), HMO:", res);
                },
                error: (err) => {
                    console.log("In getHMOData(), failed to load HMO", err);
                }
            })
    }

    private equalEditable(a: HMOEditable, b: HMOEditable): boolean {
        return ((a.short_name ?? '') === (b.short_name ?? '') &&
            (a.long_name ?? '') === (b.long_name ?? '') &&
            (a.address ?? '') === (b.address ?? '') &&
            (a.tax_account_number ?? '') === (b.tax_account_number ?? '') &&
            (a.contact_nos ?? '') === (b.contact_nos ?? '') && a.active === b.active)&&
            (a.expect_a_master_list ?? false) === (b.expect_a_master_list ?? false);
    }

    private editableFromForm(): HMOEditable {
        const v = this.form.getRawValue();
        return {
            short_name: v.short_name ?? '',
            long_name: v.long_name ?? '',
            address: v.address ?? '',
            tax_account_number: v.tax_account_number ?? '',
            contact_nos: v.contact_nos ?? '',
            expect_a_master_list: v.expect_a_master_list ?? false,
            active: v.active ?? false,
        };
    }

    protected readonly isPlatformBrowser = isPlatformBrowser;
}
