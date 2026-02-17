import {Component, computed, DestroyRef, effect, inject, OnInit, signal} from '@angular/core';
import {FormControl, FormGroup, NonNullableFormBuilder, ReactiveFormsModule, Validators} from '@angular/forms';
import {MatCard, MatCardHeader, MatCardTitle, MatCardSubtitle, MatCardContent} from '@angular/material/card';
import {MatDivider} from '@angular/material/list';
import {MatError, MatFormField, MatInput, MatLabel} from '@angular/material/input';
import {MatTab, MatTabGroup} from '@angular/material/tabs';
import {ActivatedRoute, Router} from '@angular/router';
import {MatOption, MatSelect} from '@angular/material/select';
import {MatButton} from '@angular/material/button';

import {BasicServicesTabComponent} from './basic-services-tab-component/basic-services-tab-component';
import {SpecialServicesTabComponent} from './special-services-tab-component/special-services-tab-component';
import {DentalServicesService } from '../../../api_services/dental-services-service';
import {takeUntilDestroyed} from '@angular/core/rxjs-interop';
import {DentistContractsService, DentistContractWithRates} from '../../../api_services/dentist-contracts-service';
import {catchError, finalize, map, of, tap} from 'rxjs';

type ServiceType = 'Basic' | 'Special' | 'High-End';

export interface DentalService {
  id: number;
  name: string;
  type: ServiceType;
  sort_index: number;
  active: boolean;
}

export interface DentistContractListItem {
  id: number;
  name: string;
}

export interface DentistContractDetails {
  id: number;
  name: string;
  description: string;

  // map serviceId -> rate
  rates: Record<number, number>;
}

@Component({
  selector: 'app-setup-dentist-contracts',
  standalone: true,
  imports: [
    MatCard,
    MatCardHeader,
    MatCardTitle,
    MatCardSubtitle,
    MatCardContent,
    MatTabGroup,
    MatTab,
    BasicServicesTabComponent,
    SpecialServicesTabComponent,
    MatLabel,
    MatFormField,
    MatSelect,
    MatOption,
    MatDivider,
    MatButton,
    ReactiveFormsModule,
    MatInput,
    MatError
  ],
  templateUrl: './setup-dentist-contracts.html',
  styleUrl: './setup-dentist-contracts.scss',
})
export class SetupDentistContracts implements OnInit {

  readonly NEW_CONTRACT_SENTINEL = -1;

  private fb = inject(NonNullableFormBuilder);
  private destroyRef = inject(DestroyRef);

  //-----router stuff
  private router = inject(Router);
  private route = inject(ActivatedRoute);

  // ----UI state
  readonly selectedTabIndex = signal<number>(0);
  readonly isBusy = signal<boolean>(false);
  readonly saveError = signal<string | null>(null);

  //---- master data
  readonly contracts = signal<DentistContractListItem[]>([]);
  readonly services = signal<DentalService[]>([]);

  //----selection/mode
  readonly selectedContractId = signal<number | null>(null);
  readonly isCreateMode = signal(false);

  //----form
  readonly contractForm = this.fb.group({
    name: ['', Validators.required],
    description: ['', Validators.maxLength(500)],
  });

  /**
   * Rates are a single FormGroup where each control name is the service id as string.
   * Example: ratesGroup.controls["12"] is the rate for service id 12.
   */
  ratesGroup = new FormGroup<Record<string, FormControl<number | null>>>({});

  // --- derived lists for tabs
  readonly basicServices = computed(() =>
    this.services().filter(s => s.type === 'Basic' && s.active)
  );

  readonly specialServices = computed(() =>
    this.services().filter(s => s.type === 'Special' && s.active)
  );

  constructor(
    private dentalServicesService: DentalServicesService,
    private dentistContractsService: DentistContractsService) {
          effect(() => {
              if (this.isBusy()) {
                  this.contractForm.disable({emitEvent: false});
                  this.ratesGroup.disable({emitEvent: false});
              } else {
                  this.contractForm.enable({emitEvent: false});
                  this.ratesGroup.enable({emitEvent: false});
              }
          });

    // this.loadServices();
    this.rebuildRatesControls();
  }

  ngOnInit(): void {
    // Load Contracts List
    this.loadContracts();

    // Load Services List
    this.dentalServicesService.getDentalServices().pipe(
        takeUntilDestroyed(this.destroyRef),
        map(page=>(page.items ??[]).map(transformToDentalService)),

        tap(( services:DentalService[])=>{
            const sorted = [...services].sort((a,b)=>(a.sort_index??0)-(b.sort_index ?? 0));
            this.services.set(sorted);
            console.log("In ngOnInit(), this.services:", this.services());
            this.rebuildRatesControls();
        }),
        catchError(err=> {
            console.log("Error in ngOnInit():", err);
            return of([] as DentalService[]);
        })
    ).subscribe();
  }

  // loadContracts() calls this.dentistContractsService.getAll(),
  // and sets the contracts signal to the returned list of contracts.
  private loadContracts() {
    this.isBusy.set(true);
    this.dentistContractsService.getAll()
      .pipe(
        takeUntilDestroyed(this.destroyRef),
        finalize(() => this.isBusy.set(false))
      )
      .subscribe({
        next: rows => {
          this.contracts.set(rows.map(r => ({id: r.id, name: r.name})));
        },
        error: _ => {
          this.saveError.set('Failed to load dentist contracts.');
        }
      });
  }

  onTabIndexChange(index: number) {
    this.selectedTabIndex.set(index);
    const tab = index === 1 ? 'special' : 'basic';
    this.router.navigate([], {
      relativeTo: this.route,
      queryParams: {tab},
      queryParamsHandling: 'merge',
      replaceUrl: true
    }).then();
  }

  /** Call this after you fetch services from API */
  rebuildRatesControls() {
    const all = this.services();
    const next: Record<string, FormControl<number | null>> = {};

    for (const s of all) {
      next[String(s.id)] = new FormControl<number | null>(
        null,
        {validators: [Validators.required, Validators.min(0)]}
      );
    }
    this.ratesGroup = new FormGroup(next);
  }


  /** Dropdown selection */
  onSelectContract(value: number) {
    if (value === this.NEW_CONTRACT_SENTINEL) {
      this.startCreate();
      return;
    }

    // If you want: prompt if dirty before switching. For now, we just switch.
    this.isCreateMode.set(false);
    this.selectedContractId.set(value);

    this.loadContractDetails(value);
  }

  // CHANGE: new helper to load a single contract
  // When onSelectContract is called because the user selected a new contract from the list,
  // this will load the contract details from the backend and patch the form.
  private loadContractDetails(id: number) {
    this.isBusy.set(true);
    this.saveError.set(null);

    this.dentistContractsService.getById(id)
      .pipe(
        takeUntilDestroyed(this.destroyRef),
        finalize(() => this.isBusy.set(false))
      )
      .subscribe({
        next: (data: DentistContractWithRates) => {
          // convert service response into your DentistContractDetails model
          const ratesMap: Record<number, number> = {};
          for (const r of data.rates) {
            // CHANGE: tolerate either service_id or dental_service_id from backend
            const sid = r.service_id ?? r.dental_service_id;
            if (sid != null) ratesMap[sid] = r.rate;
          }

          this.patchFromContract({
            id: data.contract.id,
            name: data.contract.name,
            description: data.contract.description,
            rates: ratesMap,
          });
        },
        error: _ => {
          this.saveError.set('Failed to load contract details.');
        }
      });
  }

  startCreate() {
    this.isCreateMode.set(true);
    this.selectedContractId.set(null);

    this.contractForm.reset({name: '', description: ''});

    // Since every service must have a rate, pick your default:
    // - If 0 is allowed: set 0
    // - If 0 is not allowed: leave null and require user input
    Object.values(this.ratesGroup.controls).forEach(ctrl => ctrl.setValue(0));
    this.markPristine();
  }


  /** When you load an existing contract from backend */
  patchFromContract(details: DentistContractDetails) {
    this.isCreateMode.set(false);
    this.selectedContractId.set(details.id);

    this.contractForm.patchValue({
      name: details.name,
      description: details.description ?? '',
    });

    // CHANGE: ensure all services get a value (default to 0 if missing)
    for (const s of this.services()) {
      const ctrl = this.ratesGroup.controls[String(s.id)];
      if (!ctrl) continue;
      const rate = details.rates[s.id] ?? 0;
      ctrl.setValue(rate);
    }


    this.markPristine();
  }

  cancelEdits() {
    // simplest: re-load the current selection or clear if creating
    if (this.isCreateMode()) {
      this.startCreate(); // resets to freshen create defaults
      return;
    }

    const id = this.selectedContractId();
    if (!id) return;

    // reload from API
    this.loadContractDetails(id);
  }

  save() {
    this.saveError.set(null);

    // mark touched so errors show
    this.contractForm.markAllAsTouched();
    this.ratesGroup.markAllAsTouched();

    if (!this.canSave()) return;

    const payload = {
      name: this.contractForm.controls.name.value.trim(),
      description: this.contractForm.controls.description.value ?? '',
      active: true,
      rates: this.buildRatesPayload(),
    };
    this.isBusy.set(true);

    if (this.isCreateMode()) {
      this.dentistContractsService.create(payload)
        .pipe(
          takeUntilDestroyed(this.destroyRef),
          finalize(() => this.isBusy.set(false))
        )
        .subscribe({
          next: res => {
            // CHANGE: refresh dropdown, select created, patch UI
            this.loadContracts();
            this.loadContractDetails(res.contract.id);
            this.isCreateMode.set(false);
            this.selectedContractId.set(res.contract.id);
          },
          error: _ => {
            this.saveError.set('Failed to create contract.');
          }
        });
    } else {
      const id = this.selectedContractId();
      if (!id) return;

      this.dentistContractsService.patch(id, {
        name: payload.name,
        description: payload.description,
        active: payload.active,
        rates: payload.rates,
      })
        .pipe(
          takeUntilDestroyed(this.destroyRef),
          finalize(() => this.isBusy.set(false))
        )
        .subscribe({
          next: _ => {
            // CHANGE: refresh dropdown names in case name changed; reload details to re-pristine
            this.loadContracts();
            this.loadContractDetails(id);
          },
          error: _ => {
            this.saveError.set('Failed to save contract.');
          }
        });
    }
  }

  buildRatesPayload(): Array<{ service_id: number; rate: number }> {
    const out: Array<{ service_id: number; rate: number }> = [];
    for (const s of this.services()) {
      const v = this.ratesGroup.controls[String(s.id)]?.value;
      out.push({service_id: s.id, rate: Number(v)});
    }
    return out;
  }

  // --- validity/dirty helpers
  hasChanges(): boolean {
    return this.contractForm.dirty || this.ratesGroup.dirty;
  }

  canSave(): boolean {
    return this.contractForm.valid && this.ratesGroup.valid && this.hasChanges();
  }

  private markPristine() {
    this.contractForm.markAsPristine();
    this.ratesGroup.markAsPristine();
  }
}

function transformToDentalService(s:any):DentalService{
    return {
        id: s.id,
        name: s.name,
        type: s.type_name === 'Basic' ? 'Basic'
            : s.type_name === 'Special' ? 'Special'
                : 'High-End',
        sort_index: s.sort_index,
        active: s.active
    }
}
