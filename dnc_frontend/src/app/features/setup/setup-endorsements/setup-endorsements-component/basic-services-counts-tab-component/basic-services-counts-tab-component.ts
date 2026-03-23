import {Component, computed, Input, signal} from '@angular/core';
import {MatFormField, MatInput, MatLabel} from '@angular/material/input';
import {FormArray, FormControl, FormGroup, ReactiveFormsModule} from '@angular/forms';
import {DentalService} from '../../../../../api_services/dental-services-service';

@Component({
  selector: 'app-basic-services-counts-tab-component',
    imports: [
        MatFormField,
        MatInput,
        MatLabel,
        ReactiveFormsModule
    ],
  templateUrl: './basic-services-counts-tab-component.html',
  styleUrl: './basic-services-counts-tab-component.scss',
})
export class BasicServicesCountsTabComponent {
    // ---- Inputs as signals
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

    // Lookup: dental_service_id -> row group
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

    readonly filteredServices = computed(() => {
        const q = this.search().trim().toLowerCase();
        const services = this.servicesSig();
        if (!q) return services;
        return services.filter(s => (s.name ?? '').toLowerCase().includes(q));
    });

    limitControlFor(serviceId: number): FormControl<number | null> | null {
        const fg = this.rowByServiceId().get(serviceId);
        if (!fg) return null;
        return fg.get('limit') as FormControl<number | null>;
    }

    // Optional: normalize to integer when user leaves the field
    normalizeLimit(ctrl: FormControl<number | null>): void {
        const v = ctrl.value;
        if (v == null) return;
        const n = Number(v);
        if (!Number.isFinite(n)) {
            ctrl.setValue(null);
            return;
        }
        ctrl.setValue(Math.trunc(n));
    }

    trackById = (_: number, s: DentalService) => s.id;
}

