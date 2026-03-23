import {Component, computed, Input, signal} from '@angular/core';
import {DentalService} from '../../../../../api_services/dental-services-service';
import {FormArray, FormControl, FormGroup, ReactiveFormsModule} from '@angular/forms';
import {MatFormField, MatInput, MatLabel} from '@angular/material/input';

@Component({
  selector: 'app-basic-services-fees-tab-component',
    imports: [
        MatFormField,
        MatInput,
        MatLabel,
        ReactiveFormsModule
    ],
  templateUrl: './basic-services-fees-tab-component.html',
  styleUrl: './basic-services-fees-tab-component.scss',
})
export class BasicServicesFeesTabComponent {
    private readonly _services = signal<DentalService[]>([]);
    private readonly _rows = signal<FormArray<FormGroup> | null>(null);

    @Input({ required: true })
    set services(v: DentalService[] | null | undefined) {
        this._services.set(v ?? []);
    }
    servicesSig = this._services.asReadonly();

    @Input({ required: true })
    set rows(v: FormArray<FormGroup> | null | undefined) {
        this._rows.set(v ?? null);
    }
    rowsSig = this._rows.asReadonly();

    @Input()
    disabled = false;

    // ---- Search
    readonly search = signal('');

    // Build a lookup map: dental_service_id -> row FormGroup
    readonly rowByServiceId = computed(() => {
        const rows = this.rowsSig();
        const map = new Map<number, FormGroup>();

        if (!rows) return map;

        for (const fg of rows.controls) {
            const idCtrl = fg.get('dental_service_id') as FormControl<number> | null;
            const id = idCtrl?.value;
            if (typeof id === 'number') map.set(id, fg);
        }
        return map;
    });

    // Filtered services based on search
    readonly filteredServices = computed(() => {
        const q = this.search().trim().toLowerCase();
        const services = this.servicesSig();

        if (!q) return services;

        return services.filter(s => (s.name ?? '').toLowerCase().includes(q));
    });

    readonly missingRateCtrl = new FormControl<number|null>({value:null, disabled:true });
    // Helper for template
    rateControlFor(serviceId: number): FormControl<number | null> {
        const fg = this.rowByServiceId().get(serviceId);
        const ctrl = fg?.get('rate') as FormControl<number|null> |null;
        return ctrl ?? this.missingRateCtrl;
    }

    trackById = (_: number, s: DentalService) => s.id;
}

