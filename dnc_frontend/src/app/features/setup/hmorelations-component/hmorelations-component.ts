import { Component, EventEmitter, Input, Output } from '@angular/core';
import { CommonModule } from '@angular/common';

import { MatCardModule } from '@angular/material/card';
import { MatListModule, MatSelectionListChange } from '@angular/material/list';
import { MatButtonModule } from '@angular/material/button';

export type HmoListItem = { id: number; short_name: unknown };

@Component({
    selector: 'app-hmo-relations',
    standalone: true,
    imports: [CommonModule, MatCardModule, MatListModule, MatButtonModule],
    templateUrl: './hmorelations-component.html',
    styleUrl: './hmorelations-component.scss',
})
export class HMORelationsComponent {
    @Input({ required: true }) title!: string;

    // accept what backend sends; we’ll normalize for display
    @Input({ required: true }) items: readonly HmoListItem[] = [];
    @Input() maxItems = 4;

    @Input() addLabel = 'Add';
    @Input() removeLabel = 'Remove';

    @Input() disableAdd = false;
    @Input() disableRemove = false;

    @Output() add = new EventEmitter<void>();
    @Output() remove = new EventEmitter<{ selected?: { id: number; short_name: string } }>();

    private selectedId?: number;

    get visibleItems(): readonly HmoListItem[] {
        return (this.items ?? []).slice(0, this.maxItems);
    }

    get canRemove(): boolean {
        const hasItems = (this.items?.length ?? 0) > 0;
        return hasItems && !this.disableRemove;
    }

    /** Always return a real string for display */
    displayShortName(item: any): string {
        const v =
            item?.short_name ??
            item?.shortName ?? // common mismatch from APIs
            '';

        // if v is object/array/etc, String(v) becomes "[object Object]" — so handle that too
        if (typeof v === 'string') return v;
        if (v == null) return '';
        if (typeof v === 'number' || typeof v === 'boolean') return String(v);

        // try common nested shapes (e.g., { value: "ABC" } / { name: "ABC" })
        if (typeof v === 'object') {
            const nested = (v as any).value ?? (v as any).name ?? (v as any).short_name ?? (v as any).shortName;
            if (typeof nested === 'string') return nested;
        }

        // last resort: show nothing instead of "[object Object]"
        return '';
    }

    onSelectionChange(ev: MatSelectionListChange) {
        this.selectedId = ev.options?.[0]?.value as number | undefined;
    }

    onAddClick() {
        if (this.disableAdd) return;
        this.add.emit();
    }

    onRemoveClick() {
        if (!this.canRemove) return;

        const raw = this.selectedId == null
            ? undefined
            : (this.items ?? []).find(x => x.id === this.selectedId);

        const selected = raw
            ? { id: raw.id, short_name: this.displayShortName(raw) }
            : undefined;

        this.remove.emit({ selected });
    }
}
