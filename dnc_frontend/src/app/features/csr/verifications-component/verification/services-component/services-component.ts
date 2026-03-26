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

    readonly checkedIds = signal<number[]>([]);

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
        return this.checkedIds().includes(serviceId);
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

        const current = this.checkedIds();

        const next = checked
            ? [...new Set([...current, service.dental_service_id])]
            : current.filter(id => id !== service.dental_service_id);

        this.checkedIds.set(next);
        this.emitSelection();
    }

    reserveAllBasic(): void {
        const basicIds = this.services()
            .filter(service => service.dental_service_type_id === 1 && !service.has_pending)
            .map(service => service.dental_service_id);

        const merged = [...new Set([...this.checkedIds(), ...basicIds])];
        this.checkedIds.set(merged);
        this.emitSelection();
    }

    clearAll(): void {
        this.checkedIds.set([]);
        this.emitSelection();
    }

    private emitSelection(): void {
        const ids = this.checkedIds();

        this.checkedIdsChange.emit(ids);

        const selectedServices = this.services().filter(service =>
            ids.includes(service.dental_service_id)
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
}
