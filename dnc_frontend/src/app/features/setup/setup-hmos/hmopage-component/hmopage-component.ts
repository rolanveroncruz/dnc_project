import {Component, computed, DestroyRef, inject, OnInit, signal} from '@angular/core';
import {HMO, HMOService} from '../../../../api_services/hmoservice';
import {ActivatedRoute } from '@angular/router';
import {takeUntilDestroyed, toSignal} from '@angular/core/rxjs-interop';
import {NonNullableFormBuilder, ReactiveFormsModule, Validators} from '@angular/forms';
import {startWith} from 'rxjs';
import {MatError, MatHint, MatInput, MatLabel} from '@angular/material/input';
import {MatSlideToggle} from '@angular/material/slide-toggle';
import {DatePipe} from '@angular/common';
import {MatTableModule} from '@angular/material/table';
import { MatFormFieldModule } from '@angular/material/form-field';
import {MatButton} from '@angular/material/button';
import {
  GenericDataTableComponent
} from '../../../../components/generic-data-table-component/generic-data-table-component';
import {TableColumn} from '../../../../components/generic-data-table-component/table-interfaces';
interface Endorsement{
  id: number;
  company: string;
  additional_benefits: string;
  date_start: string;
  date_end: string;
  created_by: string;
  endorsed_by: string;
}
type HmoEditable = Pick<
  HMO,
  'short_name' | 'long_name' | 'address' | 'tax_account_number' | 'contact_nos' | 'active'
>;
@Component({
  selector: 'app-hmopage-component',
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
export class HMOPageComponent implements OnInit{
  private fb = inject(NonNullableFormBuilder);
  route = inject(ActivatedRoute);
  readonly hmo = signal<HMO|null>(null);
  id: number;
  private destroyRef = inject(DestroyRef);

  endorsements: Endorsement[] = [
    { id: 1, company: 'Petron Corp.', additional_benefits: '-', date_start: '2021-01-01', date_end: '2021-01-31', created_by: 'Mhenie', endorsed_by: 'Juan Dela Cruz'},
    { id: 1, company: 'Golden Arches Corp.', additional_benefits: '-', date_start: '2021-01-01', date_end: '2021-01-31', created_by: 'Mhenie', endorsed_by: 'Juan Dela Cruz'},
    { id: 1, company: 'Mercury Drug Inc.', additional_benefits: '-', date_start: '2021-01-01', date_end: '2021-01-31', created_by: 'Mhenie', endorsed_by: 'Juan Dela Cruz'},
  ]
  endorsementColumns: TableColumn[] = [
    {key: 'id', label: 'ID'},
    {key: 'company', label: 'Company'},
    {key: 'additional_benefits', label: 'Additional Benefits'},
    {key: 'date_start', label: 'Date Start', cellTemplateKey: 'date'},
    {key: 'date_end', label: 'Date End', cellTemplateKey: 'date'},
    {key: 'created_by', label: 'Created By' },
    {key: 'endorsed_by', label: 'Endorsed By'},
  ];

  readonly form = this.fb.group({
    short_name: ['', [Validators.required, Validators.maxLength(50)]],
    long_name: ['', Validators.maxLength(255)],
    address: ['', Validators.maxLength(500)],
    tax_account_number: ['', [ Validators.maxLength(20)]],
    contact_nos: ['', Validators.maxLength(255)],
    active: [true],
  })
  private initialEditable: HmoEditable = {
    short_name: '',
    long_name: '',
    address: '',
    tax_account_number: '',
    contact_nos: '',
    active: true,
  }
  // Track current form values as a signal so we can compute hasChanges() precisely
  private readonly formValueSig = toSignal(
    this.form.valueChanges.pipe(startWith(this.form.getRawValue())),
    { initialValue: this.form.getRawValue() }
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
        active: v.active ?? false,
      },
      this.initialEditable
    );
  });
  // 7 columns, 4 rows lorem table


  constructor(private hmoService: HMOService) {
    this.id = Number(this.route.snapshot.paramMap.get('id'))

  }

  ngOnInit(): void {
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

    const current = this.hmo();
    if (!current) return;

    const updated: HMO = {
      ...current,
      ...this.form.getRawValue(),
    };

    // TODO: call your service here (PUT/PATCH), then on success:
    this.setHmo(updated);
  }

  resetChanges(): void {
    this.form.reset(this.initialEditable);
  }

  private pickEditable(h: HMO): HmoEditable {
    return {
      short_name: h.short_name ?? '',
      long_name: h.long_name ?? '',
      address: h.address ?? '',
      tax_account_number: h.tax_account_number ?? '',
      contact_nos: h.contact_nos ?? '',
      active: h.active,
    };
  }

  getHMOData(id: number){
    this.hmoService.getHMOById(id)
      .pipe(takeUntilDestroyed(this.destroyRef))
      .subscribe({
        next: (res) => {
          this.setHmo(res);
          console.log("In getHMOData(), HMO:",res);
        },
        error: (err) => {
          console.log("In getHMOData(), failed to load HMO", err);
        }
      })
  }
  private equalEditable(a: HmoEditable, b: HmoEditable): boolean {
    return ((a.short_name ?? '') === (b.short_name ?? '') &&
      (a.long_name ?? '') === (b.long_name ?? '') &&
      (a.address ?? '') === (b.address ?? '') &&
      (a.tax_account_number ?? '') === (b.tax_account_number ?? '') &&
      (a.contact_nos ?? '') === (b.contact_nos ?? '') && a.active === b.active);
  }

}
