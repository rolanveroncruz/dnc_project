import {Component, computed, DestroyRef, inject, OnInit, signal} from '@angular/core';
import { FormControl, FormGroup, NonNullableFormBuilder, ReactiveFormsModule, Validators} from '@angular/forms';
import {MatCard, MatCardHeader, MatCardTitle, MatCardSubtitle, MatCardContent} from '@angular/material/card';
import {MatDivider} from '@angular/material/list';
import {MatError, MatFormField, MatInput, MatLabel} from '@angular/material/input';
import {MatTab, MatTabGroup} from '@angular/material/tabs';
import {ActivatedRoute, Router} from '@angular/router';
import {MatOption, MatSelect} from '@angular/material/select';
import {MatButton} from '@angular/material/button';

import {BasicServicesTabComponent} from './basic-services-tab-component/basic-services-tab-component';
import {SpecialServicesTabComponent} from './special-services-tab-component/special-services-tab-component';
import {DentalServicesService} from '../../../api_services/dental-services-service';
import {takeUntilDestroyed} from '@angular/core/rxjs-interop';

type ServiceType = 'Basic' | 'Special' | 'High-End';

export interface DentalService {
  id: number;
  name: string;
  type: ServiceType;
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
export class SetupDentistContracts implements OnInit{
  readonly NEW_CONTRACT_SENTINEL=1;
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
  readonly isCreateMode=signal(false);

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

  constructor(private dentalServicesService:DentalServicesService) {
    // TODO: load contracts list + services list from backend
    // this.loadContracts();
    // this.loadServices();
    this.rebuildRatesControls();
  }

  ngOnInit(): void {
    this.dentalServicesService.getDentalServices()
      .pipe(takeUntilDestroyed(this.destroyRef))
      .subscribe({
        next: value => {
          console.log("In ngOnInit():", value);
          let services:DentalService[] = [];
          for (const s of value.items) {
            services.push({
              id: s.id,
              name: s.name,
              type: s.type_name === 'Basic' ? 'Basic' : s.type_name === 'Special' ? 'Special' : 'High-End',
              active: s.active});
          }
          this.services.set(services);
          this.rebuildRatesControls();
        },
        error: err => {

        }
      })
    }

  onTabIndexChange(index: number) {
    this.selectedTabIndex.set(index);
    const tab = index === 1 ? 'permissions' : 'roles';
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
        { validators: [Validators.required, Validators.min(0)] }
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

    // TODO: load contract details from API, then patch
    // this.loadContractDetails(value);
  }

  startCreate() {
    this.isCreateMode.set(true);
    this.selectedContractId.set(null);

    this.contractForm.reset({ name: '', description: '' });

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

    for (const [sid, rate] of Object.entries(details.rates)) {
      const ctrl = this.ratesGroup.controls[String(sid)];
      if (ctrl) ctrl.setValue(rate);
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

    // TODO: reload from API and patch
    // this.loadContractDetails(id);
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
      rates: this.buildRatesPayload(),
    };

    if (this.isCreateMode()) {
      // TODO: POST create with rates
      // this.createContract(payload)
    } else {
      const id = this.selectedContractId();
      if (!id) return;
      // TODO: PATCH contract + PUT rates (bulk) or single endpoint
      // this.updateContract(id, payload)
    }
  }

  buildRatesPayload(): Array<{ dental_service_id: number; rate: number }> {
    const out: Array<{ dental_service_id: number; rate: number }> = [];
    for (const s of this.services()) {
      const v = this.ratesGroup.controls[String(s.id)]?.value;
      out.push({ dental_service_id: s.id, rate: Number(v) });
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
