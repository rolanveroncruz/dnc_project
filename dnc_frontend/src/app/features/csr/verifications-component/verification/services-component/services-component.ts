import {Component, EventEmitter,  input, Output, signal} from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatCheckboxModule } from '@angular/material/checkbox';
import { MatButtonModule } from '@angular/material/button';
import { MemberServicesCountsSummary } from '../../../../../api_services/member-services-counts-service';

@Component({
    selector: 'app-services',
    standalone: true,
    imports: [
        CommonModule,
        MatCheckboxModule,
        MatButtonModule,
    ],
    templateUrl: './services-component.html',
    styleUrl: './services-component.scss',
})
export class ServicesComponent {
    readonly services = input<MemberServicesCountsSummary[]>([]);
    @Output() checkedServicesChange = new EventEmitter<MemberServicesCountsSummary[]>();
    @Output() checkedIdsChange = new EventEmitter<number[]>();

    // readonly checkedIds = signal<number[]>([]);
    readonly selectedQuantities = signal<Record<number, number>>({});

    get sortedServices(): MemberServicesCountsSummary[] {
        return [...this.services()].sort((a, b) =>
            (a.dental_service_id ?? 0) - (b.dental_service_id ?? 0)
        );
    }
    get type1Services(): MemberServicesCountsSummary[] {
        return this.sortedServices.filter(service => service.dental_service_type_id === 1);
    }
    get type2Services(): MemberServicesCountsSummary[] {
        return this.sortedServices.filter(service => service.dental_service_type_id === 2);
    }
    get type3Services(): MemberServicesCountsSummary[] {
        return this.sortedServices.filter(service => service.dental_service_type_id === 3);
    }
    private fillFixedColumns(
        items: MemberServicesCountsSummary[],
        columnCount: number,
        maxPerColumn: number
    ): MemberServicesCountsSummary[][] {
        const columns: MemberServicesCountsSummary[][] = Array.from(
            { length: columnCount },
            () => []
        );

        items.forEach((item, index) => {
            const columnIndex = Math.floor(index / maxPerColumn);

            if (columnIndex < columnCount) {
                columns[columnIndex].push(item);
            }
        });

        return columns;
    }


    get serviceColumns(): MemberServicesCountsSummary[][] {
        return [
            ...this.fillFixedColumns(this.type1Services,  2,4),
            ...this.fillFixedColumns(this.type2Services,  2,4),
            ...this.fillFixedColumns(this.type3Services,  1,4),
        ]
    }

    isChecked(serviceId: number): boolean {
        return (this.selectedQuantities()[serviceId] ?? 0) > 0;
    }

    isDisabled(service: MemberServicesCountsSummary): boolean {
        return service.has_pending;
    }

    labelFor(service: MemberServicesCountsSummary): string {
        if (service.has_pending) {
            return `${service.dental_service_name} - pending verification last ${this.formatConflictDate(service.conflict_date)}`;
        }

        return `${service.dental_service_name} (${service.counts_used}/${service.counts_allowed})`;
    }

    onToggle(service: MemberServicesCountsSummary, checked: boolean): void {
        if (service.has_pending) {
            return;
        }

        const current = {...this.selectedQuantities() };
        const serviceId = service.dental_service_id;
        if (checked) {
            current[serviceId] = 1;
        }else{
            delete current[serviceId];
        }

        this.selectedQuantities.set(current);
        this.emitSelection();
    }

    reserveAllBasic(): void {
        const next = { ...this.selectedQuantities() };

        this.services()
            .filter(service => service.dental_service_type_id === 1 && !service.has_pending)
            .forEach(service => {
                next[service.dental_service_id] = 1;
            });

        this.selectedQuantities.set(next);
        this.emitSelection();
    }
    clearAll(): void {
        this.selectedQuantities.set({});
        this.emitSelection();
    }

    private emitSelection(): void {
        const quantities = this.selectedQuantities();

        const ids = Object.entries(quantities).flatMap(([serviceId, quantity]) =>
            Array(quantity).fill(Number(serviceId))
        );

        this.checkedIdsChange.emit(ids);

        const selectedServices = this.services().filter(service =>
            (quantities[service.dental_service_id] ?? 0) > 0
        );
        this.checkedServicesChange.emit(selectedServices);
    }

    private formatConflictDate(value: Date | string | null): string {
        if (!value) {
            return '';
        }

        const date = value instanceof Date ? value : new Date(value);

        if (Number.isNaN(date.getTime())) {
            return String(value);
        }

        return date.toLocaleDateString();
    }

    getQuantity(serviceId: number): number {
        return this.selectedQuantities()[serviceId] ?? 0;
    }
    isRepeatable(service: MemberServicesCountsSummary): boolean {
        return service.verification_counts_allowed > 1;
    }

    canIncrement(service: MemberServicesCountsSummary): boolean {
        return (
            !service.has_pending &&
            this.getQuantity(service.dental_service_id) < service.verification_counts_allowed &&
            this.getQuantity(service.dental_service_id) < service.counts_allowed
        );
    }
    canDecrement(service: MemberServicesCountsSummary): boolean {
        return !service.has_pending && this.getQuantity(service.dental_service_id) > 0;
    }

    incrementQuantity(service: MemberServicesCountsSummary): void {
        if (service.has_pending) {
            return;
        }

        const serviceId = service.dental_service_id;
        const current = { ...this.selectedQuantities() };
        const currentQty = current[serviceId] ?? 0;
        const maxQty = service.verification_counts_allowed;

        if (currentQty >= maxQty) {
            return;
        }

        current[serviceId] = currentQty + 1;
        this.selectedQuantities.set(current);
        this.emitSelection();
    }

    decrementQuantity(service: MemberServicesCountsSummary): void {
        if (service.has_pending) {
            return;
        }

        const serviceId = service.dental_service_id;
        const current = { ...this.selectedQuantities() };
        const currentQty = current[serviceId] ?? 0;

        if (currentQty <= 1) {
            delete current[serviceId];
        } else {
            current[serviceId] = currentQty - 1;
        }

        this.selectedQuantities.set(current);
        this.emitSelection();
    }

    protected readonly Object = Object;
}
