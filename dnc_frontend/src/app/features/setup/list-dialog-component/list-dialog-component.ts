import { CommonModule } from '@angular/common';
import { Component, computed, inject, signal } from '@angular/core';
import { FormControl, ReactiveFormsModule } from '@angular/forms';
import { MAT_DIALOG_DATA, MatDialogModule, MatDialogRef } from '@angular/material/dialog';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatRadioModule } from '@angular/material/radio';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatSelectModule } from '@angular/material/select';

export interface ListDialogItem {
    id: number | string;
    label: string;
    /** Optional: use if you want the filter dropdown */
    group?: string | null;
    /** Optional extra search text (e.g., code, alias) */
    keywords?: string[];
}

export interface ListDialogData {
    title?: string;
    subtitle?: string;

    items: ListDialogItem[];

    /** Optional initial selection */
    initialSelectedId?: number | string | null;

    /**
     * Optional: when true, show the group filter dropdown.
     * If you never need it, set false or omit.
     */
    enableGroupFilter?: boolean;

    /** Optional label overrides */
    searchLabel?: string;
    filterLabel?: string;
    emptyText?: string;
    saveText?: string;
    cancelText?: string;
}

export interface ListDialogResult {
    selectedId: number;
    selected: ListDialogItem;
}

@Component({
    selector: 'app-list-dialog',
    standalone: true,
    imports: [
        CommonModule,
        ReactiveFormsModule,
        MatDialogModule,
        MatFormFieldModule,
        MatInputModule,
        MatSelectModule,
        MatRadioModule,
        MatButtonModule,
        MatIconModule,
    ],
    templateUrl: './list-dialog-component.html',
    styleUrl: './list-dialog-component.scss',
})
export class ListDialogComponent {
    private readonly dialogRef = inject(MatDialogRef<ListDialogComponent, ListDialogResult | null>);
    readonly data = inject<ListDialogData>(MAT_DIALOG_DATA);

    // Form controls
    readonly q = new FormControl<string>('', { nonNullable: true });
    readonly group = new FormControl<string>('', { nonNullable: true });

    // Selection (single)
    readonly selectedId = signal<number | string | null>(this.data.initialSelectedId ?? null);

    readonly title = this.data.title ?? 'Select an item';
    readonly subtitle = this.data.subtitle ?? '';
    readonly searchLabel = this.data.searchLabel ?? 'Search';
    readonly filterLabel = this.data.filterLabel ?? 'Filter';
    readonly emptyText = this.data.emptyText ?? 'No items match your search.';
    readonly saveText = this.data.saveText ?? 'Save';
    readonly cancelText = this.data.cancelText ?? 'Cancel';

    readonly enableGroupFilter = !!this.data.enableGroupFilter;

    // Unique groups for dropdown
    readonly groups = computed(() => {
        if (!this.enableGroupFilter) return [];
        const set = new Set<string>();
        for (const it of this.data.items) {
            const g = (it.group ?? '').trim();
            if (g) set.add(g);
        }
        return Array.from(set).sort((a, b) => a.localeCompare(b));
    });

    // Filtered list
    readonly filteredItems = computed(() => {
        const rawQ = (this.q.value ?? '').trim().toLowerCase();
        const group = (this.group.value ?? '').trim();

        return this.data.items.filter((it) => {
            // group filter
            if (this.enableGroupFilter && group) {
                if ((it.group ?? '') !== group) return false;
            }

            if (!rawQ) return true;

            const hay = [
                it.label,
                String(it.id),
                ...(it.keywords ?? []),
                ...(it.group ? [it.group] : []),
            ]
                .join(' ')
                .toLowerCase();

            return hay.includes(rawQ);
        });
    });

    // Convenience: keep selection valid if filtering removes it (optional)
    readonly selectionStillVisible = computed(() => {
        const id = this.selectedId();
        if (id == null) return false;
        return this.filteredItems().some((x) => x.id === id);
    });

    clearSearch() {
        this.q.setValue('');
    }

    onCancel() {
        this.dialogRef.close(null);
    }

    onSave() {
        const id = this.selectedId();
        if (id == null) return;

        const selected = this.data.items.find((x) => x.id === id);
        if (!selected) return;

        this.dialogRef.close({ selectedId: id, selected });
    }

    // Optional helper: allow click on row to select
    pick(id: number | string) {
        this.selectedId.set(id);
    }
}
